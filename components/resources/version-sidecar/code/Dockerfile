FROM ekidd/rust-musl-builder as build

COPY Cargo.toml Cargo.toml

RUN mkdir -p src \
    && echo "fn main() { println!(\"Hello World!\") }" > src/main.rs \
    && cargo build --release \
    && rm -rf src

RUN find . -name '*version_sidecar*' -delete

COPY src src

RUN cargo build --release

FROM scratch

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/version-sidecar /version-sidecar

ENTRYPOINT [ "/version-sidecar" ]