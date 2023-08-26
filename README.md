Rust-based Solidity AST analyzer.

Note: These goals/priorities will change over time.

### Short-term goals:
* Replicate Slither detectors
* Replicate 4nalyzer detectors
* Replicate Solhint rules

### Medium-term goals:
* Gas optimization support
* Vyper support
* Custom subscribable detectors

### Long-term goals:
* Control/data flow analyses
* Symbolic execution lite, with invariant analysis

### In progress:
* [ABI encodePacked Collision](https://github.com/crytic/slither/wiki/Detector-Documentation#abi-encodePacked-collision) visitor


## Attribution
* AST Visitor code from [solc-ast-rs](https://github.com/hrkrshnn/solc-ast-rs).
* Current adapter clones based on [Slither](https://github.com/crytic/slither) detectors.
