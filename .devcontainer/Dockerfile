FROM ubuntu:24.04
ARG TOOLCHAIN=1.78.0
ARG TZ=Europe/Madrid

ENV DEBIAN_FRONTEND=noninteractive
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt -y update && apt -y upgrade &&\
    apt -y install build-essential curl git tzdata rust-lldb &&\
    rm -rf /var/lib/apt/lists/*

# Use user ubuntu
RUN apt-get update \
    && apt-get install -y sudo \
    && echo ubuntu ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/ubuntu \
    && chmod 0440 /etc/sudoers.d/ubuntu

USER ubuntu

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=$TOOLCHAIN

RUN echo 'PATH="${PATH}:$HOME/.cargo/bin"' >> $HOME/.bashrc