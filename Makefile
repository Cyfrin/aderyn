##################
# Pre-requisites
# 	- Foundry
# 	- pnpm
#	- yarn
#	- cargo-clippy
##################

# Run if setting up for first time
.PHONY: setup
setup:
	git submodule update --init --recursive
	cd tests/ccip-contracts/contracts/;\
	pnpm install
	cd tests/2024-05-Sablier/v2-core/;\
	yarn install && forge build
	cd tests/prb-math/;\
	npm install && forge build
	

# Check for tests to pass
.PHONY: test
test:
	cargo test

# Run before sending PRs
.PHONY: reportgen
reportgen:
	cargo fmt
	cargo clippy -- -D warnings 
	cli/reportgen.sh