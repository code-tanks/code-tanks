FROM rust:1.63.0 AS builder
WORKDIR /ctbuilder

COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml \
    && cargo install --bin ctbuilder --path . --debug \
    && sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY src src
RUN cargo install --bin ctbuilder --path . --debug

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl

WORKDIR /ctbuilder

RUN curl -fsSL https://get.docker.com -o get-docker.sh
RUN sh get-docker.sh
RUN rm get-docker.sh

COPY Dockerfiles Dockerfiles

COPY --from=builder /usr/local/cargo/bin/ctbuilder /usr/local/bin/ctbuilder

CMD ["ctbuilder"]
