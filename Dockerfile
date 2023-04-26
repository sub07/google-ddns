FROM alpine:latest

ARG APP_NAME

WORKDIR "/$APP_NAME"

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk add curl build-base pkgconfig libressl-dev
RUN curl https://sh.rustup.rs -sSf >> /tmp/rustup-init
RUN chmod +x /tmp/rustup-init
RUN /tmp/rustup-init -y --default-toolchain none
RUN . "$HOME/.cargo/env"
RUN /root/.cargo/bin/rustup default nightly

ENTRYPOINT ["/root/.cargo/bin/cargo", "run", "--release"]
