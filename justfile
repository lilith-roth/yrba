
build:
    cargo build --release

build-target target:
    cargo build --release --target {{ target }}

build-vendored:
    cargo build --release --features vendored-openssl

build-vendored-target target:
    cargo build --release --features vendored-openssl --target {{ target }}

build-dev:
    cargo build

build-dev-vendored:
    cargo build --features vendored-openssl

build-docker:
    docker build . -t dcpacky/yrba-official:latest

build-rpm:
    cargo generate-rpm

build-rpm-target target:
    cargo generate-rpm --target {{ target }}

build-nix:
    nix build --impure

run-docker-compose:
    docker compose up

run-docker-compose-cron:
    docker compose up -f docker-compose-cron.yml

run-nix:
    nix run . --impure

test:
    cargo test

format:
    cargo fmt
