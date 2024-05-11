##################
# Pre-requisites
# 	- Foundry
# 	- pnpm
#	- cargo-clippy
##################

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

# Run if setting up for first time
.PHONY: setup
setup:
	git submodule update --init --recursive
	cd tests/ccip-contracts/contracts/;\
	pnpm install
