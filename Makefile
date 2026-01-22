# Tools required for the setup
REQUIRED_TOOLS := forge pnpm yarn cargo-clippy

# Colors for better UX
GREEN = \033[0;32m
YELLOW = \033[0;33m
RED = \033[0;31m
NC = \033[0m # No Color

.PHONY: default
default: setup ## Run `make setup` by default

.PHONY: help
help: ## Display this help message
	@awk 'BEGIN {FS = ":.*?## "}; /^[a-zA-Z_-]+:.*?## / {printf "$(GREEN)%-30s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: check-tools
check-tools: ## Check if all required tools are installed
	@echo "$(YELLOW)Checking for required tools...$(NC)"
	@for tool in $(REQUIRED_TOOLS); do \
		if ! command -v $$tool >/dev/null 2>&1; then \
			echo "$(RED)Error: $$tool is not installed.$(NC)"; \
			case $$tool in \
				forge) \
					echo "To install Foundry (which includes 'forge'), run:"; \
					echo "  curl -L https://foundry.paradigm.xyz | bash"; \
					echo "  foundryup"; \
					;; \
				pnpm) \
					echo "To install pnpm, run:"; \
					echo "  npm install -g pnpm"; \
					;; \
				yarn) \
					echo "To install yarn, run:"; \
					echo "  npm install -g yarn"; \
					;; \
				cargo-clippy) \
					echo "To install cargo-clippy, ensure Rust is installed, then run:"; \
					echo "  rustup component add clippy"; \
					;; \
				*) \
					echo "No installation instructions available for $$tool."; \
					;; \
			esac; \
			exit 1; \
		else \
			echo "$(GREEN)✓$(NC) $$tool is installed."; \
		fi; \
	done
	@echo "$(GREEN)All required tools are installed.$(NC)"

.PHONY: setup
setup: check-tools ## Set up the project for the first time
	@echo "$(YELLOW)Setting up the project...$(NC)"

	@echo "$(YELLOW)Updating git submodules...$(NC)"
	git submodule update --init --recursive

	@echo "$(YELLOW)Installing dependencies for ccip-contracts...$(NC)"
	pnpm install --prefix tests/ccip-contracts/contracts/ --frozen-lockfile

	@echo "$(YELLOW)Installing dependencies for 2024-05-Sablier...$(NC)"
	pnpm install --prefix tests/2024-05-Sablier/v2-core

	@echo "$(YELLOW)Installing dependencies for prb-math...$(NC)"
	pnpm install --prefix tests/prb-math

	@echo "$(YELLOW)Installing dependencies for 2024-07-templegold (root)...$(NC)"
	cd tests/2024-07-templegold && yarn install --frozen-lockfile --ignore-engines && git restore package.json || true

	@echo "$(YELLOW)Installing dependencies for 2024-07-templegold (protocol)...$(NC)"
	cd tests/2024-07-templegold/protocol && yarn install --frozen-lockfile --ignore-engines

	@echo "$(YELLOW)Installing dependencies for hardhat-js-playground...$(NC)"
	cd tests/hardhat-js-playground && yarn install --frozen-lockfile

	@echo "$(YELLOW)Ensuring clean git state...$(NC)"
	cd tests/2024-07-templegold && git restore package.json || true
	git checkout -- tests/ || true

	@echo "$(GREEN)Project setup complete!$(NC)"

.PHONY: clean
clean: ## Clean all installed dependencies (node_modules)
	@echo "$(YELLOW)Cleaning installed dependencies...$(NC)"
	rm -rf tests/ccip-contracts/contracts/node_modules
	rm -rf tests/2024-05-Sablier/v2-core/node_modules
	rm -rf tests/prb-math/node_modules
	rm -rf tests/2024-07-templegold/node_modules
	rm -rf tests/2024-07-templegold/protocol/node_modules
	rm -rf tests/hardhat-js-playground/node_modules
	@echo "$(GREEN)Clean complete!$(NC)"

# Debug: Print the value of a variable with `make print-VAR_NAME`
print-%: ; @echo $*=$($*)

