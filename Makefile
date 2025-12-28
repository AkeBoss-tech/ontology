.PHONY: help backend frontend dev install build clean stop all

# Default target
.DEFAULT_GOAL := help

# Configuration
BACKEND_PORT ?= 8080
FRONTEND_PORT ?= 3000
APP_NAME ?= object-explorer

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m # No Color

help: ## Show this help message
	@echo "$(BLUE)Ontology Framework - Makefile Commands$(NC)"
	@echo ""
	@echo "$(GREEN)Setup:$(NC)"
	@echo "  make install          Install all dependencies (Rust and Node.js)"
	@echo "  make install-backend   Install Rust dependencies"
	@echo "  make install-frontend  Install Node.js dependencies"
	@echo ""
	@echo "$(GREEN)Development:$(NC)"
	@echo "  make dev              Run both backend and frontend (default: object-explorer)"
	@echo "  make backend          Run only the GraphQL backend server"
	@echo "  make frontend         Run only the frontend app (default: object-explorer)"
	@echo "  make frontend APP=app-name  Run specific frontend app"
	@echo ""
	@echo "$(GREEN)Available Frontend Apps:$(NC)"
	@echo "  - object-explorer (default)"
	@echo "  - object-views"
	@echo "  - vertex"
	@echo "  - map-app"
	@echo "  - financial-portfolio"
	@echo "  - census-example"
	@echo "  - ontology-manager"
	@echo "  - foundry-rules"
	@echo "  - machinery"
	@echo "  - dynamic-scheduling"
	@echo ""
	@echo "$(GREEN)Build:$(NC)"
	@echo "  make build            Build both backend and frontend"
	@echo "  make build-backend    Build Rust backend"
	@echo "  make build-frontend   Build frontend apps"
	@echo ""
	@echo "$(GREEN)Utilities:$(NC)"
	@echo "  make clean            Clean build artifacts"
	@echo "  make stop             Stop all running servers"
	@echo "  make test             Run tests"
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

backend: ## Run the GraphQL backend server
	@echo "$(BLUE)Starting GraphQL backend on port $(BACKEND_PORT)...$(NC)"
	@echo "$(YELLOW)Press Ctrl+C to stop$(NC)"
	cd rust-core && cargo run --bin server

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
	cd ui-framework/apps/$(APP_NAME) && npm run dev

dev: ## Run both backend and frontend together
	@echo "$(BLUE)Starting backend and frontend...$(NC)"
	@echo "$(GREEN)Backend: http://localhost:$(BACKEND_PORT)/graphql$(NC)"
	@echo "$(GREEN)Frontend: http://localhost:$(FRONTEND_PORT)$(NC)"
	@echo "$(YELLOW)Press Ctrl+C to stop both$(NC)"
	@bash -c 'trap "kill 0" EXIT; \
	cd rust-core && cargo run --bin server & \
	BACKEND_PID=$$!; \
	sleep 3; \
	cd ../ui-framework/apps/$(APP_NAME) && npm run dev & \
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

stop: ## Stop all running servers
	@echo "$(BLUE)Stopping servers...$(NC)"
	@pkill -f "cargo run --bin server" || true
	@pkill -f "vite" || true
	@lsof -ti:$(BACKEND_PORT) | xargs kill -9 2>/dev/null || true
	@lsof -ti:$(FRONTEND_PORT) | xargs kill -9 2>/dev/null || true
	@echo "$(GREEN)✓ Servers stopped$(NC)"

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
