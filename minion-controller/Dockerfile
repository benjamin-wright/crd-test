FROM rust:1.39.0-slim as build

RUN apt-get update && apt-get install libssl-dev pkg-config -y
WORKDIR /var/apps/src

COPY . .
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo build --release
RUN ls