# build
FROM rust:1.41.1-stretch AS builder

RUN apt-get update && \
    apt-get -y install ca-certificates \
        cmake musl-tools libssl-dev openssl && \
    rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

COPY . .

ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cd api && cargo build --target x86_64-unknown-linux-musl --release --no-default-features --features "prod"

# runtime
FROM alpine:3.11.3

RUN apk --no-cache add ca-certificates

COPY --from=builder /target/x86_64-unknown-linux-musl/release/api .

CMD [ "/api" ]