FROM ekidd/rust-musl-builder as build

COPY . .

RUN cargo build --release

FROM scratch

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/minion-controller /minion-controller

ENTRYPOINT [ "/minion-controller" ]