#!/bin/bash

cd tests/contract-playground
forge build --ast
FOUNDRY_PROFILE=sample forge build --ast
cd ../../

#### MARKDOWN REPORTS ######

# Basic report.md 
cargo run -- ./tests/contract-playground --skip-build &

# Basic report.profile.md 
FOUNDRY_PROFILE=sample cargo run -- -o report.sample_profile.md ./tests/contract-playground --skip-build &

# Create report-config.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json -o report-config.md ./tests/contract-playground/ --skip-build  &

# Create report.judge.md 
cargo run -- ./tests/contract-playground -o judgeops/current/report.judge.md --skip-build &

# Create report-config.judge.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json ./tests/contract-playground -o judgeops/current/report-config.judge.md --skip-build &


##### JSON REPORTS ########

# Basic report.json
cargo run -- ./tests/contract-playground -o report.json --skip-build &

# Create report-config.json based on config file
cargo run -- --config-file ./tests/aderyn.config.json -o report-config.json ./tests/contract-playground/ --skip-build &

wait

#### Other scripts #############

cli/bot_archivegen.sh
cli/sample_metricsdbgen.sh judgeops/samples/sample_db.json

