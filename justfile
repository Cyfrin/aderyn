# Configuration
required_tools := "forge pnpm yarn cargo-clippy"

# Colors
green := '\033[0;32m'
yellow := '\033[0;33m'
red := '\033[0;31m'
nc := '\033[0m'

default: setup

help:
    @just --list

check-tools:
    #!/usr/bin/env bash
    set -euo pipefail
    echo -e "{{yellow}}Checking for required tools...{{nc}}"

    declare -A install_instructions=(
        [forge]="curl -L https://foundry.paradigm.xyz | bash && foundryup"
        [pnpm]="npm install -g pnpm"
        [yarn]="npm install -g yarn"
        [cargo-clippy]="rustup component add clippy"
    )

    for tool in {{required_tools}}; do
        if ! command -v "$tool" >/dev/null 2>&1; then
            echo -e "{{red}}Error: $tool is not installed.{{nc}}"
            if [[ -n "${install_instructions[$tool]:-}" ]]; then
                echo "To install, run: ${install_instructions[$tool]}"
            fi
            exit 1
        fi
        echo -e "{{green}}✓{{nc}} $tool is installed."
    done

    echo -e "{{green}}All required tools are installed.{{nc}}"

setup: check-tools
    #!/usr/bin/env bash
    set -euo pipefail

    info() { echo -e "{{yellow}}$1{{nc}}"; }
    success() { echo -e "{{green}}✓ $1{{nc}}"; }

    run_install() {
        local name="$1"; shift
        info "Installing dependencies for $name..."
        "$@"
        success "$name done"
    }

    info "Setting up the project..."
    info "Updating git submodules..."
    git submodule update --init --recursive

    info "Installing all dependencies in parallel..."
    pids=()

    (run_install "ccip-contracts" \
        pnpm install --prefix tests/ccip-contracts/contracts/ --frozen-lockfile) &
    pids+=($!)

    (run_install "2024-05-Sablier" \
        pnpm install --prefix tests/2024-05-Sablier/v2-core) &
    pids+=($!)

    (run_install "prb-math" \
        pnpm install --prefix tests/prb-math) &
    pids+=($!)

    # templegold: root and protocol must run sequentially
    (run_install "2024-07-templegold (root)" \
        yarn --cwd tests/2024-07-templegold install --frozen-lockfile --ignore-engines
     run_install "2024-07-templegold (protocol)" \
        yarn --cwd tests/2024-07-templegold/protocol install --frozen-lockfile --ignore-engines) &
    pids+=($!)

    (run_install "hardhat-js-playground" \
        yarn --cwd tests/hardhat-js-playground install --frozen-lockfile) &
    pids+=($!)

    # Wait for all background jobs
    failed=0
    for pid in "${pids[@]}"; do
        wait "$pid" || failed=1
    done

    if [[ $failed -ne 0 ]]; then
        echo -e "{{red}}Some installations failed!{{nc}}"
        exit 1
    fi

    info "Ensuring clean git state..."
    git -C tests/2024-07-templegold restore package.json || true
    git -C tests/2024-07-templegold/protocol restore package.json || true
    git -C tests/hardhat-js-playground restore package.json || true
    git checkout -- tests/ || true

    echo -e "{{green}}Project setup complete!{{nc}}"

clean:
    #!/usr/bin/env bash
    set -euo pipefail
    echo -e "{{yellow}}Cleaning installed dependencies...{{nc}}"

    dirs=(
        tests/ccip-contracts/contracts/node_modules
        tests/2024-05-Sablier/v2-core/node_modules
        tests/prb-math/node_modules
        tests/2024-07-templegold/node_modules
        tests/2024-07-templegold/protocol/node_modules
        tests/hardhat-js-playground/node_modules
    )

    rm -rf "${dirs[@]}"

    echo -e "{{green}}Clean complete!{{nc}}"
