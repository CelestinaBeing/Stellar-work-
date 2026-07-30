DOCKER = docker
COMPOSE = docker compose

.PHONY: help up down logs clean build dev test-contract coverage-contract test-frontend lint-frontend typecheck

help:
	@echo "StellarWork Development Commands"
	@echo "================================"
	@echo ""
	@echo "Docker Compose (Recommended):"
	@echo "  make up               Start all services (frontend + local Stellar + contract builder)"
	@echo "  make down             Stop all services (preserves volumes)"
	@echo "  make logs             View logs from all services (follow mode)"
	@echo "  make logs-stellar     View Stellar service logs"
	@echo "  make logs-frontend    View frontend service logs"
	@echo "  make clean            Remove all services and volumes (fresh start)"
	@echo ""
	@echo "Testing (with Docker):"
	@echo "  make test-contract    Run contract unit tests in Docker"
	@echo "  make coverage-contract Run contract coverage analysis"
	@echo "  make test-frontend    Run frontend tests in Docker"
	@echo ""
	@echo "Building and Development (with Docker):"
	@echo "  make build            Build frontend production bundle in Docker"
	@echo "  make lint-frontend    Run ESLint on frontend in Docker"
	@echo "  make typecheck        Run TypeScript type checking"
	@echo ""
	@echo "Development (without Docker):"
	@echo "  make dev              Start frontend dev server locally (no Docker)"
	@echo ""
	@echo "For detailed setup and troubleshooting, see: docs/DOCKER_COMPOSE_SETUP.md"

up:
	@echo "Starting all services..."
	$(COMPOSE) up

down:
	@echo "Stopping services..."
	$(COMPOSE) down

logs:
	$(COMPOSE) logs -f

logs-stellar:
	$(COMPOSE) logs -f stellar

logs-frontend:
	$(COMPOSE) logs -f frontend

build:
	$(COMPOSE) exec frontend npm run build

dev:
	cd frontend && npm run dev

test-contract:
	$(COMPOSE) exec contract-builder bash -c "cd /workspace/contracts/escrow && cargo test"

coverage-contract:
	$(COMPOSE) exec contract-builder bash -c "cd /workspace && ./contracts/coverage.sh"

test-frontend:
	$(COMPOSE) exec frontend npm test

lint-frontend:
	$(COMPOSE) exec frontend npm run lint

typecheck:
	$(COMPOSE) exec frontend npm run typecheck

clean:
	@echo "Cleaning up all containers and volumes..."
	$(COMPOSE) down -v
	cd frontend && rm -rf .next node_modules 2>/dev/null || true
