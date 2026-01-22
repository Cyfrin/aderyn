# Tools required for the setup
required_tools := "forge pnpm yarn cargo-clippy"

# Colors for better UX
green := '\033[0;32m'
yellow := '\033[0;33m'
red := '\033[0;31m'
nc := '\033[0m'

# Default recipe
default: setup

# Display help
help:
    @just --list

# Check if all required tools are installed
check-tools:
    #!/usr/bin/env bash
    set -euo pipefail
    echo -e "{{yellow}}Checking for required tools...{{nc}}"
    for tool in {{required_tools}}; do
        if ! command -v $tool >/dev/null 2>&1; then
            echo -e "{{red}}Error: $tool is not installed.{{nc}}"
            case $tool in
                forge)
                    echo "To install Foundry (which includes 'forge'), run:"
                    echo "  curl -L https://foundry.paradigm.xyz | bash"
                    echo "  foundryup"
                    ;;
                pnpm)
                    echo "To install pnpm, run:"
                    echo "  npm install -g pnpm"
                    ;;
                yarn)
                    echo "To install yarn, run:"
                    echo "  npm install -g yarn"
                    ;;
                cargo-clippy)
                    echo "To install cargo-clippy, ensure Rust is installed, then run:"
                    echo "  rustup component add clippy"
                    ;;
                *)
                    echo "No installation instructions available for $tool."
                    ;;
            esac
            exit 1
        else
            echo -e "{{green}}✓{{nc}} $tool is installed."
        fi
    done
    echo -e "{{green}}All required tools are installed.{{nc}}"

# Set up the project for the first time
setup: check-tools
    #!/usr/bin/env bash
    set -euo pipefail
    echo -e "{{yellow}}Setting up the project...{{nc}}"

    echo -e "{{yellow}}Updating git submodules...{{nc}}"
    git submodule update --init --recursive

    echo -e "{{yellow}}Installing all dependencies in parallel...{{nc}}"

    pids=()

    (echo -e "{{yellow}}Installing dependencies for ccip-contracts...{{nc}}" && \
     pnpm install --prefix tests/ccip-contracts/contracts/ --frozen-lockfile && \
     echo -e "{{green}}✓ ccip-contracts done{{nc}}") &
    pids+=($!)

    (echo -e "{{yellow}}Installing dependencies for 2024-05-Sablier...{{nc}}" && \
     pnpm install --prefix tests/2024-05-Sablier/v2-core && \
     echo -e "{{green}}✓ 2024-05-Sablier done{{nc}}") &
    pids+=($!)

    (echo -e "{{yellow}}Installing dependencies for prb-math...{{nc}}" && \
     pnpm install --prefix tests/prb-math && \
     echo -e "{{green}}✓ prb-math done{{nc}}") &
    pids+=($!)

    # templegold root and protocol must run sequentially
    (echo -e "{{yellow}}Installing dependencies for 2024-07-templegold (root)...{{nc}}" && \
     cd tests/2024-07-templegold && yarn install --frozen-lockfile --ignore-engines && \
     git restore package.json && \
     echo -e "{{green}}✓ 2024-07-templegold (root) done{{nc}}" && \
     echo -e "{{yellow}}Installing dependencies for 2024-07-templegold (protocol)...{{nc}}" && \
     cd protocol && yarn install --frozen-lockfile --ignore-engines && \
     echo -e "{{green}}✓ 2024-07-templegold (protocol) done{{nc}}") &
    pids+=($!)

    (echo -e "{{yellow}}Installing dependencies for hardhat-js-playground...{{nc}}" && \
     cd tests/hardhat-js-playground && yarn install --frozen-lockfile && \
     echo -e "{{green}}✓ hardhat-js-playground done{{nc}}") &
    pids+=($!)

    # Wait for all and capture failures
    failed=0
    for pid in "${pids[@]}"; do
        wait $pid || failed=1
    done

    if [ $failed -ne 0 ]; then
        echo -e "{{red}}Some installations failed!{{nc}}"
        exit 1
    fi

    echo -e "{{yellow}}Ensuring clean git state...{{nc}}"
    cd tests/2024-07-templegold && git restore package.json || true
    git checkout -- tests/ || true

    echo -e "{{green}}Project setup complete!{{nc}}"

# Clean all installed dependencies (node_modules)
clean:
    #!/usr/bin/env bash
    set -euo pipefail
    echo -e "{{yellow}}Cleaning installed dependencies...{{nc}}"
    rm -rf tests/ccip-contracts/contracts/node_modules
    rm -rf tests/2024-05-Sablier/v2-core/node_modules
    rm -rf tests/prb-math/node_modules
    rm -rf tests/2024-07-templegold/node_modules
    rm -rf tests/2024-07-templegold/protocol/node_modules
    rm -rf tests/hardhat-js-playground/node_modules
    echo -e "{{green}}Clean complete!{{nc}}"
