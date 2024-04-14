#!/bin/bash

#### MARKDOWN REPORTS ######

# Basic report.md 
cargo run --  --scope src/ --exclude lib/ ./tests/contract-playground &

# Create report-config.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o report-config.md ./tests/contract-playground/  &

# Create report.judge.md 
cargo run --  --scope src/ --exclude lib/ -o judgeops/current/report.judge.md  ./tests/contract-playground &

# Create report-config.judge.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o judgeops/current/report-config.judge.md ./tests/contract-playground &


##### JSON REPORTS ########

# Basic report.json
cargo run -- --scope src/ --exclude lib/ -o report.json ./tests/contract-playground  &

# Create report-config.json based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o report-config.json ./tests/contract-playground/  &

wait

#### Other scripts #############

cli/bot_archivegen.sh
cli/sample_metricsdbgen.sh judgeops/samples/sample_db.json

