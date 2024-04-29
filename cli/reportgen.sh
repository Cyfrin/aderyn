#!/bin/bash

#### MARKDOWN REPORTS ######

# Basic report.md 
cargo run --  --scope src/ --exclude lib/ ./tests/contract-playground --skip-build --skip-update-check &

# Create report-config.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o report-config.md ./tests/contract-playground/ --skip-build --skip-update-check  &

# Create report.judge.md 
cargo run --  --scope src/ --exclude lib/ -o judgeops/current/report.judge.md  ./tests/contract-playground --skip-build --skip-update-check &

# Create report-config.judge.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o judgeops/current/report-config.judge.md ./tests/contract-playground --skip-build --skip-update-check &

# Adhoc sol files report.md 
cargo run --  ./tests/adhoc-sol-files -o ./tests/adhoc-sol-files-report.md --skip-build --skip-update-check &

# nft-report.md (Handle remappings)
cd tests/foundry-nft-f23 && forge build --ast && cd ../.. &&
cargo run --  ./tests/foundry-nft-f23 --scope src/ --exclude lib/ -o ./tests/nft-report.md --skip-build --skip-update-check &


##### JSON REPORTS ########

# Basic report.json
cargo run -- --scope src/ --exclude lib/ -o report.json ./tests/contract-playground --skip-build --skip-update-check &

# Create report-config.json based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o report-config.json ./tests/contract-playground/ --skip-build --skip-update-check &

wait

#### Other scripts #############

cli/sample_metricsdbgen.sh judgeops/samples/sample_db.json

