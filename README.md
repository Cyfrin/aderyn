Rust-based Solidity AST analyzer and context builder.

Note: These goals/priorities will change over time.

### Short-term goals:
* Traverse the AST and create a public "Context" symbol table
* Publish as a reusable library

### Medium-term goals:
* Support Multiple Abstract Syntax Trees representing multiple Solidity files
* Support Foundry/Hardhat/Truffle/Solc output formats for ingesting AST
* Support Vyper

### Long-term goals:
Create tools that utilize this context library to:
* Provide automated gas optimizations
* Custom subscribable detectors
* Control/data flow analyses
* Symbolic execution lite, with invariant analysis

## Attribution
* AST Visitor code from [solc-ast-rs](https://github.com/hrkrshnn/solc-ast-rs).
* Current adapter clones based on [Slither](https://github.com/crytic/slither) detectors.
