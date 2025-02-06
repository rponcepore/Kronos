.PHONY: all begin check_deps build lint format test report clean

# No need for build step, as test does a build, and compiling twice is unnecessary.
all: begin check_deps format lint test report

# Variables
PROJECT_NAME := $(shell basename $(PWD))

begin:
	@echo "Executing make for $(PROJECT_NAME)"

check_deps:
	@cargo install cargo-audit
	@cargo audit

# This is a no-op if rustfmt already installed
format: 
	@output=$$(rustup component add rustfmt 2>&1); \
	if echo "$$output" | grep -q "is up to date"; then \
		# Silence if up to date \
		:; \
	else \
		echo "$$output"; \
	fi
	@cargo fmt

# This is a no-op if clippy is already installed
lint:
	@output=$$(rustup component add clippy 2>&1); \
	if echo "$$output" | grep -q "is up to date"; then \
		# Silence if up to date \
		:; \
	else \
		echo "$$output"; \
	fi
	@cargo clippy -- -D warnings

build:
	cargo build

test:
	@ cargo test

report:
	@echo ""
	@echo "make backend complete."
	@echo ""

clean:
	@echo "make clean not implemented"