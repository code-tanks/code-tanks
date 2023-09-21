FROM rust:1.72.1 AS server_builder
WORKDIR /ctserver

COPY server/dummy.rs .
COPY server/Cargo.toml .

ARG profile=dev

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml \
    && cargo install --bin ctserver --path . --profile $profile \
    && sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY server/src src
RUN cargo install --bin ctserver --path . --profile $profile

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl

WORKDIR /ctserver

COPY --from=server_builder /usr/local/cargo/bin/ctserver /usr/local/bin/ctserver

CMD ["ctserver"]
