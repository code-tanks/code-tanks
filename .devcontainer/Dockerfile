FROM ubuntu:latest

ENV PATH "$PATH:/home/developer/flutter/bin"
ENV PATH "$PATH:/home/developer/.cargo/bin"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git build-essential pkg-config libssl-dev jq \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev

RUN useradd -ms /bin/bash developer
USER developer
WORKDIR /home/developer

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.63.0 -y
RUN rustup component add rust-analysis
RUN rustup target install wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli
RUN cargo install basic-http-server