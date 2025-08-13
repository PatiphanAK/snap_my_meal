include .env
export $(shell sed 's/=.*//' .env)

DOCKER_HOST = unix:///var/run/docker.sock
export DOCKER_HOST

all: dbup run_app_api

dbup:
	@echo "🐘 Starting PostgreSQL container..."
	docker compose -f database/postgres/docker-compose.yml up -d db
	@echo "✅ PostgreSQL started with migrations auto-loaded"

dbwait:
	@echo "⏳ Waiting for database to be ready..."
	@until docker exec products_db pg_isready -U $(POSTGRES_USER) > /dev/null 2>&1; do \
		sleep 1; \
	done
	@echo "✅ Database is ready"

run_app_api:
	@echo "🚀 Starting the API server..."
	cd myapp && cargo run

dbreset:
	@echo "💥 Removing old PostgreSQL volume and stopping services..."
	docker compose -f database/postgres/docker-compose.yml down -v
	@echo "✅ Database reset complete"

clean:
	@echo "🧹 Cleaning up Docker containers and local build artifacts..."
	docker compose -f database/postgres/docker-compose.yml down
	cd myapp && cargo clean
	@echo "✅ Cleanup complete"