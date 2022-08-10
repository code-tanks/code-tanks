FROM rust:1.62.1 AS builder
WORKDIR /codetanks

COPY . .
RUN cargo install --profile release --path . --bin codetanks

FROM ubuntu:focal AS runner

WORKDIR /codetanks

COPY --from=builder /usr/local/cargo/bin/codetanks /usr/local/bin/codetanks

CMD ["codetanks"]
