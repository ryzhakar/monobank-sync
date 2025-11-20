# Makefile for monobank-sync-rust

.PHONY: help build build-sqlite build-postgres test test-sqlite test-postgres clean prepare-offline fmt clippy docs

# Default target
help:
	@echo "Available targets:"
	@echo "  build          - Build for SQLite (default)"
	@echo "  build-sqlite   - Build for SQLite with online mode"
	@echo "  build-postgres - Build for PostgreSQL with online mode"
	@echo "  build-offline  - Build with offline mode (no database required)"
	@echo "  test           - Run tests for SQLite"
	@echo "  test-postgres  - Run tests for PostgreSQL"
	@echo "  prepare-offline- Generate SQLx offline data"
	@echo "  fmt            - Format code"
	@echo "  clippy         - Run clippy linter"
	@echo "  docs           - Generate documentation"
	@echo "  clean          - Clean build artifacts"
	@echo ""
	@echo "Environment variables:"
	@echo "  DATABASE_URL   - Database connection URL"
	@echo "  FEATURES       - Additional Cargo features"

# Build targets
build:
	cargo build --features online

build-sqlite: build

build-postgres:
	cargo build --features postgres,online

build-offline:
	cargo build --features offline

# Test targets
test:
	cargo test --features online

test-postgres:
	cargo test --features postgres,online

test-offline:
	cargo test --features offline

# Development tools
prepare-offline:
	cargo sqlx prepare --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --features online -- -D warnings

clippy-postgres:
	cargo clippy --features postgres,online -- -D warnings

docs:
	cargo doc --no-deps --open

# Clean
clean:
	cargo clean

# Docker development
docker-build:
	docker build -t monobank-sync .

docker-run:
	docker run --env-file .env monobank-sync

# CI simulation
ci-check:
	cargo fmt --all -- --check
	cargo clippy --features online -- -D warnings
	cargo clippy --features postgres,online -- -D warnings
	cargo build --features online
	cargo build --features postgres,online
	cargo build --features offline
	cargo test --features offline

# Database setup (for local development)
db-setup-sqlite:
	@echo "SQLite setup: Just ensure DATABASE_URL points to your .db file"
	@echo "Example: DATABASE_URL=sqlite:./monobank.db"

db-setup-postgres:
	@echo "PostgreSQL setup: Ensure PostgreSQL is running and DATABASE_URL is set"
	@echo "Example: DATABASE_URL=postgresql://user:password@localhost/dbname"

