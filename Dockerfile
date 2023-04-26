FROM rust:latest

ARG APP_NAME
WORKDIR "/$APP_NAME"

ENTRYPOINT ["cargo", "run", "--release"]
