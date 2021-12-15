# syntax=docker/dockerfile:1.2
FROM rust:1.57-alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --target x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/debug/json2rss .

FROM alpine:3.7
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/json2rss .
CMD ["./json2rss"]
