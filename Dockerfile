FROM rust:1.62.1 AS builder
WORKDIR /codetanks

COPY src/dummy.rs .
COPY Cargo.toml
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo install --profile release --path .
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY src src
COPY api api
RUN cargo install --profile release --path .

FROM ubuntu:focal AS runner

WORKDIR /codetanks

COPY --from=builder /usr/local/cargo/bin/codetanks /usr/local/bin/codetanks

CMD ["codetanks"]
