#!/bin/bash

cargo run -- ./tests/contract-playground && cargo run -- ./tests/contract-playground -o report.json && cargo run --bin bot_example -- -p orig_
