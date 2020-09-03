FROM alpine:latest

WORKDIR app
COPY target/x86_64-unknown-linux-musl/release/rust_todo /app/

ENTRYPOINT ["/app/rust_todo"]
