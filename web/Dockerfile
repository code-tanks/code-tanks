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

FROM python:3.10

WORKDIR /ctweb

RUN pip install fastapi "uvicorn[standard]"

COPY --from=viewer_builder /ctviewer/target/release/web /ctweb/web

COPY main.py

CMD ["uvicorn", "main:app", "--proxy-headers", "--host", "0.0.0.0", "--port", "8080"]