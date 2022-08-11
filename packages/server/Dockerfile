FROM rust:1.62.1 AS builder
WORKDIR /codetanks

COPY . .
RUN cargo install --path .

FROM ubuntu:focal AS runner

WORKDIR /codetanks

COPY --from=builder /usr/local/cargo/bin/ctserver /usr/local/bin/ctserver

CMD ["ctserver"]
