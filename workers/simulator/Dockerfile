FROM rust:1.63.0 AS builder
WORKDIR /ctsim

COPY . .
RUN cargo install --bin ctsim --path .

FROM ubuntu:focal AS runner

WORKDIR /ctsim

COPY --from=builder /usr/local/cargo/bin/ctsim /usr/local/bin/ctsim

CMD ["ctsim"]
