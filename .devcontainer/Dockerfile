FROM ubuntu:latest

ENV PATH "$PATH:/home/developer/flutter/bin"
ENV PATH "$PATH:/home/developer/.cargo/bin"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git build-essential pkg-config libssl-dev \
    unzip xz-utils zip libglu1-mesa openjdk-8-jdk wget clang cmake ninja-build libgtk-3-dev

RUN useradd -ms /bin/bash developer
USER developer
WORKDIR /home/developer

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.62.1 -y
RUN rustup component add rust-analysis

RUN git clone -b 'stable' --single-branch --depth 1  https://github.com/flutter/flutter.git
RUN flutter doctor