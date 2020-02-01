FROM rust:1.39.0-slim as build

RUN apt-get update && apt-get install libssl-dev pkg-config -y
WORKDIR /var/app/src

COPY . .

RUN cargo build --release

FROM scratch

COPY --from=build /var/app/src/target/release/src /var/app/controller

ENTRYPOINT [ "/var/app/controller" ]