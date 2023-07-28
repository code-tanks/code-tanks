FROM rust:1.70.0 AS builder

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev

WORKDIR /ctsim

COPY worker_simulator/dummy.rs .
COPY worker_simulator/Cargo.toml .
RUN sed -i '/ctsimlib/d' Cargo.toml
RUN sed -i '/ct-api/d' Cargo.toml
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml

ARG profile=dev

RUN cargo install --bin ctsim --path . --profile $profile

####

# COPY worker_simulator/dummy.rs .
COPY worker_simulator/Cargo.toml .
# RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN sed -i 's#../simulator#simulator#' Cargo.toml
RUN sed -i 's#../api#api#' Cargo.toml
COPY api api
COPY simulator simulator

# ARG profile=dev

# RUN cargo install --bin ctsim --path . --profile $profile
# RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY worker_simulator/src src
RUN cargo install --bin ctsim --path . --profile $profile

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git jq
    # build-essential pkg-config libssl-dev jq \
    # g++ pkg-config libx11-dev libasound2-dev libudev-dev

RUN curl -fsSL https://get.docker.com -o get-docker.sh && \
    sh get-docker.sh && rm get-docker.sh

WORKDIR /ctsim

COPY --from=builder /usr/local/cargo/bin/ctsim /usr/local/bin/ctsim

CMD ["ctsim"]