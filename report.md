# Critical Issues
# High Issues
## Using `delegatecall` in loop
When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.
- Found in src/ExtendedInheritance.sol: 421:19:0
# Medium Issues
## Centralization Risk for trusted owners
Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.
- Found in src/AdminContract.sol: unknown
- Found in src/AdminContract.sol: 258:9:35
- Found in src/DeprecatedOZFunctions.sol: unknown
## Solmate's SafeTransferLib does not check for token contract's existence
There is a subtle difference between the implementation of solmate's SafeTransferLib and OZ's SafeERC20: OZ's SafeERC20 checks if the token is a contract or not, solmate's SafeTransferLib does not.
https://github.com/transmissions11/solmate/blob/main/src/utils/SafeTransferLib.sol#L9 
`@dev Note that none of the functions in this library check that a token has code at all! That responsibility is delegated to the caller`

- Found in src/DeprecatedOZFunctions.sol: 898:17:37
- Found in src/T11sTranferer.sol: 294:18:43
- Found in src/DeprecatedOZFunctions.sol: 579:22:37
# Low Issues
## `abi.encodePacked()` should not be used with dynamic types when passing the result to a hash function such as `keccak256()`
Use `abi.encode()` instead which will pad items to 32 bytes, which will [prevent hash collisions](https://docs.soliditylang.org/en/v0.8.13/abi-spec.html#non-standard-packed-mode) (e.g. `abi.encodePacked(0x123,0x456)` => `0x123456` => `abi.encodePacked(0x1,0x23456)`, but `abi.encode(0x123,0x456)` => `0x0...1230...456`). Unless there is a compelling reason, `abi.encode` should be preferred. If there is only one argument to `abi.encodePacked()` it can often be cast to `bytes()` or `bytes32()` [instead](https://ethereum.stackexchange.com/questions/30912/how-to-compare-strings-in-solidity#answer-82739).
If all arguments are strings and or bytes, `bytes.concat()` should be used instead.
- Found in src/KeccakContract.sol: 584:16:41
- Found in src/KeccakContract.sol: 731:16:41
- Found in src/KeccakContract.sol: 878:16:41
## `ecrecover` is susceptible to signature malleability
The `ecrecover` function is susceptible to signature malleability. This means that the same message can be signed in multiple ways, allowing an attacker to change the message signature without invalidating it. This can lead to unexpected behavior in smart contracts, such as the loss of funds or the ability to bypass access control. Consider using OpenZeppelin's ECDSA library instead of the built-in function.
- Found in src/ExtendedInheritance.sol: 638:9:0
## Deprecated OpenZeppelin functions should not be used
Openzeppelin has deprecated several functions and replaced with newer versions. Please consult https://docs.openzeppelin.com/
- Found in src/DeprecatedOZFunctions.sol: 737:10:37
- Found in src/DeprecatedOZFunctions.sol: 898:17:37
## Unsafe ERC20 Operations should not be used
ERC20 functions may not behave as expected. For example: return values are not always meaningful. It is recommended to use OpenZeppelin's SafeERC20 library.
- Found in src/DeprecatedOZFunctions.sol: 1236:18:37
- Found in src/DeprecatedOZFunctions.sol: 1062:13:37
## Solidity pragma should be specific, not wide
Consider using a specific version of Solidity in your contracts instead of a wide version. For example, instead of `pragma solidity ^0.8.0;`, use `pragma solidity 0.8.0;`
- Found in src/IContractInheritance.sol: 32:24:39
- Found in src/Counter.sol: 39:24:36
- Found in src/InheritanceBase.sol: 32:23:2
# NC Issues
## Missing checks for `address(0)` when assigning values to address state variables
Assigning values to address state variables without checking for `address(0)`.
- Found in src/StateVariables.sol: 2121:14:1
