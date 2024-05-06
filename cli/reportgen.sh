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
cd tests/foundry-nft-f23 && forge install && cd ../.. &&
cargo run --  ./tests/foundry-nft-f23 --scope src/ --exclude lib/ -o ./tests/nft-report.md --skip-update-check &

# ccip-functions-report.md (Handle remappings)
cargo run -- tests/ccip-contracts/contracts --scope src/v0.8/functions/ --exclude "tests/,test/,testhelpers/,lib/,node_modules/,mocks/,vendor/" -o tests/ccip-functions-report.md

##### JSON REPORTS ########

# Basic report.json
cargo run -- --scope src/ --exclude lib/ -o report.json ./tests/contract-playground --skip-build --skip-update-check &

# Create report-config.json based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o report-config.json ./tests/contract-playground/ --skip-build --skip-update-check &

wait

#### Other scripts #############

cli/sample_metricsdbgen.sh judgeops/samples/sample_db.json

