FROM rust:latest as builder
WORKDIR /app
COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN cargo install --path .

FROM debian:bullseye-slim
ARG APP_NAME=app
RUN apt update
RUN apt install -y libssl1.1 libc6 ca-certificates
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/${APP_NAME} /usr/local/bin/app
ENTRYPOINT app
