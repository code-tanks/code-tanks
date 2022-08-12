FROM rust:1.63.0 AS builder
WORKDIR /ctserver

COPY . .
RUN cargo --bin ctserver install --path . --debug

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl

WORKDIR /ctserver

COPY --from=builder /usr/local/cargo/bin/ctserver /usr/local/bin/ctserver

CMD ["ctserver"]
