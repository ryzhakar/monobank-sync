# monobank-sync development commands
# Run `just` or `just --list` to see all available commands

# Default recipe - show available commands
default:
    @just --list

# Install development dependencies
setup:
    @echo "📦 Installing Rust toolchain components..."
    rustup component add clippy rustfmt
    @echo "📦 Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features postgres,rustls
    @echo "📦 Installing pre-commit..."
    which pre-commit > /dev/null || pip install pre-commit
    @echo "🔧 Setting up pre-commit hooks..."
    pre-commit install
    @echo "🔧 Generating SQLx offline data..."
    just prepare-offline
    @echo "✅ Development environment ready"

# Generate SQLx offline query data for PostgreSQL
prepare-offline:
    #!/usr/bin/env bash
    set -e
    echo "🔧 Generating SQLx offline data for PostgreSQL..."

    POSTGRES_CONTAINER="tmp-sqlx-postgres"

    cleanup() {
        echo "🧹 Cleaning up..."
        docker stop "$POSTGRES_CONTAINER" 2>/dev/null || true
        docker rm "$POSTGRES_CONTAINER" 2>/dev/null || true
    }
    trap cleanup EXIT

    echo "🐘 Starting temporary PostgreSQL container..."
    docker run -d --name "$POSTGRES_CONTAINER" \
        -e POSTGRES_PASSWORD=testpass \
        -e POSTGRES_DB=sqlx_test \
        -p 5433:5432 \
        postgres:15 > /dev/null

    echo "🐘 Waiting for PostgreSQL to be ready..."
    sleep 5
    until docker exec "$POSTGRES_CONTAINER" pg_isready -U postgres > /dev/null 2>&1; do
        echo "   Waiting for PostgreSQL..."
        sleep 1
    done

    echo "🐘 Running migrations..."
    DATABASE_URL="postgresql://postgres:testpass@localhost:5433/sqlx_test" \
        sqlx migrate run --source migrations/postgres

    echo "🐘 Generating offline data..."
    DATABASE_URL="postgresql://postgres:testpass@localhost:5433/sqlx_test" \
        cargo sqlx prepare --workspace

    echo "✅ SQLx offline data generated successfully!"
    echo "📁 Files updated in .sqlx/"

# Run with SQLite (with custom DATABASE_URL)
run-sqlite DATABASE_URL="sqlite://./local.db":
    @echo "🚀 Running with SQLite..."
    cargo build --features sqlite --no-default-features
    DATABASE_URL={{DATABASE_URL}} cargo run --features sqlite --no-default-features

# Create a new migration
migrate-new NAME:
    @echo "📝 Creating new migration: {{NAME}}"
    sqlx migrate add -r {{NAME}} --source migrations/postgres

# Run migrations (PostgreSQL)
migrate-run DATABASE_URL:
    @echo "⬆️  Running migrations..."
    DATABASE_URL={{DATABASE_URL}} sqlx migrate run --source migrations/postgres

# Revert last migration (PostgreSQL)
migrate-revert DATABASE_URL:
    @echo "⬇️  Reverting last migration..."
    DATABASE_URL={{DATABASE_URL}} sqlx migrate revert --source migrations/postgres

# Show migration status
migrate-info DATABASE_URL:
    @echo "ℹ️  Migration status..."
    DATABASE_URL={{DATABASE_URL}} sqlx migrate info --source migrations/postgres
