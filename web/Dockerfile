FROM rust:1.71.1 AS viewer_builder
WORKDIR /ctviewer

RUN rustup target install wasm32-unknown-unknown
RUN cargo install -f wasm-bindgen-cli --version 0.2.87

COPY worker_simulator/dummy.rs .
COPY viewer/Cargo.toml .
RUN sed -i '/ctengine/d' Cargo.toml
RUN sed -i '/ctgraphics/d' Cargo.toml
RUN sed -i '/ct-api/d' Cargo.toml
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml

ARG profile=dev

RUN cargo build --bin ctviewer --target wasm32-unknown-unknown --profile $profile

####

# COPY server/dummy.rs .
COPY viewer/Cargo.toml .

# RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN sed -i 's#../engine#engine#' Cargo.toml
RUN sed -i 's#../api#api#' Cargo.toml
RUN sed -i 's#../graphics#graphics#' Cargo.toml
COPY api api
COPY graphics graphics
COPY simulator simulator


# ARG profile=dev

# RUN cargo build --bin ctviewer --target wasm32-unknown-unknown --profile $profile
# RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY viewer/src src
RUN cargo build --bin ctviewer --target wasm32-unknown-unknown --profile $profile
# RUN echo "hi"
# RUN file="$(ls -1 target/wasm32-unknown-unknown/release/wasm32-unknown-unknown)" && echo $file

RUN if [ "$profile" = "dev" ]; then \
        export output=debug; \
    else \
        export output=release; \
    fi && \
    wasm-bindgen --out-name ctviewer \
        --out-dir target/release/web \
        --target web target/wasm32-unknown-unknown/$output/ctviewer.wasm

FROM python:3.11

WORKDIR /ctweb

RUN pip install fastapi "uvicorn[standard]" requests pandas

COPY desktop/assets/ /ctweb/assets/
COPY web/main.py .
# COPY --from=viewer_builder /ctviewer/target/release/web /ctweb/web
COPY --from=viewer_builder /ctviewer/target/release/web /ctweb/assets

CMD ["uvicorn", "main:app", "--proxy-headers", "--host", "0.0.0.0", "--port", "8080"]