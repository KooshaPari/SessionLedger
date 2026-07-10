# SessionLedger developer task surface
# Aligns with AGENTS.md / .github/workflows/ci.yml where noted.

CARGO ?= cargo
DAEMON_MANIFEST := crates/sl-daemon/Cargo.toml

.PHONY: help build test lint fmt clippy package dev dev-down

help:
	@echo "Targets:"
	@echo "  build     compile sl-daemon and sl-viewer (debug)"
	@echo "  test      cargo test --all-features --locked (+ daemon)"
	@echo "  fmt       apply rustfmt (workspace + daemon)"
	@echo "  clippy    cargo clippy --all-targets --all-features"
	@echo "  lint      fmt --check + clippy (CI-equivalent gate)"
	@echo "  package   desktop packaging (packaging/Makefile)"
	@echo "  dev       build then process-compose up"
	@echo "  dev-down  process-compose down"

## build - compile sl-daemon and sl-viewer (debug profile)
build:
	$(CARGO) build --manifest-path $(DAEMON_MANIFEST)
	$(CARGO) build -p sl-viewer

## test - workspace suite (CI flags) plus excluded sl-daemon
test:
	$(CARGO) test --all-features --locked
	$(CARGO) test --manifest-path $(DAEMON_MANIFEST)

## fmt - apply rustfmt across workspace + daemon
fmt:
	$(CARGO) fmt --all
	$(CARGO) fmt --manifest-path $(DAEMON_MANIFEST)

## clippy - lint with clippy (workspace + daemon)
clippy:
	$(CARGO) clippy --all-targets --all-features --locked
	$(CARGO) clippy --manifest-path $(DAEMON_MANIFEST) --all-targets

## lint - format check + clippy (matches CI rustfmt/clippy steps)
lint:
	$(CARGO) fmt --all --check
	$(CARGO) fmt --manifest-path $(DAEMON_MANIFEST) --check
	$(CARGO) clippy --all-targets --all-features --locked
	$(CARGO) clippy --manifest-path $(DAEMON_MANIFEST) --all-targets

## package - desktop packaging via packaging/Makefile
package:
	$(MAKE) -C packaging package-all

## dev - build both crates then bring up the process-compose stack
dev: build
	process-compose up

## dev-down - tear down the process-compose stack
dev-down:
	process-compose down
