use std::thread;
use std::time::Duration;
use log::{error, info, warn};
use reqwest::blocking::Client;
use reqwest::Error;

fn fetch_ip(client: &Client) -> Result<String, Error> {
    let r = client.get("https://domains.google.com/checkip").send()?.text()?;
    Ok(r)
}

fn post_ddns(client: &Client, auth: &str, host: &str, ip: &str) -> Result<String, Error> {
    client.post(format!("https://domains.google.com/nic/update?hostname={}&myip={}", host, ip))
        .header("Authorization", format!("Basic {}", auth))
        .header("Content-length", 0)
        .send()?
        .text()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(_e) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();


    let username = std::env::var("DDNS_USERNAME").expect("DDNS_USERNAME env variable is not set");
    let password = std::env::var("DDNS_PASSWORD").expect("DDNS_PASSWORD env variable is not set");
    let host = std::env::var("DDNS_HOST").expect("DDNS_HOST env variable is not set");
    let minutes_interval = match std::env::var("DDNS_MINUTES_INTERVAL") {
        Ok(i) => i.parse().unwrap_or_else(|_| {
            warn!("Invalid DDNS_MINUTES_INTERVAL env variable : {}, defaulting to 30 minutes", i);
            30
        }),
        Err(_) => {
            info!("DDNS_INTERVAL env variable is not set, defaulting to 30 minutes");
            30
        }
    };

    info!("Using interval : {} minutes", minutes_interval);

    let mut last_ip: Option<String> = None;

    loop {
        let client = reqwest::blocking::Client::builder()
            .tcp_keepalive(Duration::from_secs(10))
            .timeout(Duration::from_secs(5))
            .user_agent("GoogleDDNS/0.1.0")
            .build()?;
        if let Ok(ip) = &fetch_ip(&client) {
            let do_post_ddns = if let Some(last_ip) = last_ip.as_ref() {
                if ip.eq(last_ip) {
                    info!("Same ip, don't request: {}", ip);
                    false
                } else {
                    true
                }
            } else {
                last_ip = Some(ip.clone());
                true
            };
            if do_post_ddns {
                let encoded = base64::encode(format!("{}:{}", username, password));
                match post_ddns(&client, &encoded, &host, &ip) {
                    Ok(res) => info!("{}", res),
                    Err(e) => error!("{}", e),
                }
            }
        }
        thread::sleep(Duration::from_secs(minutes_interval * 60))
    }
}
