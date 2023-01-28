FROM ubuntu:latest as builder

ENV PATH "$PATH:/root/.cargo/bin"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git build-essential pkg-config libssl-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.66.0 -y

WORKDIR /app

RUN git clone -b 'v0.1.4' --single-branch --depth 1 https://github.com/code-tanks/code-tanks.git /app

RUN cargo build --bin ctrunner

ARG url

RUN curl server:8088/raw/$url > runner/src/my_tank.rs

RUN cargo install --path runner

FROM ubuntu:latest AS runner

WORKDIR /app

RUN apt update

COPY --from=builder /root/.cargo/bin/ctrunner /usr/local/bin/ctrunner

EXPOSE 8080
CMD ["ctrunner"]
