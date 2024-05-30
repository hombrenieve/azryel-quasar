FROM ubuntu:24.04



RUN apt -y update && apt -y upgrade &&\
    apt -y install build-essential curl git &&\
    rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=1.78.0
ENV PATH="${PATH}:$HOME/.cargo/bin"