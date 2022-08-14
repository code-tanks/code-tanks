FROM rust:1.63.0 AS builder
WORKDIR /ctserver

COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml \
    && cargo install --bin ctserver --path . --debug \
    && sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY src src
RUN cargo install --bin ctserver --path . --debug

FROM ubuntu:bionic AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl

WORKDIR /ctserver

COPY --from=builder /usr/local/cargo/bin/ctserver /usr/local/bin/ctserver

CMD ["ctserver"]
