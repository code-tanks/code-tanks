FROM ubuntu:focal AS builder
ENV PATH "/root/.cargo/bin:$PATH"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git build-essential pkg-config libssl-dev jq \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.70.0 -y
RUN curl -fsSL https://get.docker.com -o get-docker.sh
RUN sh get-docker.sh
RUN rm get-docker.sh

WORKDIR /ctall

RUN rustup target install wasm32-unknown-unknown
RUN cargo install -f wasm-bindgen-cli --version 0.2.87

COPY api api
COPY cli cli
COPY viewer viewer
COPY simulator simulator
COPY simulator_graphics simulator_graphics
COPY worker_builder worker_builder
COPY worker_simulator worker_simulator
COPY web web
COPY desktop desktop
COPY runner runner
COPY server server
COPY Cargo.toml .

RUN cargo build --profile dev

RUN apt-get install -y postgresql

WORKDIR /ocypod

RUN git clone https://github.com/davechallis/ocypod.git && \
    cd ocypod && cargo build --release

WORKDIR /ctall

# COPY /ocypod/target/release/ocypod-server .

COPY scripts scripts


COPY ocypod.toml .

RUN apt-get install -y sudo
RUN apt install lsb-release gpg

RUN curl -fsSL https://packages.redis.io/gpg | gpg --dearmor -o /usr/share/keyrings/redis-archive-keyring.gpg

RUN echo "deb [signed-by=/usr/share/keyrings/redis-archive-keyring.gpg] https://packages.redis.io/deb $(lsb_release -cs) main" | tee /etc/apt/sources.list.d/redis.list

RUN apt-get update
RUN apt-get install -y redis

RUN sed -i 's|ocypod-redis|localhost:6379|g' ocypod.toml

RUN apt-get install -y redis-server
COPY init.sh .
RUN chmod 777 init.sh

CMD [ "./init.sh" ]


