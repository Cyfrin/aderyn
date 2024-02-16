#!/bin/bash

#### MARKDOWN REPORTS ######

# Basic report.md 
cargo run -- ./tests/contract-playground

# Create report-config.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json -o report-config.md ./tests/contract-playground/ 

# Create report.judge.md 
cargo run -- ./tests/contract-playground -o report.judge.md

# Create report-config.judge.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json ./tests/contract-playground -o report-config.judge.md


##### JSON REPORTS ########

# Basic report.json
cargo run -- ./tests/contract-playground -o report.json 

# Create report-config.json based on config file
cargo run -- --config-file ./tests/aderyn.config.json -o report-config.json ./tests/contract-playground/ 


#### BOT ###################

# Make sure nyth users can actually interact with driver
cargo run --bin bot_example -- -p orig_ 

#### Other scripts #############

cli/bot_archivegen.sh
cli/sample_metricsdbgen.sh sample_db.json
