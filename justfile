
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
    docker build . -f docker/Dockerfile -t dcpacky/yrba-official:latest

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

test: test-unit-tests test-integration-tests

test-unit-tests:
    cargo test --bins

test-integration-tests:
    #!/usr/bin/env sh
    docker compose -f docker/docker-compose-integration-tests.yml up --build -d --wait
    if ! cargo test --test '*'; then
        docker compose -f docker/docker-compose-integration-tests.yml down
        exit 1
    fi
    docker compose -f docker/docker-compose-integration-tests.yml down

lint: lint-rust lint-nix
	
lint-rust:
	cargo clippy --verbose

lint-nix:
	nix run nixpkgs#statix -- check .

format: format-rust format-nix

format-rust:
	cargo fmt

format-nix:
	nix fmt

format-check: format-check-rust format-check-nix

format-check-rust:
	cargo fmt --verbose --check

format-check-nix:
	nix flake check

install-build-tools:
    cargo install cargo-generate-rpm cargo-deb

mdbook-dev:
	mdbook watch docs --open

