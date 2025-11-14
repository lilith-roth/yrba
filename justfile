
build:
    cargo build --release

build-dev:
    cargo build

build-docker:
    docker build . -t dcpacky/yrba-official:latest

build-rpm:
    cargo generate-rpm

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
