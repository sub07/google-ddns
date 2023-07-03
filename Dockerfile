FROM rust:latest as builder
WORKDIR /
RUN USER=root cargo new app --bin
WORKDIR /app
COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt update
RUN apt install -y libssl1.1 libc6 ca-certificates
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/google-ddns /usr/local/bin/google-ddns
COPY log4rs.yaml /etc/google-ddns/
CMD ["google-ddns"]
