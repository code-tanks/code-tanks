FROM ubuntu:latest

ENV PATH "$PATH:/home/developer/.cargo/bin"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git build-essential pkg-config libssl-dev

RUN useradd -ms /bin/bash developer
USER developer
WORKDIR /home/developer

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.66.0 -y

WORKDIR /app

RUN git clone -b 'v0.1.0' --single-branch --depth 1 https://github.com/code-tanks/rust-api.git /app

RUN cargo build

ARG url

RUN curl http://localhost:8089/raw/$url > src/my_tank.rs

RUN cargo install

CMD ["ct-api"]