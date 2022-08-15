FROM rust:1.63.0 AS builder
WORKDIR /ctgame

COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#bin/simulator.rs#dummy.rs#' Cargo.toml \
    && cargo install --bin ctsim --path . --debug \
    && sed -i 's#dummy.rs#bin/simulator.rs#' Cargo.toml
COPY bin/simulator.rs bin/simulator.rs
COPY src src
RUN cargo install --bin ctsim --path .

FROM ubuntu:focal AS runner

WORKDIR /ctgame

COPY --from=builder /usr/local/cargo/bin/ctsim /usr/local/bin/ctsim

CMD ["ctsim"]
