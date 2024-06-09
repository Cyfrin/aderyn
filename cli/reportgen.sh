#!/bin/bash

#### MARKDOWN REPORTS ######

# Basic report.md 
cargo run --  -i src/ -x lib/ ./tests/contract-playground -o ./reports/report.md --skip-update-check &

# Adhoc sol files report.md 
cargo run --  ./tests/adhoc-sol-files -o ./reports/adhoc-sol-files-report.md --skip-update-check &

# Aderyn.toml with nested root
cargo run -- ./tests/2024-05-Sablier -o ./reports/sablier-aderyn-toml-nested-root.md --skip-update-check &

# nft-report.md (Handle remappings)
cd tests/foundry-nft-f23 && forge install && cd ../.. &&
cargo run --  ./tests/foundry-nft-f23 -i src/ -x lib/ -o ./reports/nft-report.md --skip-update-check &

# ccip-functions-report.md (Handle remappings)
cargo run -- tests/ccip-contracts/contracts --src src/v0.8/functions/ -x "tests/,test/,mocks/" -o ./reports/ccip-functions-report.md &

# Extract src, scope and exclude from foundry profile in case of foundry project
FOUNDRY_PROFILE=uniswap cargo run tests/contract-playground/ -o ./reports/uniswap_profile.md &

# PRB Math (uses new solidity features)
cargo run -- tests/prb-math -o reports/prb-math-report.md --skip-update-check &

##### JSON REPORTS ########

# Basic report.json
cargo run -- -i src/ -x lib/ -o ./reports/report.json ./tests/contract-playground --skip-update-check &

# Adhoc sol files Highs only JSON report
cargo run --  ./tests/adhoc-sol-files -o ./reports/adhoc-sol-files-highs-only-report.json --skip-update-check --highs-only &


##### SARIF REPORTS ########

# Basic report.sarif
cargo run -- ./tests/contract-playground -o ./reports/report.sarif --skip-update-check &

wait