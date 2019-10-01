FROM rust:stretch AS builder
WORKDIR /ws
# Clang is required for Argonautica.
RUN apt-get update -y && apt-get install -y clang
COPY Cargo.toml .
COPY src ./src
RUN cargo build
CMD cargo run
