.PHONY: fmt check test build clean pre-commit pre-push

# Format all Rust code
fmt:
	cargo fmt --all

# Check formatting without changing files
check-fmt:
	cargo fmt --all -- --check

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
test:
	cargo test --all

# Build the project
build:
	cargo build --all

# Clean build artifacts
clean:
	cargo clean

# Pre-commit checks (format + clippy + test)
pre-commit: fmt clippy test
	@echo "✅ Pre-commit checks completed successfully!"

# Pre-push checks (format check + clippy + test)
pre-push: check-fmt clippy test
	@echo "✅ Pre-push checks completed successfully!"

# Install git hooks
install-hooks:
	@echo "Installing git hooks..."
	@chmod +x .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-push
	@echo "✅ Git hooks installed successfully!"

# Help
help:
	@echo "Available commands:"
	@echo "  fmt          - Format all Rust code"
	@echo "  check-fmt    - Check formatting without changing files"
	@echo "  clippy       - Run clippy lints"
	@echo "  test         - Run all tests"
	@echo "  build        - Build the project"
	@echo "  clean        - Clean build artifacts"
	@echo "  pre-commit   - Run pre-commit checks (fmt + clippy + test)"
	@echo "  pre-push     - Run pre-push checks (check-fmt + clippy + test)"
	@echo "  install-hooks - Install git hooks"
	@echo "  help         - Show this help message"
