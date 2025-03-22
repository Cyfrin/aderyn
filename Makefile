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

# Debug print vars with `make print-VAR_NAME`
print-%: ; @echo $*=$($*)

