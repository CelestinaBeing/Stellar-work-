DOCKER = docker
COMPOSE = docker compose

.PHONY: help up down build test-contract coverage-contract lint-frontend dev clean monitoring-up monitoring-down

COMPOSE_MONITORING = $(COMPOSE) -f docker-compose.yml -f monitoring/docker-compose.monitoring.yml

help:
	@echo "StellarWork Development Commands"
	@echo "================================"
	@echo "make up               Start all services (frontend + local Stellar)"
	@echo "make down             Stop all services"
	@echo "make build            Build frontend for production"
	@echo "make dev              Start frontend dev server (without Docker)"
	@echo "make test-contract    Run contract unit tests"
	@echo "make coverage-contract Run contract test coverage analysis"
	@echo "make test-frontend    Run frontend unit tests"
	@echo "make lint-frontend    Run ESLint on frontend"
	@echo "make typecheck        Run TypeScript type checking"
	@echo "make monitoring-up    Start Prometheus + Grafana + Alertmanager"
	@echo "make monitoring-down  Stop the monitoring stack"
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

coverage-contract:
	./contracts/coverage.sh

test-frontend:
	cd frontend && npm test

lint-frontend:
	cd frontend && npm run lint

typecheck:
	cd frontend && npm run typecheck

monitoring-up:
	$(COMPOSE_MONITORING) up -d
	@echo "Grafana: http://localhost:3001 (admin/admin) | Prometheus: http://localhost:9090"

monitoring-down:
	$(COMPOSE_MONITORING) down

clean:
	$(COMPOSE) down -v
	cd frontend && rm -rf .next node_modules
