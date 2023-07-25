FROM ubuntu:focal AS builder
ENV PATH "/root/.cargo/bin:$PATH"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git build-essential pkg-config libssl-dev jq \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev
#     sudo postgresql redis-server

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.70.0 -y
RUN curl -fsSL https://get.docker.com -o get-docker.sh
# RUN sh get-docker.sh
# RUN rm get-docker.sh

WORKDIR /ocypod

RUN git clone https://github.com/davechallis/ocypod.git && \
    cd ocypod && cargo build --release

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

ARG profile=dev

RUN cargo build --profile $profile

RUN cargo install --bin ctserver --path server --profile $profile
RUN cargo install --bin ctsim --path worker_simulator --profile $profile
RUN cargo install --bin ctbuilder --path worker_builder --profile $profile

# COPY scripts scripts
# COPY ocypod.toml .

# RUN sed -i 's|ocypod-redis|localhost:6379|g' ocypod.toml

# COPY init.sh .
# RUN chmod 777 init.sh

# COPY scripts/Dockerfiles Dockerfiles
# RUN sed -i 's#COPY $url#RUN curl localhost:8089/raw/$url >#' Dockerfiles/*.Dockerfile

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git jq \
    sudo postgresql redis-server
    # build-essential pkg-config libssl-dev jq \
    # g++ pkg-config libx11-dev libasound2-dev libudev-dev \

RUN curl -fsSL https://get.docker.com -o get-docker.sh && \
    sh get-docker.sh && rm get-docker.sh

WORKDIR /ctall

COPY --from=builder /ocypod/ocypod/target/release/ocypod-server /ocypod/ocypod/target/release/ocypod-server
COPY --from=builder /root/.cargo/bin/ctsim /usr/local/bin/ctsim
COPY --from=builder /root/.cargo/bin/ctserver /usr/local/bin/ctserver
COPY --from=builder /root/.cargo/bin/ctbuilder /usr/local/bin/ctbuilder

COPY scripts scripts
COPY ocypod.toml .

RUN sed -i 's|ocypod-redis|localhost:6379|g' ocypod.toml

COPY init.sh .
RUN chmod 777 init.sh

COPY scripts/Dockerfiles Dockerfiles
RUN sed -i 's#COPY $url#RUN curl localhost:8089/raw/$url >#' Dockerfiles/*.Dockerfile

CMD [ "./init.sh" ]