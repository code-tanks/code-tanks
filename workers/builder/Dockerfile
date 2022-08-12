FROM rust:1.63.0 AS builder
WORKDIR /ctbuilder

COPY . .
RUN cargo install --bin ctbuilder --path . --debug

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl

WORKDIR /ctbuilder

RUN curl -fsSL https://get.docker.com -o get-docker.sh

COPY Dockerfiles Dockerfiles

COPY --from=builder /usr/local/cargo/bin/ctbuilder /usr/local/bin/ctbuilder

CMD ["ctbuilder"]