.PHONY: help install install-backend install-frontend services-up services-down services-logs services-status backend frontend dev start build build-backend build-frontend clean stop kill test object-explorer object-views vertex map-app financial-portfolio census-example

# Default target
.DEFAULT_GOAL := help

# Configuration - Ports can be overridden via environment variables
BACKEND_PORT ?= 8080
FRONTEND_PORT ?= 3000
ELASTICSEARCH_PORT ?= 9200
DGRAPH_HTTP_PORT ?= 8081
DGRAPH_GRPC_PORT ?= 9080
APP_NAME ?= census-example
ONTOLOGY_PATH ?= examples/census/config/census_ontology.yaml

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m # No Color

help: ## Show this help message
	@echo "$(BLUE)Ontology Framework - Makefile Commands$(NC)"
	@echo ""
	@echo "$(GREEN)Configuration (can be overridden via environment variables):$(NC)"
	@echo "  BACKEND_PORT=$(BACKEND_PORT)"
	@echo "  FRONTEND_PORT=$(FRONTEND_PORT)"
	@echo "  ELASTICSEARCH_PORT=$(ELASTICSEARCH_PORT)"
	@echo "  DGRAPH_HTTP_PORT=$(DGRAPH_HTTP_PORT)"
	@echo "  DGRAPH_GRPC_PORT=$(DGRAPH_GRPC_PORT)"
	@echo "  APP_NAME=$(APP_NAME)"
	@echo "  ONTOLOGY_PATH=$(ONTOLOGY_PATH)"
	@echo ""
	@echo "$(GREEN)Setup:$(NC)"
	@echo "  make install          Install all dependencies (Rust and Node.js)"
	@echo "  make install-backend   Install Rust dependencies"
	@echo "  make install-frontend  Install Node.js dependencies"
	@echo ""
	@echo "$(GREEN)Services:$(NC)"
	@echo "  make services-up      Start Docker services (Elasticsearch, Dgraph)"
	@echo "  make services-down    Stop Docker services"
	@echo "  make services-logs    Show Docker services logs"
	@echo ""
	@echo "$(GREEN)Development:$(NC)"
	@echo "  make start            Start everything (Docker services + backend + frontend)"
	@echo "  make dev              Run both backend and frontend together"
	@echo "  make backend          Run only the GraphQL backend server"
	@echo "  make frontend         Run only the frontend app"
	@echo "  make frontend APP=app-name  Run specific frontend app"
	@echo ""
	@echo "$(GREEN)Available Frontend Apps:$(NC)"
	@echo "  - object-explorer"
	@echo "  - object-views"
	@echo "  - vertex"
	@echo "  - map-app"
	@echo "  - financial-portfolio"
	@echo "  - census-example (default)"
	@echo "  - ontology-manager"
	@echo "  - foundry-rules"
	@echo "  - machinery"
	@echo "  - dynamic-scheduling"
	@echo "  - function-executor (NEW)"
	@echo "  - interface-explorer (NEW)"
	@echo "  - ontology-viewer (NEW)"
	@echo "  - model-manager (NEW)"
	@echo "  - platform (UNIFIED WORKSTATION)"
	@echo ""
	@echo "$(GREEN)Build:$(NC)"
	@echo "  make build            Build both backend and frontend"
	@echo "  make build-backend    Build Rust backend"
	@echo "  make build-frontend   Build frontend apps"
	@echo ""
	@echo "$(GREEN)Utilities:$(NC)"
	@echo "  make clean            Clean build artifacts"
	@echo "  make stop             Stop all running servers and Docker services"
	@echo "  make kill             Forcefully kill all processes (aggressive cleanup)"
	@echo "  make test             Run tests"
	@echo ""
	@echo "$(YELLOW)Examples:$(NC)"
	@echo "  make start APP_NAME=census-example FRONTEND_PORT=3000"
	@echo "  make services-up ELASTICSEARCH_PORT=9201"
	@echo ""

install: install-backend install-frontend ## Install all dependencies
	@echo "$(GREEN)✓ All dependencies installed$(NC)"

install-backend: ## Install Rust dependencies
	@echo "$(BLUE)Installing Rust dependencies...$(NC)"
	cd rust-core && cargo fetch

install-frontend: ## Install Node.js dependencies
	@echo "$(BLUE)Installing Node.js dependencies...$(NC)"
	cd ui-framework && npm install
	@echo "$(BLUE)Installing dependencies for all apps...$(NC)"
	@for app in ui-framework/apps/*/; do \
		if [ -f "$$app/package.json" ]; then \
			echo "$(YELLOW)Installing $$app...$(NC)"; \
			cd "$$app" && npm install || true; \
			cd ../../..; \
		fi; \
	done
	@echo "$(GREEN)✓ Frontend dependencies installed$(NC)"

services-up: ## Start Docker services (Elasticsearch, Dgraph)
	@echo "$(BLUE)Starting Docker services...$(NC)"
	@echo "$(YELLOW)Elasticsearch: http://localhost:$(ELASTICSEARCH_PORT)$(NC)"
	@echo "$(YELLOW)Dgraph HTTP: http://localhost:$(DGRAPH_HTTP_PORT)$(NC)"
	@echo "$(YELLOW)Dgraph gRPC: localhost:$(DGRAPH_GRPC_PORT)$(NC)"
	ELASTICSEARCH_PORT=$(ELASTICSEARCH_PORT) \
	DGRAPH_HTTP_PORT=$(DGRAPH_HTTP_PORT) \
	DGRAPH_GRPC_PORT=$(DGRAPH_GRPC_PORT) \
	docker-compose up -d
	@echo "$(GREEN)✓ Docker services started$(NC)"
	@echo "$(YELLOW)Waiting for services to be healthy...$(NC)"
	@sleep 5
	@docker-compose ps

services-down: ## Stop Docker services
	@echo "$(BLUE)Stopping Docker services...$(NC)"
	@docker-compose down
	@echo "$(GREEN)✓ Docker services stopped$(NC)"

services-logs: ## Show Docker services logs
	@docker-compose logs -f

services-status: ## Show Docker services status
	@docker-compose ps

backend: ## Run the GraphQL backend server
	@echo "$(BLUE)Starting GraphQL backend on port $(BACKEND_PORT)...$(NC)"
	@echo "$(YELLOW)Ontology: $(ONTOLOGY_PATH)$(NC)"
	@echo "$(YELLOW)Press Ctrl+C to stop$(NC)"
	cd rust-core/graphql-api && \
	PORT=$(BACKEND_PORT) \
	ONTOLOGY_PATH=../../$(ONTOLOGY_PATH) \
	cargo run --bin server

frontend: ## Run the frontend app (default: object-explorer)
	@if [ ! -d "ui-framework/apps/$(APP_NAME)" ]; then \
		echo "$(YELLOW)App '$(APP_NAME)' not found. Available apps:$(NC)"; \
		ls -1 ui-framework/apps/ | sed 's/^/  - /'; \
		exit 1; \
	fi
	@if [ ! -d "ui-framework/apps/$(APP_NAME)/node_modules" ]; then \
		echo "$(YELLOW)Installing dependencies for $(APP_NAME)...$(NC)"; \
		cd ui-framework/apps/$(APP_NAME) && npm install; \
	fi
	@echo "$(BLUE)Starting frontend app '$(APP_NAME)' on port $(FRONTEND_PORT)...$(NC)"
	@echo "$(YELLOW)Press Ctrl+C to stop$(NC)"
	cd ui-framework/apps/$(APP_NAME) && \
	PORT=$(FRONTEND_PORT) \
	VITE_GRAPHQL_URL=http://localhost:$(BACKEND_PORT) \
	npm run dev

dev: ## Run both backend and frontend together
	@echo "$(BLUE)Starting backend and frontend...$(NC)"
	@echo "$(GREEN)Backend: http://localhost:$(BACKEND_PORT)/graphql$(NC)"
	@echo "$(GREEN)Frontend: http://localhost:$(FRONTEND_PORT)$(NC)"
	@echo "$(YELLOW)Press Ctrl+C to stop both$(NC)"
	@bash -c 'trap "kill 0" EXIT; \
	(cd rust-core/graphql-api && \
	PORT=$(BACKEND_PORT) \
	ONTOLOGY_PATH=../../$(ONTOLOGY_PATH) \
	cargo run --bin server) & \
	BACKEND_PID=$$!; \
	sleep 3; \
	cd ui-framework/apps/$(APP_NAME) && \
	PORT=$(FRONTEND_PORT) \
	VITE_GRAPHQL_URL=http://localhost:$(BACKEND_PORT) \
	npm run dev & \
	FRONTEND_PID=$$!; \
	wait $$BACKEND_PID $$FRONTEND_PID'

start: services-up ## Start everything (Docker services + backend + frontend)
	@echo "$(BLUE)Starting complete stack...$(NC)"
	@echo "$(GREEN)Elasticsearch: http://localhost:$(ELASTICSEARCH_PORT)$(NC)"
	@echo "$(GREEN)Dgraph HTTP: http://localhost:$(DGRAPH_HTTP_PORT)$(NC)"
	@echo "$(GREEN)Backend GraphQL: http://localhost:$(BACKEND_PORT)/graphql$(NC)"
	@echo "$(GREEN)Frontend: http://localhost:$(FRONTEND_PORT)$(NC)"
	@echo "$(YELLOW)Press Ctrl+C to stop everything$(NC)"
	@bash -c 'trap "make stop" EXIT INT TERM; \
	cd rust-core/graphql-api && \
	PORT=$(BACKEND_PORT) \
	ONTOLOGY_PATH=../../$(ONTOLOGY_PATH) \
	cargo run --bin server & \
	BACKEND_PID=$$!; \
	sleep 5; \
	cd ../../ui-framework/apps/$(APP_NAME) && \
	PORT=$(FRONTEND_PORT) \
	VITE_GRAPHQL_URL=http://localhost:$(BACKEND_PORT) \
	npm run dev & \
	FRONTEND_PID=$$!; \
	wait $$BACKEND_PID $$FRONTEND_PID'

build: build-backend build-frontend ## Build both backend and frontend
	@echo "$(GREEN)✓ Build complete$(NC)"

build-backend: ## Build Rust backend
	@echo "$(BLUE)Building Rust backend...$(NC)"
	cd rust-core && cargo build --release
	@echo "$(GREEN)✓ Backend built$(NC)"

build-frontend: ## Build all frontend apps
	@echo "$(BLUE)Building frontend apps...$(NC)"
	cd ui-framework && npm run build
	@echo "$(GREEN)✓ Frontend apps built$(NC)"

clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(NC)"
	cd rust-core && cargo clean
	cd ui-framework && rm -rf node_modules apps/*/node_modules apps/*/dist
	@echo "$(GREEN)✓ Clean complete$(NC)"

stop: ## Stop all running servers and Docker services
	@echo "$(BLUE)Stopping all services...$(NC)"
	@pkill -f "cargo run --bin server" || true
	@pkill -f "vite" || true
	@lsof -ti:$(BACKEND_PORT) | xargs kill -9 2>/dev/null || true
	@lsof -ti:$(FRONTEND_PORT) | xargs kill -9 2>/dev/null || true
	@make services-down || true
	@echo "$(GREEN)✓ All services stopped$(NC)"

kill: ## Forcefully kill all processes (servers, Docker, ports)
	@echo "$(BLUE)Forcefully killing all services...$(NC)"
	@echo "$(YELLOW)Killing Rust/Cargo processes...$(NC)"
	@pkill -9 -f "cargo run --bin server" 2>/dev/null || true
	@pkill -9 -f "target.*server" 2>/dev/null || true
	@echo "$(YELLOW)Killing Vite/Node frontend processes...$(NC)"
	@pkill -9 -f "vite" 2>/dev/null || true
	@pkill -9 -f "node.*vite" 2>/dev/null || true
	@echo "$(YELLOW)Killing processes on configured ports...$(NC)"
	@lsof -ti:$(BACKEND_PORT) 2>/dev/null | xargs kill -9 2>/dev/null || true
	@lsof -ti:$(FRONTEND_PORT) 2>/dev/null | xargs kill -9 2>/dev/null || true
	@lsof -ti:$(ELASTICSEARCH_PORT) 2>/dev/null | xargs kill -9 2>/dev/null || true
	@lsof -ti:$(DGRAPH_HTTP_PORT) 2>/dev/null | xargs kill -9 2>/dev/null || true
	@lsof -ti:$(DGRAPH_GRPC_PORT) 2>/dev/null | xargs kill -9 2>/dev/null || true
	@echo "$(YELLOW)Forcefully stopping Docker containers...$(NC)"
	@docker-compose kill 2>/dev/null || true
	@docker-compose down -v --remove-orphans 2>/dev/null || true
	@echo "$(YELLOW)Cleaning up any remaining Docker containers...$(NC)"
	@docker ps -a --filter "name=ontology-" -q 2>/dev/null | xargs docker rm -f 2>/dev/null || true
	@echo "$(GREEN)✓ All processes forcefully killed$(NC)"

test: ## Run tests
	@echo "$(BLUE)Running Rust tests...$(NC)"
	cd rust-core && cargo test
	@echo "$(BLUE)Running frontend tests...$(NC)"
	cd ui-framework && npm test || echo "$(YELLOW)No frontend tests configured$(NC)"

# Quick start targets for specific apps
object-explorer: APP_NAME=object-explorer
object-explorer: dev ## Run object-explorer app with backend

object-views: APP_NAME=object-views FRONTEND_PORT=3001
object-views: dev ## Run object-views app with backend

vertex: APP_NAME=vertex FRONTEND_PORT=3002
vertex: dev ## Run vertex app with backend

map-app: APP_NAME=map-app FRONTEND_PORT=3003
map-app: dev ## Run map-app with backend

financial-portfolio: APP_NAME=financial-portfolio FRONTEND_PORT=3004
financial-portfolio: dev ## Run financial-portfolio app with backend

census-example: APP_NAME=census-example FRONTEND_PORT=3005
census-example: dev ## Run census-example app with backend

function-executor: APP_NAME=function-executor FRONTEND_PORT=5180
function-executor: dev ## Run function-executor app with backend

interface-explorer: APP_NAME=interface-explorer FRONTEND_PORT=5181
interface-explorer: dev ## Run interface-explorer app with backend

ontology-viewer: APP_NAME=ontology-viewer FRONTEND_PORT=5182
ontology-viewer: dev ## Run ontology-viewer app with backend

platform: APP_NAME=platform FRONTEND_PORT=5200
platform: dev ## Run unified platform app

.PHONY: help install install-backend install-frontend services-up services-down services-logs services-status backend frontend dev start build build-backend build-frontend clean stop kill test object-explorer object-views vertex map-app financial-portfolio census-example function-executor interface-explorer ontology-viewer model-manager platform
