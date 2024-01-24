	
.PHONY: setup, add_migration, run, up, clippy, fmt

setup:	
	cargo install cargo-watch
	cargo install sqlx-cli 

up:
	docker-compose up -d

clippy: 
	cargo clippy -- -D warnings

fmt:
	cargo fmt	

check: fmt clippy


run: check
	cargo watch -x run

add_migration:
ifndef name
	$(error name is not set: --* Usage: make add_migration name=migration_name *--)
endif
	@sqlx migrate add -r $(name)

migrate:
	@sqlx migrate run

revert:
	@sqlx migrate revert