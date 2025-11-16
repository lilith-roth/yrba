
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

build-deb:
    cargo deb

build-deb-target target:
    cargo deb -- --target {{ target }} --features vendored-openssl

build-nix:
    nix build

run-docker-compose:
    docker compose up

run-docker-compose-cron:
    docker compose up -f docker-compose-cron.yml

run-nix:
    nix run .

test:
    cargo test --verbose

lint:
    cargo clippy --verbose

format:
    cargo fmt

format-check:
    cargo fmt --verbose --check
