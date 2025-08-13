#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

MIGRATIONS_DIR="/app/migrations" # The path to your SQL migration files inside the Docker container.

# Check if the migrations directory exists.
if [ ! -d "$MIGRATIONS_DIR" ]; then
    echo "❌ Migration directory not found: $MIGRATIONS_DIR"
    exit 1
fi

echo "🚀 Starting database migration..."

# Loop through all .sql files in the migrations directory, sorted by name.
for SQL_FILE in $(ls "$MIGRATIONS_DIR"/*.sql | sort); do
    echo "  -> Running migration: $SQL_FILE"

    # Execute the SQL file using psql.
    # The environment variables (PGPASSWORD, DB_HOST, etc.) are expected to be set.
    PGPASSWORD="$POSTGRES_PASSWORD" psql \
        -h "$DB_HOST" \
        -p "$DB_PORT" \
        -U "$POSTGRES_USER" \
        -d "$POSTGRES_DB" \
        -f "$SQL_FILE"

    echo "  ✅ Success: $SQL_FILE"
done

echo "🎉 All migrations completed successfully."