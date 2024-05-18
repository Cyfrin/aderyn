#!/bin/bash

#### MARKDOWN REPORTS ######

# Basic report.md 
cargo run --  --scope src/ --exclude lib/ ./tests/contract-playground -o ./reports/report.md --skip-update-check --icf &

# Create report-config.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o ./reports/report-config.md ./tests/contract-playground/ --skip-update-check --icf &

# Adhoc sol files report.md 
cargo run --  ./tests/adhoc-sol-files -o ./reports/adhoc-sol-files-report.md --skip-update-check --icf &

# nft-report.md (Handle remappings)
cd tests/foundry-nft-f23 && forge install && cd ../.. &&
cargo run --  ./tests/foundry-nft-f23 --scope src/ --exclude lib/ -o ./reports/nft-report.md --skip-update-check --icf &

# ccip-functions-report.md (Handle remappings)
cargo run -- tests/ccip-contracts/contracts --src src/v0.8/functions/ --exclude "tests/,test/,mocks/" -o ./reports/ccip-functions-report.md --icf

# Extract src, scope and exclude from foundry profile in case of foundry project
FOUNDRY_PROFILE=uniswap cargo run tests/contract-playground/ -o ./reports/uniswap_profile.md --icf


##### JSON REPORTS ########

# Basic report.json
cargo run -- --scope src/ --exclude lib/ -o ./reports/report.json ./tests/contract-playground --skip-update-check --icf &

# Create report-config.json based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o ./reports/report-config.json ./tests/contract-playground/ --skip-update-check --icf &

##### SARIF REPORTS ########

# Basic report.sarif
cargo run -- ./tests/contract-playground -o ./reports/report.sarif --skip-update-check --icf &

wait