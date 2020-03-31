# build
FROM ekidd/rust-musl-builder AS builder

ADD --chown=rust:rust . .

RUN cd api && cargo build --target x86_64-unknown-linux-musl --release --no-default-features --features "prod"

# runtime
FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/api \
    /usr/local/bin/

CMD [ "/usr/local/bin/api" ]