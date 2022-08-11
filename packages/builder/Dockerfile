FROM rust:alpine AS builder
WORKDIR /ctbuilder

COPY . .
RUN cargo install --path .

FROM docker:dind AS runner

RUN apk update \
    && apk add \
    curl

WORKDIR /ctbuilder

COPY src/Dockerfile .

COPY --from=builder /usr/local/cargo/bin/ctbuilder /usr/local/bin/ctbuilder

CMD ["ctbuilder"]