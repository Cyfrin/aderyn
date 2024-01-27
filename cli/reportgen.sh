#!/bin/bash

cargo run -- ./tests/contract-playground && cargo run -- ./tests/contract-playground -o report.json && cargo run --bin bot_example -- -p orig_ && cargo run --bin bot_fw_assembler -- prod && cargo run -- --config-file ./tests/aderyn.config.json -o report-config.md ./tests/contract-playground/