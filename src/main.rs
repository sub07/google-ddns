use std::fmt::{Display, Formatter};
use std::time::Duration;
use base64::Engine;
use base64::engine::general_purpose;

use log::{error, info, warn};
use reqwest::blocking::Client;

enum Error {
    FetchIp(reqwest::Error),
    PostIp(reqwest::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FetchIp(e) => { write!(f, "Ip fetching error : {e}") }
            Error::PostIp(e) => { write!(f, "Ip posting error : {e}") }
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

trait HandleError {
    fn handle_error(self) -> Self;
}

impl<T> HandleError for Result<T> {
    fn handle_error(self) -> Self {
        match &self {
            Ok(_) => {}
            Err(e) => {
                error!("{e}");
            }
        }
        self
    }
}

struct ConnectionData {
    auth: String,
    host: String,
}

struct Context {
    data: ConnectionData,
    client: Client,
}

fn fetch_ip(context: &Context) -> Result<String> {
    context.client.get("https://api.ipify.org?format=text")
        .send().map_err(Error::FetchIp)?
        .text()
        .map_err(Error::FetchIp)
}

fn post_ddns(context: &Context, ip: &str) -> Result<String> {
    context.client.post(format!("https://domains.google.com/nic/update?hostname={}&myip={}", context.data.host, ip))
        .header("Authorization", format!("Basic {}", context.data.auth))
        .header("Content-length", 0)
        .send().map_err(Error::FetchIp)?
        .text()
        .map_err(Error::PostIp)
}

fn do_loop(context: &Context) {
    let wait_interval = get_wait_interval();
    let mut last_ip = "".to_string();

    loop {
        match fetch_ip(context) {
            Ok(ip) => {
                info!("Ip is {ip}");
                info!("Last ip was {last_ip}");

                if last_ip != ip {
                    info!("Different ip, posting ip to ddns");
                    match post_ddns(context, &ip) {
                        Ok(res) => info!("{res}"),
                        Err(e) => error!("Failed to post ip : {e}"),
                    }
                    last_ip = ip;
                } else {
                    info!("Same ip, skipping ddns post")
                }
            }
            Err(e) => {
                error!("{e}");
            }
        };

        std::thread::sleep(wait_interval)
    }
}

fn setup_logger() {
    if let Err(_e) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

fn get_context() -> Context {
    let username = std::env::var("DDNS_USERNAME").expect("DDNS_USERNAME env variable is not set");
    let password = std::env::var("DDNS_PASSWORD").expect("DDNS_PASSWORD env variable is not set");

    let auth = general_purpose::STANDARD.encode(format!("{}:{}", username, password));

    let host = std::env::var("DDNS_HOST").expect("DDNS_HOST env variable is not set");


    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("GoogleDDNS/0.1.0")
        .https_only(true)
        .build()
        .expect("Could not build http client");

    Context {
        data: ConnectionData {
            auth,
            host,
        },
        client,
    }
}

fn get_wait_interval() -> Duration {
    let minutes_interval = match std::env::var("DDNS_MINUTES_INTERVAL") {
        Ok(i) => i.parse().unwrap_or_else(|_| {
            warn!("Invalid DDNS_MINUTES_INTERVAL env variable : {}, defaulting to 30 minutes", i);
            30
        }),
        Err(_) => {
            warn!("DDNS_INTERVAL env variable is not set, defaulting to 30 minutes");
            30
        }
    };

    info!("Using wait interval of {minutes_interval} minute(s)");

    Duration::from_secs(minutes_interval * 60)
}

fn main() {
    setup_logger();
    let context = get_context();
    do_loop(&context);
}
