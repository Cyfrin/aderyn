Rust-based Solidity AST analyzer and context builder.

Note: These goals/priorities will change over time.

### Short-term goals - Working Examples:
* [x] Traverse the AST and create a public "Context" symbol table
* [x] Create a detector architecture
* Recreate 4nalyzer detectors
  * [x] [High: delegatecall in loop](https://github.com/Picodes/4naly3er/blob/main/src/issues/H/delegateCallInLoop.ts)
  * [x] [Medium: centralization risk](https://github.com/Picodes/4naly3er/blob/main/src/issues/M/centralizationRisk.ts)
  * [x] [Medium: solmate SafeTransferLib does not check existence](https://github.com/Picodes/4naly3er/blob/main/src/issues/M/solmateSafeTransferLib.ts)
  * [x] [Low: Avoid encodePacked](https://github.com/Picodes/4naly3er/blob/main/src/issues/L/avoidEncodePacked.ts)
  * [x] [Low: Deprecated functions](https://github.com/Picodes/4naly3er/blob/main/src/issues/L/deprecatedFunctions.ts)
  * [ ] ~[Low: Empty function body](https://github.com/Picodes/4naly3er/blob/main/src/issues/L/emptyBody.ts)~
  * [ ] ~[Low: Front-runnable initializer](https://github.com/Picodes/4naly3er/blob/main/src/issues/L/frontRunnableInitializer.ts)~
  * [x] [Low: Unsafe ERC20 Operations](https://github.com/Picodes/4naly3er/blob/main/src/issues/L/unsafeERC20Operations.ts)
  * [x] [Low: Unspecific Pragma](https://github.com/Picodes/4naly3er/blob/main/src/issues/L/unspecifiedPragma.ts)
  * [x] [Low: Use of ecrecover](https://github.com/Picodes/4naly3er/blob/main/src/issues/L/useOfEcrecover.ts)
  * [x] [NC: Address(0) check](https://github.com/Picodes/4naly3er/blob/main/src/issues/NC/address0Check.ts)
  * [ ] [NC: Non-reentrant before modifiers](https://github.com/Picodes/4naly3er/blob/main/src/issues/NC/nonReentrantBeforeModifiers.ts)
  * [ ] [NC: Require with string](https://github.com/Picodes/4naly3er/blob/main/src/issues/NC/requireWithString.ts)
  * [ ] [NC: Return value from approve](https://github.com/Picodes/4naly3er/blob/main/src/issues/NC/returnValueOfApprove.ts)
  * [ ] [NC: Todo in code](https://github.com/Picodes/4naly3er/blob/main/src/issues/NC/todoLeftInTheCode.ts)
  * [x] [NC: Unindexed events](https://github.com/Picodes/4naly3er/blob/main/src/issues/NC/unindexedEvent.ts)
  * [x] [NC: Use constants](https://github.com/Picodes/4naly3er/blob/main/src/issues/NC/useConstants.ts)
  * [x] [NC: Useless public function](https://github.com/Picodes/4naly3er/blob/main/src/issues/NC/uselessPublic.ts)
  * [ ] ~[GAS: All](https://github.com/Picodes/4naly3er/tree/main/src/issues/GAS)~

### Medium-term goals - Auditor Aid:
* [x] Support Multiple Abstract Syntax Trees representing multiple Solidity files
* Support Foundry/Hardhat/Truffle/Solc output formats for ingesting AST
  * [x] Foundry
  * [ ] Hardhat
* Support functionality from:
  * Consensys Solidity Analyzer
    * Complexity
    * nSLOC
    * Percentage YUL code 👀
  * Slither

### Long-term goals - Product:
Create tools that utilize the context library to:
* Provide automated gas optimizations
* Custom subscribable detectors
* Control/data flow analyses
* Symbolic execution lite, with invariant analysis
* Vyper support

## Attribution
* AST Visitor code from [solc-ast-rs](https://github.com/hrkrshnn/solc-ast-rs).
* Current detectors based on [4nalyzer](https://github.com/Picodes/4naly3er) detectors.
