FROM rust:1.63.0 AS builder
WORKDIR /ctbuilder

COPY . .
RUN cargo install --path .

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl

WORKDIR /ctbuilder

COPY --from=builder /usr/local/cargo/bin/ctbuilder /usr/local/bin/ctbuilder

CMD ["ctbuilder"]