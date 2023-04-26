FROM rust:latest

WORKDIR "/app"

ENTRYPOINT ["cargo", "run", "--release"]
