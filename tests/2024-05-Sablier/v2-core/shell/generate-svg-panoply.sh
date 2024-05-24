#!/usr/bin/env bash

# Notes:
# - Generates a panoply of SVGs with different accent colors and card contents.

# Pre-requisites:
# - foundry (https://getfoundry.sh)

# Strict mode: https://gist.github.com/vncsna/64825d5609c146e80de8b1fd623011ca
set -euo pipefail

./shell/generate-svg.sh 0 "Pending" "100" 5
./shell/generate-svg.sh 0 "Pending" "100" 21
./shell/generate-svg.sh 0 "Pending" "100" 565

./shell/generate-svg.sh 0 "Canceled" "100" 3
./shell/generate-svg.sh 0 "Canceled" "100" 3
./shell/generate-svg.sh 144 "Canceled" "29.81K" 24
./shell/generate-svg.sh 7231 "Canceled" "421.11K" 24

./shell/generate-svg.sh 15 "Streaming" "86.1K" 0
./shell/generate-svg.sh 42 "Streaming" "581" 0
./shell/generate-svg.sh 79 "Streaming" "66.01K" 0
./shell/generate-svg.sh 399 "Streaming" "314K" 0
./shell/generate-svg.sh 800 "Streaming" "50.04K" 0
./shell/generate-svg.sh 1030 "Streaming" "48.93M" 1021
./shell/generate-svg.sh 4235 "Streaming" "8.91M" 1
./shell/generate-svg.sh 5000 "Streaming" "1.5K" 1
./shell/generate-svg.sh 7291 "Streaming" "756.12T" 7211
./shell/generate-svg.sh 9999 "Streaming" "3.32K" 88
./shell/generate-svg.sh 4999 "Streaming" "999.45K" 10000

./shell/generate-svg.sh 10000 "Settled" "1" 892
./shell/generate-svg.sh 10000 "Settled" "14.94K" 11
./shell/generate-svg.sh 10000 "Settled" "733" 3402
./shell/generate-svg.sh 10000 "Settled" "645.01M" 3402
./shell/generate-svg.sh 10000 "Settled" "990.12B" 6503

./shell/generate-svg.sh 10000 "Depleted" "1" 892
./shell/generate-svg.sh 10000 "Depleted" "79.1B" 892
./shell/generate-svg.sh 4972 "Depleted" "29" 3402
./shell/generate-svg.sh 744 "Depleted" "343.01K" 3402
./shell/generate-svg.sh 10000 "Depleted" "84.1M" 6503
