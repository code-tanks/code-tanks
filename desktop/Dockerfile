FROM rust:1.70.0 AS builder

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev

WORKDIR /ctdesktop

COPY desktop/dummy.rs .
COPY api api
COPY simulator_graphics simulator_graphics
COPY simulator simulator
COPY desktop/Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN sed -i 's#../simulator#simulator#' Cargo.toml

ARG profile=dev

RUN cargo install --bin ctdesktop --path . --profile $profile
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY desktop/src src
RUN cargo install --bin ctdesktop --path . --profile $profile

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl jq
    # build-essential pkg-config libssl-dev jq \
    # g++ pkg-config libx11-dev libasound2-dev libudev-dev

# RUN curl -fsSL https://get.docker.com -o get-docker.sh && \
#     sh get-docker.sh && rm get-docker.sh

WORKDIR /ctdesktop

COPY --from=builder /usr/local/cargo/bin/ctdesktop /usr/local/bin/ctdesktop

ENTRYPOINT ["ctdesktop"]
CMD []