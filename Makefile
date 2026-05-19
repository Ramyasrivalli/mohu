# =============================================================================
# mohu — developer convenience targets
#
# Usage : make <target>
# Run     make help   to see all available targets with descriptions.
#
# Windows users: run all targets via WSL or Git Bash.
# =============================================================================

.PHONY: fmt fmt-check lint test bench fuzz check build release clean deny changelog ci help


# -----------------------------------------------------------------------------
# Formatting
# -----------------------------------------------------------------------------

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check


# -----------------------------------------------------------------------------
# Linting
# -----------------------------------------------------------------------------

lint:
	cargo clippy --workspace --all-targets -- -D warnings


# -----------------------------------------------------------------------------
# Testing
# -----------------------------------------------------------------------------

test:
	cargo test --workspace


# -----------------------------------------------------------------------------
# Benchmarks
# -----------------------------------------------------------------------------

bench:
	cargo build --benches --workspace


# -----------------------------------------------------------------------------
# Fuzzing  (requires nightly: rustup install nightly && cargo install cargo-fuzz)
# -----------------------------------------------------------------------------

fuzz:
	cargo fuzz build --fuzz-dir fuzz


# -----------------------------------------------------------------------------
# Fast compile check — no code generation
# -----------------------------------------------------------------------------

check:
	cargo check --workspace


# -----------------------------------------------------------------------------
# Build
# -----------------------------------------------------------------------------

build:
	cargo build --workspace

release:
	cargo build --workspace --release


# -----------------------------------------------------------------------------
# Cleanup
# -----------------------------------------------------------------------------

clean:
	cargo clean


# -----------------------------------------------------------------------------
# Dependency audit  (requires: cargo install cargo-deny)
# -----------------------------------------------------------------------------

deny:
	cargo deny check


# -----------------------------------------------------------------------------
# Changelog generation  (requires: cargo install git-cliff)
# -----------------------------------------------------------------------------

changelog:
	git cliff --output CHANGELOG.md


# -----------------------------------------------------------------------------
# CI — run fmt + lint + test in sequence (mirrors what CI enforces)
# Use this as your local dev loop before every push.
# -----------------------------------------------------------------------------

ci: fmt lint test


# -----------------------------------------------------------------------------
# help — list all available targets with one-line descriptions
# -----------------------------------------------------------------------------

help:
	@echo ""
	@echo "Usage: make <target>"
	@echo ""
	@echo "Formatting:"
	@echo "  fmt          Format all source files (cargo fmt --all)"
	@echo "  fmt-check    Check formatting without modifying files"
	@echo ""
	@echo "Linting:"
	@echo "  lint         Run clippy across the workspace; warnings are errors"
	@echo ""
	@echo "Testing:"
	@echo "  test         Run the full test suite (cargo test --workspace)"
	@echo ""
	@echo "Building:"
	@echo "  build        Debug build across the workspace"
	@echo "  release      Optimised release build across the workspace"
	@echo "  check        Fast compile check without codegen (cargo check)"
	@echo ""
	@echo "Benchmarks & Fuzzing:"
	@echo "  bench        Build all benchmark binaries (cargo build --benches)"
	@echo "  fuzz         Build fuzz targets via cargo-fuzz (requires nightly)"
	@echo ""
	@echo "Maintenance:"
	@echo "  clean        Remove all build artifacts (cargo clean)"
	@echo "  deny         Run dependency audit (requires cargo-deny)"
	@echo "  changelog    Regenerate CHANGELOG.md via git-cliff"
	@echo ""
	@echo "CI:"
	@echo "  ci           Run fmt + lint + test in sequence (local CI loop)"
	@echo ""
	@echo "  Windows: run all targets via WSL or Git Bash."
	@echo ""
