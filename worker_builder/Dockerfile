FROM rust:1.70.0 AS builder_builder

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
    curl jq

WORKDIR /ctbuilder

RUN curl -fsSL https://get.docker.com -o get-docker.sh && \
    sh get-docker.sh && rm get-docker.sh

COPY scripts/Dockerfiles Dockerfiles

RUN sed -i 's#COPY $url#RUN curl localhost:8089/raw/$url >#' Dockerfiles/*.Dockerfile

COPY --from=builder_builder /usr/local/cargo/bin/ctbuilder /usr/local/bin/ctbuilder

CMD ["ctbuilder"]