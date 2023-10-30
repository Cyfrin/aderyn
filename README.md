# 🦜 Aderyn: Solidity AST Analyzer

## What is Aderyn?

Aderyn is a Solidity Static Analyzer. It takes a bird's eye view over your smart contracts, traversing the Abstract Syntax Trees (AST) to pinpoint suspected vulnerabilities. Aderyn prints out these potential issues in an easy-to-consume markdown format.

## When/Why to Use it?

Use Aderyn when developing or auditing Solidity smart contracts to quickly identify areas where the code may not be following best practices or has potential vulnerabilities.

## Features

* [Hardhat](https://hardhat.org/) and [Foundry](https://book.getfoundry.sh/) support
* Modular [detectors](./src/detect/)
* AST Traversal
* Markdown reports

# Usage

## Prerequisites

[Install Rust](https://www.rust-lang.org/tools/install)

## Installation

In the command line, run:
```sh
cargo install aderyn
```

# Usage Instructions

## Quick Start

The project you're running Aderyn on should be either a Foundry or compiled Hardhat project.

```sh
aderyn --root /path/to/your/foundry/project/root/directory/
```

That's it! Aderyn identifies whether the project root is a Foundry or Hardhat repo, then uses the compiled AST files to hunt for vulnerabilities. `report.md` will be output in the directory in which you ran the command.

## Params

1. You must provide the root directory of the repo you want to analyze:
`--root /path/to/repo/root/`

## Supported Development Frameworks

Aderyn automatically detects the development framework so long as it's Foundry or Hardhat. 

### Foundry

If Foundry is detected in the project root, Aderyn will first run `forge build` to ensure that the contract compiles correctly and the latest artifacts are available.

### Hardhat

If Hardhat is detected, Aderyn does not auto-compile. Make sure to run `hardhat compile` BEFORE running Aderyn. 

# Roadmap

## Medium-term goals - Auditor Aid:
* [x] Support Multiple Abstract Syntax Trees representing multiple Solidity files
* [x] Support Foundry/Hardhat/Truffle/Solc output formats for ingesting AST
  * [x] Foundry
  * [x] Hardhat
* Complexity score (with Percentage YUL code & nsloc)
* More complex static analysis detectors
* auto-fixes
* installer that doesn't require Rust (aderynup)
* ...

## Long-term goals - Product:
Create tools that utilize the context library to:
* Provide automated gas optimizations
* Custom subscribable detectors
* Control/data flow analyses
* Symbolic execution
* Invariant handler generation
* Vyper support

# Contributing & License

Help us build Aderyn 🦜 Please see our [contribution guidelines](./CONTRIBUTING.md).
This repo is published under the [MIT License](./LICENSE).

# Attribution
* AST Visitor code from [solc-ast-rs](https://github.com/hrkrshnn/solc-ast-rs).
* Original detectors based on [4nalyzer](https://github.com/Picodes/4naly3er) detectors.
