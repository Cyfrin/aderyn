setup:
	@echo "Installing essential libraries"
	git submodule update --init --recursive
	cd tests/ccip-contracts/contracts/;\
	pnpm install
