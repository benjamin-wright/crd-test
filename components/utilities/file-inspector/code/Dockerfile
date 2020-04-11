FROM ekidd/rust-musl-builder:nightly-2020-04-10 as build

COPY Cargo.toml Cargo.toml

RUN mkdir -p src \
    && echo "fn main() { println!(\"Hello World!\") }" > src/main.rs \
    && cargo build --release \
    && rm -rf src

RUN find . -name '*file_inspector*' -delete

COPY src src

RUN cargo build --release

FROM scratch

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/file-inspector /file-inspector

ENTRYPOINT [ "/file-inspector" ]