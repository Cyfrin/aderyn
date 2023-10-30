# Table of Contents

- [High Issues](#high-issues)
  - [H-1: Using `delegatecall` in loop](#H-1)
- [Medium Issues](#medium-issues)
  - [M-1: Centralization Risk for trusted owners](#M-1)
  - [M-2: Solmate's SafeTransferLib does not check for token contract's existence](#M-2)
- [Low Issues](#low-issues)
  - [L-1: `abi.encodePacked()` should not be used with dynamic types when passing the result to a hash function such as `keccak256()`](#L-1)
  - [L-2: `ecrecover` is susceptible to signature malleability](#L-2)
  - [L-3: Deprecated OpenZeppelin functions should not be used](#L-3)
  - [L-4: Unsafe ERC20 Operations should not be used](#L-4)
  - [L-5: Solidity pragma should be specific, not wide](#L-5)
- [NC Issues](#nc-issues)
  - [NC-1: Missing checks for `address(0)` when assigning values to address state variables](#NC-1)
  - [NC-2: Functions not used internally could be marked external](#NC-2)
  - [NC-3: Constants should be defined and used instead of literals](#NC-3)
  - [NC-4: Event is missing `indexed` fields](#NC-4)
  - [NC-5: `require()` / `revert()` statements should have descriptive reason strings or custom errors](#NC-5)
  - [NC-6: The `nonReentrant` `modifier` should occur before all other modifiers](#NC-6)


# High Issues

<a name="H-1"></a>
## H-1: Using `delegatecall` in loop

When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.

- Found in src/ExtendedInheritance.sol: 488:19:39


# Medium Issues

<a name="M-1"></a>
## M-1: Centralization Risk for trusted owners

Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.

- Found in src/AdminContract.sol: unknown
- Found in src/AdminContract.sol: 377:9:36
- Found in src/AdminContract.sol: 506:9:36
- Found in src/DeprecatedOZFunctions.sol: unknown


<a name="M-2"></a>
## M-2: Solmate's SafeTransferLib does not check for token contract's existence

There is a subtle difference between the implementation of solmate's SafeTransferLib and OZ's SafeERC20: OZ's SafeERC20 checks if the token is a contract or not, solmate's SafeTransferLib does not.
https://github.com/transmissions11/solmate/blob/main/src/utils/SafeTransferLib.sol#L9 
`@dev Note that none of the functions in this library check that a token has code at all! That responsibility is delegated to the caller`


- Found in src/DeprecatedOZFunctions.sol: 579:22:38
- Found in src/DeprecatedOZFunctions.sol: 898:17:38
- Found in src/T11sTranferer.sol: 294:18:44


# Low Issues

<a name="L-1"></a>
## L-1: `abi.encodePacked()` should not be used with dynamic types when passing the result to a hash function such as `keccak256()`

Use `abi.encode()` instead which will pad items to 32 bytes, which will [prevent hash collisions](https://docs.soliditylang.org/en/v0.8.13/abi-spec.html#non-standard-packed-mode) (e.g. `abi.encodePacked(0x123,0x456)` => `0x123456` => `abi.encodePacked(0x1,0x23456)`, but `abi.encode(0x123,0x456)` => `0x0...1230...456`). Unless there is a compelling reason, `abi.encode` should be preferred. If there is only one argument to `abi.encodePacked()` it can often be cast to `bytes()` or `bytes32()` [instead](https://ethereum.stackexchange.com/questions/30912/how-to-compare-strings-in-solidity#answer-82739).
If all arguments are strings and or bytes, `bytes.concat()` should be used instead.

- Found in src/KeccakContract.sol: 731:16:42
- Found in src/KeccakContract.sol: 878:16:42
- Found in src/KeccakContract.sol: 584:16:42


<a name="L-2"></a>
## L-2: `ecrecover` is susceptible to signature malleability

The `ecrecover` function is susceptible to signature malleability. This means that the same message can be signed in multiple ways, allowing an attacker to change the message signature without invalidating it. This can lead to unexpected behavior in smart contracts, such as the loss of funds or the ability to bypass access control. Consider using OpenZeppelin's ECDSA library instead of the built-in function.

- Found in src/ExtendedInheritance.sol: 705:9:39


<a name="L-3"></a>
## L-3: Deprecated OpenZeppelin functions should not be used

Openzeppelin has deprecated several functions and replaced with newer versions. Please consult https://docs.openzeppelin.com/

- Found in src/DeprecatedOZFunctions.sol: 737:10:38
- Found in src/DeprecatedOZFunctions.sol: 898:17:38


<a name="L-4"></a>
## L-4: Unsafe ERC20 Operations should not be used

ERC20 functions may not behave as expected. For example: return values are not always meaningful. It is recommended to use OpenZeppelin's SafeERC20 library.

- Found in src/DeprecatedOZFunctions.sol: 1062:13:38
- Found in src/DeprecatedOZFunctions.sol: 1322:13:38
- Found in src/DeprecatedOZFunctions.sol: 1424:13:38
- Found in src/DeprecatedOZFunctions.sol: 1598:18:38
- Found in src/DeprecatedOZFunctions.sol: 1272:13:38


<a name="L-5"></a>
## L-5: Solidity pragma should be specific, not wide

Consider using a specific version of Solidity in your contracts instead of a wide version. For example, instead of `pragma solidity ^0.8.0;`, use `pragma solidity 0.8.0;`

- Found in src/IContractInheritance.sol: 32:24:40
- Found in src/Counter.sol: 39:24:37
- Found in src/InheritanceBase.sol: 32:23:41


# NC Issues

<a name="NC-1"></a>
## NC-1: Missing checks for `address(0)` when assigning values to address state variables

Assigning values to address state variables without checking for `address(0)`.

- Found in src/StateVariables.sol: 2121:14:43


<a name="NC-2"></a>
## NC-2: Functions not used internally could be marked external



- Found in src/AdminContract.sol: 302:26:36
- Found in src/StateVariables.sol: 2500:376:43
- Found in src/StateVariables.sol: 1755:145:43
- Found in src/Counter.sol: 120:80:37
- Found in src/StateVariables.sol: 1426:292:43
- Found in src/StateVariables.sol: 2148:346:43
- Found in src/StateVariables.sol: 2063:79:43
- Found in src/StateVariables.sol: 1906:151:43


<a name="NC-3"></a>
## NC-3: Constants should be defined and used instead of literals



- Found in src/Counter.sol: 434:1:37
- Found in src/ExtendedInheritance.sol: 466:1:39


<a name="NC-4"></a>
## NC-4: Event is missing `indexed` fields

Index event fields make the field more quickly accessible to off-chain tools that parse events. However, note that each index field costs extra gas during emission, so it's not necessarily best to index the maximum allowed per event (three fields). Each event should use three indexed fields if there are three or more fields, and gas usage is not particularly of concern for the events in question. If there are fewer than three fields, all of the fields should be indexed.

- Found in src/ExtendedInheritance.sol: 144:45:39
- Found in src/InheritanceBase.sol: 150:28:41


<a name="NC-5"></a>
## NC-5: `require()` / `revert()` statements should have descriptive reason strings or custom errors



- Found in src/DeprecatedOZFunctions.sol: 1389:6:38
- Found in src/DeprecatedOZFunctions.sol: 1264:7:38


<a name="NC-6"></a>
## NC-6: The `nonReentrant` `modifier` should occur before all other modifiers

This is a best-practice to protect against reentrancy in other modifiers

- Found in src/AdminContract.sol: 334:109:36


