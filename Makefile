# Environment variables for local dev
TEST_DB_URL := postgres://postgres:postgres@localhost:5432/test_db
DB_URL:= postgres://postgres:postgres@localhost:5432/app_db
MIGRATION_PATH := ./migrations

.PHONY: init setup up wait_for_db clippy fmt check run add_migration db_create migrate revert test_db_create test_migrate test

# Set up local dev environment
init: setup up wait_for_db migrate test_migrate

# Check code formatting and linting
check: fmt clippy

# Install development tools
setup:
	@echo "Setting up development tools..."
	cargo install cargo-watch
	cargo install sqlx-cli

# Start Docker services
up:
	@echo "Starting Docker services..."
	docker-compose up -d

# Wait for the database to be ready
wait_for_db:
	@echo "Waiting for the database to be ready..."
	@until sqlx database create --database-url $(DB_URL); do \
    	sleep 1; \
    	echo "Retrying..."; \
	done
	@echo "Database is ready."

# Lint the code
clippy:
	cargo clippy -- -D warnings

# Format the code
fmt:
	cargo fmt

# Run the application with auto-reload on code changes
run: check
	cargo watch -x run

# Add a new database migration
add_migration:
ifndef name
	$(error name is not set: --* Usage: make add_migration name=migration_name *--)
endif
	@sqlx migrate add -r $(name) --database-url $(DB_URL)

# Create the main database
db_create:
	@sqlx database create --database-url $(DB_URL)

# Run migrations on the main database
migrate: 
	@sqlx migrate run --database-url $(DB_URL)

# Revert the last database migration on the main database
revert:
	@sqlx migrate revert --database-url $(DB_URL)

# Create the test database
test_db_create:
	@sqlx database create --database-url $(TEST_DB_URL)

# Run migrations on the test database
test_migrate: test_db_create
	@sqlx migrate run --database-url $(TEST_DB_URL) --source $(MIGRATION_PATH)

# Run tests in single-threaded mode
test:
	@echo "Running tests..."
	cargo test -- --test-threads=1 --nocapture --color always
