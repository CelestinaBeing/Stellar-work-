DOCKER = docker
COMPOSE = docker compose

.PHONY: help up down build test-contract lint-frontend dev clean

help:
	@echo "StellarWork Development Commands"
	@echo "================================"
	@echo "make up               Start all services (frontend + local Stellar)"
	@echo "make down             Stop all services"
	@echo "make build            Build frontend for production"
	@echo "make dev              Start frontend dev server (without Docker)"
	@echo "make test-contract    Run contract unit tests"
	@echo "make test-frontend    Run frontend unit tests"
	@echo "make lint-frontend    Run ESLint on frontend"
	@echo "make typecheck        Run TypeScript type checking"
	@echo "make clean            Remove Docker volumes and cached data"

up:
	$(COMPOSE) up -d

down:
	$(COMPOSE) down

build:
	$(COMPOSE) exec frontend npm run build

dev:
	cd frontend && npm run dev

test-contract:
	cd contracts/escrow && cargo test

test-frontend:
	cd frontend && npm test

lint-frontend:
	cd frontend && npm run lint

typecheck:
	cd frontend && npm run typecheck

clean:
	$(COMPOSE) down -v
	cd frontend && rm -rf .next node_modules
