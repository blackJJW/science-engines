# science-engines Makefile

SHELL := /bin/bash
.DEFAULT_GOAL := help

# Docker compose command
DC := docker compose
SERVICE := dev

.PHONY: help
help:
	@echo "Targets:"
	@echo "  make up        - Build and start dev container"
	@echo "  make down      - Stop container (keep volumes)"
	@echo "  make cleanvol  - Stop container and remove volumes (CAUTION)"
	@echo "  make sh        - Open a shell in dev container"
	@echo "  make test      - Run cargo tests (workspace)"
	@echo "  make fmt       - Run rustfmt (check)"
	@echo "  make fmtfix    - Run rustfmt (apply)"
	@echo "  make clippy    - Run clippy (deny warnings)"
	@echo "  make check     - cargo check (workspace)"
	@echo "  make build     - cargo build (workspace)"
	@echo "  make ci        - fmt + clippy + test"

.PHONY: up
up:
	$(DC) up -d --build

.PHONY: down
down:
	$(DC) down

.PHONY: cleanvol
cleanvol:
	$(DC) down -v

.PHONY: sh
sh:
	$(DC) exec $(SERVICE) bash

# Rust commands (run inside container)
.PHONY: test
test:
	$(DC) exec $(SERVICE) bash -c "cargo test --workspace"

.PHONY: fmt
fmt:
	$(DC) exec $(SERVICE) bash -c "cargo fmt --all -- --check"

.PHONY: fmtfix
fmtfix:
	$(DC) exec $(SERVICE) bash -c "cargo fmt --all"

.PHONY: clippy
clippy:
	$(DC) exec $(SERVICE) bash -lc "cargo clippy --workspace --all-targets -- -D warnings"

.PHONY: check
check:
	$(DC) exec $(SERVICE) bash -c "cargo check --workspace"

.PHONY: build
build:
	$(DC) exec $(SERVICE) bash -c "cargo build --workspace"

.PHONY: ci
ci: fmt clippy test