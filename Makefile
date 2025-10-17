# Makefile for msk_musik project
# Provides convenient commands for development workflows

.PHONY: help check build test fmt clippy clean setup-hooks ci-check

# Default target
help:
	@echo "🎵 msk_musik Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  setup-hooks    Install Git pre-commit hooks"
	@echo ""
	@echo "Development:"
	@echo "  check          Quick check (format + clippy)"
	@echo "  build          Build the workspace"
	@echo "  test           Run all tests"
	@echo "  fmt            Format code with rustfmt"
	@echo "  clippy         Run clippy lints"
	@echo "  clean          Clean build artifacts"
	@echo ""
	@echo "CI Simulation:"
	@echo "  ci-check       Run full CI workflow locally"
	@echo ""
	@echo "💡 Use 'make setup-hooks' first to enable pre-commit validation"

# Install Git hooks
setup-hooks:
	@echo "🔧 Setting up Git pre-commit hooks..."
	@./scripts/setup-hooks.sh

# Quick checks (format + clippy)
check: fmt clippy

# Format code
fmt:
	@echo "📝 Formatting code..."
	@cargo fmt --all

# Run clippy lints
clippy:
	@echo "🔍 Running clippy lints..."
	@cargo clippy --all-targets --all-features -- -D warnings

# Build workspace
build:
	@echo "🔨 Building workspace..."
	@cargo build --all-targets

# Run tests
test:
	@echo "🧪 Running tests..."
	@cargo test --all
	@cargo test --doc

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean

# Run full CI workflow locally
ci-check:
	@echo "🚀 Running full CI workflow locally..."
	@echo ""
	@echo "📝 1. Checking code formatting..."
	@cargo fmt --all -- --check
	@echo "✅ Code formatting passed"
	@echo ""
	@echo "🔍 2. Running clippy lints..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✅ Clippy lints passed"
	@echo ""
	@echo "🔨 3. Building workspace..."
	@cargo build --verbose --all-targets
	@echo "✅ Build passed"
	@echo ""
	@echo "🧪 4. Running unit tests..."
	@cargo test --verbose --all
	@echo "✅ Unit tests passed"
	@echo ""
	@echo "📚 5. Running doc tests..."
	@cargo test --doc --verbose
	@echo "✅ Doc tests passed"
	@echo ""
	@echo "🎉 All CI checks passed! Your code is ready for GitHub."