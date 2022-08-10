FROM rust:1.62.1 AS builder
WORKDIR /ctcli

COPY . .
RUN cargo install --profile release --path . --bin ctcli

FROM ubuntu:focal AS runner

WORKDIR /ctcli

COPY --from=builder /usr/local/cargo/bin/ctcli /usr/local/bin/ctcli

CMD ["ctcli"]
