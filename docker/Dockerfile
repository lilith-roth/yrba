FROM rust:1.91-alpine3.22

ARG BUILD_DEPS="zlib-dev zlib-static musl-dev openssl-dev openssl-libs-static"

RUN apk add $BUILD_DEPS

RUN mkdir /build && mkdir /app
WORKDIR /build

COPY . ./

RUN cargo build --release --target-dir /app &&  \
    mkdir -p /app/docker && \
    cp /build/docker/cron_start.sh /app/docker/cron_start.sh &&  \
    chmod +x /app/docker/cron_start.sh && \
    rm -rf /build && \
    apk del $BUILD_DEPS

CMD ["/app/release/yrba"]
