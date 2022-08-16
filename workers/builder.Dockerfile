# FROM rust:1.63.0 AS builder_builder
FROM ubuntu:latest AS builder_builder

WORKDIR /ctbuilder

ENV PATH "$PATH:~/.cargo/bin"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.63.0 -y

COPY workers/dummy.rs .
COPY simulator simulator
COPY workers/Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN sed -i 's#../simulator#simulator#' Cargo.toml

RUN cargo install --bin ctbuilder --path . --debug
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY workers/src src
RUN cargo install --bin ctbuilder --path . --debug

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl jq

WORKDIR /ctbuilder

RUN curl -fsSL https://get.docker.com -o get-docker.sh
RUN sh get-docker.sh
RUN rm get-docker.sh

COPY workers/Dockerfiles Dockerfiles

COPY --from=builder_builder /usr/local/cargo/bin/ctbuilder /usr/local/bin/ctbuilder

CMD ["ctbuilder"]