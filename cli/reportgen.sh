#!/bin/bash

#### MARKDOWN REPORTS ######

# Basic report.md 
cargo run --  --scope src/ --exclude lib/ ./tests/contract-playground --skip-build &

# Create report-config.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o report-config.md ./tests/contract-playground/ --skip-build  &

# Create report.judge.md 
cargo run --  --scope src/ --exclude lib/ -o judgeops/current/report.judge.md  ./tests/contract-playground --skip-build &

# Create report-config.judge.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o judgeops/current/report-config.judge.md ./tests/contract-playground --skip-build &


##### JSON REPORTS ########

# Basic report.json
cargo run -- --scope src/ --exclude lib/ -o report.json ./tests/contract-playground --skip-build  &

# Create report-config.json based on config file
cargo run -- --config-file ./tests/aderyn.config.json --exclude lib/ -o report-config.json ./tests/contract-playground/ --skip-build  &

wait

#### Other scripts #############

cli/bot_archivegen.sh
cli/sample_metricsdbgen.sh judgeops/samples/sample_db.json

