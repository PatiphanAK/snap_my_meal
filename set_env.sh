#!/bin/bash

# ================================
# APP Load ENV
# ================================
# Loads environment variables from a .env file
# Usage: source set_env.sh
# Optional: DEBUG=true source set_env.sh
# ================================

ENV_FILE=".env"

# Check if the .env file exists
if [ ! -f "$ENV_FILE" ]; then
    echo "❌ Error: $ENV_FILE not found."
    echo "➡️  Please create one based on .env.example"
    return 1 2>/dev/null || exit 1
fi

echo "📦 Loading environment from $ENV_FILE..."

while IFS='=' read -r key value || [ -n "$key" ]; do
    # Trim leading/trailing whitespace
    key=$(echo "$key" | xargs)
    value=$(echo "$value" | xargs)

    # Skip comments and empty lines
    [[ "$key" == "" || "$key" == \#* ]] && continue

    # Remove optional surrounding quotes from value
    value="${value%\"}"
    value="${value#\"}"
    value="${value%\'}"
    value="${value#\'}"

    export "$key=$value"

    # Show what was exported if DEBUG=true
    [ "$DEBUG" == "true" ] && echo "✅ Exported: $key"
done < "$ENV_FILE"

echo "✅ Environment variables loaded from $ENV_FILE."