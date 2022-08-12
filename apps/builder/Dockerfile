FROM rust:1.63.0-alpine AS builder
WORKDIR /ctbuilder

COPY . .
RUN cargo install --profile dev --path .

FROM docker:dind AS runner

RUN apk update \
    && apk add \
    curl

WORKDIR /ctbuilder

COPY Dockerfiles/* .

COPY --from=builder /usr/local/cargo/bin/ctbuilder /usr/local/bin/ctbuilder

CMD ["ctbuilder"]