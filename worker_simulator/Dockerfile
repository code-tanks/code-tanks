FROM ubuntu:focal AS builder
ENV PATH "/root/.cargo/bin:$PATH"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl g++ pkg-config libx11-dev libasound2-dev libudev-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.66.0 -y

WORKDIR /ctsim

COPY worker_simulator/dummy.rs .
COPY api api
COPY simulator simulator
COPY worker_simulator/Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN sed -i 's#../simulator#simulator#' Cargo.toml
RUN cargo install --bin ctsim --path . --profile dev
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY worker_simulator/src src
RUN cargo install --bin ctsim --path . --profile dev

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git build-essential pkg-config libssl-dev jq \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev

RUN curl -fsSL https://get.docker.com -o get-docker.sh
RUN sh get-docker.sh
RUN rm get-docker.sh

WORKDIR /ctsim

COPY --from=builder /root/.cargo/bin/cttest /usr/local/bin/cttest
COPY --from=builder /root/.cargo/bin/ctsim /usr/local/bin/ctsim

CMD ["ctsim"]