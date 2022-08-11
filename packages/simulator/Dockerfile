FROM rust:1.62.1 AS builder
WORKDIR /ctsim

COPY . .
RUN cargo install --path .

FROM ubuntu:focal AS runner

WORKDIR /ctsim

COPY --from=builder /usr/local/cargo/bin/ctsim /usr/local/bin/ctsim

CMD ["ctsim"]
