#!/bin/bash

cargo run -- ./tests/contract-playground && cargo run -- ./tests/contract-playground -o report.json && cargo run --bin bot_example -- -p orig_

touch aderyn_pilot/archive.zip
rm aderyn_pilot/archive.zip
zip -r9 aderyn_pilot/archive.zip aderyn_pilot/bot_starter_pack -x "aderyn_pilot/bot_starter_pack/target/*" -x "aderyn_pilot/bot_starter_pack/.git/*"