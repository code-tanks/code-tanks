FROM rust:1.63.0 AS server_builder
WORKDIR /ctserver

COPY server/dummy.rs .
COPY server/Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml \
    && cargo install --bin ctserver --path . --debug \
    && sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY server/src src
RUN cargo install --bin ctserver --path . --debug

FROM rust:1.63.0 AS viewer_builder
WORKDIR /ctviewer

RUN rustup target install wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli

COPY server/dummy.rs .
COPY simulator simulator
COPY viewer/Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN sed -i 's#../simulator#simulator#' Cargo.toml
RUN cargo build --bin ctviewer --target wasm32-unknown-unknown --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY viewer/src src
RUN cargo build --bin ctviewer --target wasm32-unknown-unknown --release
RUN wasm-bindgen --out-name ctviewer \
    --out-dir target/release/web \
    --target web target/wasm32-unknown-unknown/release/ctviewer.wasm

FROM ubuntu:focal AS runner

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl

WORKDIR /ctserver

COPY --from=server_builder /usr/local/cargo/bin/ctserver /usr/local/bin/ctserver
COPY --from=viewer_builder /ctviewer/target/release/web /ctserver/web

COPY viewer/web/* /ctserver/web

CMD ["ctserver"]
