FROM rust:1.71.1 AS ocypod_builder

RUN git clone https://github.com/davechallis/ocypod.git

ARG profile=dev

RUN cd ocypod && cargo install --path . --profile $profile

FROM rust:1.71.1 AS ctsim_builder

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    g++ pkg-config libx11-dev libasound2-dev libudev-dev

WORKDIR /ctsim

COPY worker_simulator/dummy.rs .
COPY api api
COPY simulator simulator
COPY worker_simulator/Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN sed -i 's#../engine#engine#' Cargo.toml
RUN sed -i 's#../api#api#' Cargo.toml

ARG profile=dev

RUN cargo install --bin ctsim --path . --profile $profile
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY worker_simulator/src src
RUN cargo install --bin ctsim --path . --profile $profile

FROM rust:1.71.1 AS ctserver_builder
WORKDIR /ctserver

COPY server/dummy.rs .
COPY server/Cargo.toml .

ARG profile=dev

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml \
    && cargo install --bin ctserver --path . --profile $profile \
    && sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY server/src src
RUN cargo install --bin ctserver --path . --profile $profile

FROM rust:1.71.1 AS ctbuilder_builder

COPY worker_builder/dummy.rs .
COPY worker_builder/Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml

ARG profile=dev

RUN cargo install --bin ctbuilder --path . --profile $profile
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY worker_builder/src src
RUN cargo install --bin ctbuilder --path . --profile $profile

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git jq \
    sudo postgresql redis-server

RUN curl -fsSL https://get.docker.com -o get-docker.sh && \
    sh get-docker.sh && rm get-docker.sh

WORKDIR /ctall

COPY --from=ocypod_builder /usr/local/cargo/bin/ocypod-server /usr/local/bin/ocypod-server
COPY --from=ctsim_builder /usr/local/cargo/bin/ctsim /usr/local/bin/ctsim
COPY --from=ctserver_builder /usr/local/cargo/bin/ctserver /usr/local/bin/ctserver
COPY --from=ctbuilder_builder /usr/local/cargo/bin/ctbuilder /usr/local/bin/ctbuilder

COPY scripts scripts
COPY ocypod.toml .

RUN sed -i 's|ocypod-redis|localhost:6379|g' ocypod.toml

COPY init.sh .
RUN chmod 777 init.sh

COPY scripts/Dockerfiles Dockerfiles
RUN sed -i 's#COPY $url#RUN curl -sS localhost:8089/raw/$url >#' Dockerfiles/*/**Dockerfile

CMD [ "./init.sh" ]