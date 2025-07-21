FROM rust:1.88-alpine3.22

ARG BUILD_DEPS="zlib-dev zlib-static musl-dev openssl-dev openssl-libs-static"

RUN apk add $BUILD_DEPS

RUN mkdir /build && mkdir /app
WORKDIR /build

COPY . ./

RUN cargo build --release --target-dir /app &&  \
    cp /build/cron_start.sh /app/ &&  \
    chmod +x /app/cron_start.sh && \
    rm -rf /build
RUN apk del $BUILD_DEPS

RUN adduser -H -D backup
USER backup

ENTRYPOINT ["/app/release/yrba", "-c", "/app/config.toml"]
