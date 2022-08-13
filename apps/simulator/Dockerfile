FROM rust:1.63.0 AS builder
WORKDIR /ctsim

COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo install --bin ctsim --path .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
COPY src src
RUN cargo install --bin ctsim --path .

FROM ubuntu:focal AS runner

WORKDIR /ctsim

COPY --from=builder /usr/local/cargo/bin/ctsim /usr/local/bin/ctsim

CMD ["ctsim"]
