# Table of Contents

- [Contract Summary](#contract-summary)
- [High Issues](#high-issues)
  - [H-1: Using `delegatecall` in loop](#H-1)
- [Low Issues](#low-issues)
  - [L-1: `abi.encodePacked()` should not be used with dynamic types when passing the result to a hash function such as `keccak256()`](#L-1)
  - [L-2: `ecrecover` is susceptible to signature malleability](#L-2)
  - [L-3: Unsafe ERC20 Operations should not be used](#L-3)
  - [L-4: Solidity pragma should be specific, not wide](#L-4)
- [NC Issues](#nc-issues)
  - [NC-1: Missing checks for `address(0)` when assigning values to address state variables](#NC-1)
  - [NC-2: Functions not used internally could be marked external](#NC-2)
  - [NC-3: Constants should be defined and used instead of literals](#NC-3)
  - [NC-4: Event is missing `indexed` fields](#NC-4)


# Contract Summary

Contracts analyzed:

- "contracts/InheritanceBase.sol"
- "contracts/StateVariables.sol"
- "contracts/KeccakContract.sol"
- "contracts/Lock.sol"
- "contracts/Counter.sol"
- "contracts/ExtendedInheritance.sol"
- "contracts/IContractInheritance.sol"


# High Issues

<a name="H-1"></a>
## H-1: Using `delegatecall` in loop

When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.

