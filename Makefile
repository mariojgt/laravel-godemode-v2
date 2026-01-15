# Laravel God Mode - Makefile
# Cross-platform desktop app for managing Laravel & Node.js Docker environments

.PHONY: help install dev build build-debug clean setup icons release check lint format test

# Colors
GREEN  := \033[32m
CYAN   := \033[36m
YELLOW := \033[33m
RED    := \033[31m
RESET  := \033[0m

.DEFAULT_GOAL := help

## ═══════════════════════════════════════════════════════════════════
## 🚀 Laravel God Mode - Development Commands
## ═══════════════════════════════════════════════════════════════════

help: ## Show this help message
	@echo "$(CYAN)🚀 Laravel God Mode - Development Commands$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""

## ═══════════════════════════════════════════════════════════════════
## 📦 Setup & Installation
## ═══════════════════════════════════════════════════════════════════

install: ## Install all dependencies (npm + cargo)
	@echo "$(CYAN)📦 Installing npm dependencies...$(RESET)"
	npm install
	@echo "$(CYAN)📦 Installing Rust dependencies...$(RESET)"
	cd src-tauri && cargo fetch
	@echo "$(GREEN)✅ Dependencies installed!$(RESET)"

setup: install ## Full project setup (install + create directories)
	@echo "$(CYAN)📁 Setting up project directories...$(RESET)"
	@mkdir -p ~/.laravel-godmode/templates
	@mkdir -p ~/Documents/laravel-godmode/projects
	@cp -r templates/* ~/.laravel-godmode/templates/ 2>/dev/null || true
	@echo "$(GREEN)✅ Setup complete!$(RESET)"

## ═══════════════════════════════════════════════════════════════════
## 🛠️  Development
## ═══════════════════════════════════════════════════════════════════

dev: ## Start development server (frontend + Tauri)
	@echo "$(CYAN)🛠️  Starting development server...$(RESET)"
	npm run tauri dev

dev-frontend: ## Start only the frontend dev server
	@echo "$(CYAN)🌐 Starting frontend dev server...$(RESET)"
	npm run dev

dev-open: ## Start dev server and open in browser
	@echo "$(CYAN)🌐 Starting frontend and opening browser...$(RESET)"
	npm run dev -- --open

## ═══════════════════════════════════════════════════════════════════
## 🏗️  Build
## ═══════════════════════════════════════════════════════════════════

build: ## Build for production (optimized release)
	@echo "$(CYAN)🏗️  Building for production...$(RESET)"
	npm run tauri build
	@echo "$(GREEN)✅ Build complete! Check src-tauri/target/release/bundle/$(RESET)"

build-debug: ## Build debug version (faster, larger)
	@echo "$(CYAN)🏗️  Building debug version...$(RESET)"
	npm run tauri build -- --debug
	@echo "$(GREEN)✅ Debug build complete!$(RESET)"

build-frontend: ## Build only the frontend
	@echo "$(CYAN)🌐 Building frontend...$(RESET)"
	npm run build

build-universal: ## Build universal macOS binary (Intel + Apple Silicon)
	@echo "$(CYAN)🍎 Building universal macOS binary...$(RESET)"
	npm run tauri build -- --target universal-apple-darwin
	@echo "$(GREEN)✅ Universal build complete!$(RESET)"

## ═══════════════════════════════════════════════════════════════════
## 🧹 Maintenance
## ═══════════════════════════════════════════════════════════════════

clean: ## Clean all build artifacts
	@echo "$(YELLOW)🧹 Cleaning build artifacts...$(RESET)"
	rm -rf node_modules
	rm -rf dist
	rm -rf src-tauri/target
	@echo "$(GREEN)✅ Cleaned!$(RESET)"

clean-rust: ## Clean only Rust build artifacts
	@echo "$(YELLOW)🧹 Cleaning Rust artifacts...$(RESET)"
	cd src-tauri && cargo clean
	@echo "$(GREEN)✅ Rust artifacts cleaned!$(RESET)"

clean-npm: ## Clean only npm artifacts
	@echo "$(YELLOW)🧹 Cleaning npm artifacts...$(RESET)"
	rm -rf node_modules
	rm -rf dist
	rm -f package-lock.json
	@echo "$(GREEN)✅ npm artifacts cleaned!$(RESET)"

## ═══════════════════════════════════════════════════════════════════
## ✅ Code Quality
## ═══════════════════════════════════════════════════════════════════

check: ## Check Rust code without building
	@echo "$(CYAN)🔍 Checking Rust code...$(RESET)"
	cd src-tauri && cargo check

lint: ## Run linters (TypeScript + Rust)
	@echo "$(CYAN)🔍 Linting TypeScript...$(RESET)"
	npx vue-tsc --noEmit || true
	@echo "$(CYAN)🔍 Linting Rust...$(RESET)"
	cd src-tauri && cargo clippy

format: ## Format all code (Prettier + Rustfmt)
	@echo "$(CYAN)✨ Formatting TypeScript/Vue...$(RESET)"
	npx prettier --write "src/**/*.{ts,vue,css}"
	@echo "$(CYAN)✨ Formatting Rust...$(RESET)"
	cd src-tauri && cargo fmt

## ═══════════════════════════════════════════════════════════════════
## 📋 Utilities
## ═══════════════════════════════════════════════════════════════════

icons: ## Generate app icons from source (requires icon.png in root)
	@echo "$(CYAN)🎨 Generating icons...$(RESET)"
	@if [ -f "icon.png" ]; then \
		npm run tauri icon icon.png; \
		echo "$(GREEN)✅ Icons generated!$(RESET)"; \
	else \
		echo "$(RED)❌ No icon.png found in root directory$(RESET)"; \
	fi

update: ## Update all dependencies
	@echo "$(CYAN)⬆️  Updating npm dependencies...$(RESET)"
	npm update
	@echo "$(CYAN)⬆️  Updating Rust dependencies...$(RESET)"
	cd src-tauri && cargo update
	@echo "$(GREEN)✅ Dependencies updated!$(RESET)"

info: ## Show project info and versions
	@echo "$(CYAN)📋 Project Info$(RESET)"
	@echo "  Node.js: $$(node --version)"
	@echo "  npm:     $$(npm --version)"
	@echo "  Rust:    $$(rustc --version)"
	@echo "  Cargo:   $$(cargo --version)"
	@echo "  Docker:  $$(docker --version 2>/dev/null || echo 'Not installed')"
	@echo ""
	@echo "$(CYAN)📁 Paths$(RESET)"
	@echo "  Templates: ~/.laravel-godmode/templates"
	@echo "  Projects:  ~/Documents/laravel-godmode/projects"

release: build ## Create a release build with version bump
	@echo "$(GREEN)🎉 Release build ready!$(RESET)"
	@echo "$(CYAN)📦 Installers available at:$(RESET)"
	@ls -la src-tauri/target/release/bundle/*/ 2>/dev/null || echo "  (build in progress)"

## ═══════════════════════════════════════════════════════════════════
## 🐳 Docker (for testing)
## ═══════════════════════════════════════════════════════════════════

docker-check: ## Check if Docker is running
	@echo "$(CYAN)🐳 Checking Docker...$(RESET)"
	@docker info > /dev/null 2>&1 && echo "$(GREEN)✅ Docker is running$(RESET)" || echo "$(RED)❌ Docker is not running$(RESET)"

docker-prune: ## Clean up Docker resources
	@echo "$(YELLOW)🧹 Pruning Docker resources...$(RESET)"
	docker system prune -f
	@echo "$(GREEN)✅ Docker cleaned!$(RESET)"
