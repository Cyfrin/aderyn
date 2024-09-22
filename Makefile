##################
# Pre-requisites
# 	- Foundry
# 	- pnpm
#	- yarn
#	- cargo-clippy
##################

.PHONY: help
help:
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: setup
setup: ## Run if setting up for first time
	git submodule update --init --recursive
	cd tests/ccip-contracts/contracts/;\
	pnpm install
	cd tests/2024-05-Sablier/v2-core/;\
	yarn install && forge build
	cd tests/prb-math/;\
	npm install && forge build
	cd tests/2024-07-templegold/;\
	yarn


.PHONY: pr
pr: ## Run before sending PRs
	cargo +nightly fmt --all
	cargo test --quiet
	cargo clippy --quiet --workspace --all-targets --all-features
	cli/reportgen.sh


.PHONY: build
build: ## Build the compiler
	cargo build --release


.PHONY: test
test: ## Run the compiler unit tests
	cargo test
	cargo clippy --quiet --workspace --all-targets --all-features

.PHONY: fmt
fmt: ## Run the rust formatter
	cargo +nightly fmt --all

.PHONY: test-watch
test-watch: ## Run compiler tests when files change
	watchexec -e rs,toml "cargo test --quiet"


# Debug print vars with `make print-VAR_NAME`
print-%: ; @echo $*=$($*)

