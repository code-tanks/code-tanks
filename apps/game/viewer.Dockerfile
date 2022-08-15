FROM rust:1.63.0 AS builder
WORKDIR /ctgame

RUN rustup target install wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli

COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#bin/viewer.rs#dummy.rs#' Cargo.toml \
    && cargo build --bin ctviewer --target wasm32-unknown-unknown \
    && sed -i 's#dummy.rs#bin/viewer.rs#' Cargo.toml
COPY bin/viewer.rs bin/viewer.rs
COPY src src
RUN cargo build --bin ctviewer --target wasm32-unknown-unknown
RUN wasm-bindgen --out-name ctviewer \
    --out-dir target/debug/web \
    --target web target/wasm32-unknown-unknown/debug/ctviewer.wasm

FROM nginx
COPY web /usr/share/nginx/html

COPY --from=builder /ctgame/target/debug/web /usr/share/nginx/html/build