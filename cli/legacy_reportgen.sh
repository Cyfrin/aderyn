#!/bin/bash

cd ./tests/contract-playground/
forge build --ast --force
cd ../../

#### MARKDOWN REPORTS ######

# Basic report.md 
cargo run -- -o ./reports_legacy/report.md ./tests/contract-playground/ --skip-update-check --skip-build &

# Extract scope and exclude from foundry profile in case of foundry project
FOUNDRY_PROFILE=uniswap cargo run tests/contract-playground/ -o ./reports_legacy/uniswap_profile.md --skip-update-check --skip-build &

wait


##### JSON REPORTS ########

# Basic report.json
cargo run -- -o ./reports_legacy/report.json ./tests/contract-playground/ --skip-update-check --skip-build &

wait 
##### SARIF REPORTS ########

# Basic report.sarif
cargo run -- ./tests/contract-playground -o ./reports_legacy/report.sarif --skip-update-check --skip-build &

wait