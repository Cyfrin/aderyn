# 🦜 Aderyn: Solidity AST Analyzer

## What is Aderyn?

Aderyn is a Solidity Static Analyzer. It takes a bird's eye view over your smart contracts, traversing the Abstract Syntax Trees (AST) to pinpoint suspected vulnerabilities. Aderyn prints out these potential issues in an easy-to-consume markdown format.

## When/Why to Use it?

Use Aderyn when developing or auditing Solidity smart contracts to quickly identify areas where the code may not be following best practices or has potential vulnerabilities.

## Features

* Hardhat and Foundry support
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

The project you're running aderyn on should be either a Foundry or compiled Hardhat project.

```sh
aderyn --root /path/to/your/foundry/project/root/directory/
```

That's it! `report.md` will be output in the directory in which you ran the command.

## Options

1. You must provide the root directory of the repo you want to analyze:
`--root /path/to/repo/root/`

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
* Current detectors based on [4nalyzer](https://github.com/Picodes/4naly3er) detectors.
