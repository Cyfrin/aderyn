#!/bin/bash

#### MARKDOWN REPORTS ######

# Basic report.md 
cargo run -- -o ./reports_legacy/report.md ./tests/contract-playground/ --skip-update-check &

# Create report-config.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json -o ./reports_legacy/report-config.md ./tests/contract-playground/ --skip-update-check &

# Extract scope and exclude from foundry profile in case of foundry project
FOUNDRY_PROFILE=uniswap cargo run tests/contract-playground/ -o ./reports_legacy/uniswap_profile.md &


##### JSON REPORTS ########

# Basic report.json
cargo run -- -o ./reports_legacy/report.json ./tests/contract-playground/ --skip-update-check &

# Create report-config.json based on config file
cargo run -- --config-file ./tests/aderyn.config.json -o ./reports_legacy/report-config.json ./tests/contract-playground/ --skip-update-check &

##### SARIF REPORTS ########

# Basic report.sarif
cargo run -- ./tests/contract-playground -o ./reports_legacy/report.sarif --skip-update-check &

wait