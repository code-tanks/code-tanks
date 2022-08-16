FROM ubuntu:latest AS builder

WORKDIR /ctsim

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl g++ pkg-config libx11-dev libasound2-dev libudev-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.63.0 -y

COPY worker_simulator/dummy.rs .
COPY simulator simulator
COPY worker_simulator/Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN sed -i 's#../simulator#simulator#' Cargo.toml

ENV PATH "$PATH:~/.cargo/bin"

RUN cargo install --bin ctsim --path . --debug
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY worker_simulator/src src
RUN cargo install --bin ctsim --path . --debug

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl jq


COPY --from=builder /usr/local/cargo/bin/ctsim /usr/local/bin/ctsim

CMD ["ctsim"]