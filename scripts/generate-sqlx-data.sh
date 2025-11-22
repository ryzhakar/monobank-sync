#!/bin/bash
set -e

echo "🔧 Generating SQLx offline data for PostgreSQL..."

POSTGRES_CONTAINER="tmp-sqlx-postgres"

# Cleanup function
cleanup() {
    echo "🧹 Cleaning up..."
    docker stop "$POSTGRES_CONTAINER" 2>/dev/null || true
    docker rm "$POSTGRES_CONTAINER" 2>/dev/null || true
}

# Set trap to cleanup on exit
trap cleanup EXIT

# Start PostgreSQL container
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

# Cleanup will happen automatically via trap
