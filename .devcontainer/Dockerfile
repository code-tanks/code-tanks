FROM ubuntu:latest

ENV PATH "$PATH:/home/developer/.cargo/bin"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git build-essential pkg-config libssl-dev jq \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev

RUN curl -fsSL https://get.docker.com -o get-docker.sh
RUN sh get-docker.sh
RUN rm get-docker.sh

RUN useradd -ms /bin/bash developer
USER developer
WORKDIR /home/developer

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.70.0 -y
RUN rustup component add rust-analysis
RUN rustup target install wasm32-unknown-unknown

VOLUME [ "/var/lib/docker" ]