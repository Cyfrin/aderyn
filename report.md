# Critical Issues
# High Issues
## Using `delegatecall` in loop
When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.
- Found in contracts/ExtendedInheritance.sol: 488:19:1
# Medium Issues
# Low Issues
## `abi.encodePacked()` should not be used with dynamic types when passing the result to a hash function such as `keccak256()`
Use `abi.encode()` instead which will pad items to 32 bytes, which will [prevent hash collisions](https://docs.soliditylang.org/en/v0.8.13/abi-spec.html#non-standard-packed-mode) (e.g. `abi.encodePacked(0x123,0x456)` => `0x123456` => `abi.encodePacked(0x1,0x23456)`, but `abi.encode(0x123,0x456)` => `0x0...1230...456`). Unless there is a compelling reason, `abi.encode` should be preferred. If there is only one argument to `abi.encodePacked()` it can often be cast to `bytes()` or `bytes32()` [instead](https://ethereum.stackexchange.com/questions/30912/how-to-compare-strings-in-solidity#answer-82739).
If all arguments are strings and or bytes, `bytes.concat()` should be used instead.
- Found in contracts/KeccakContract.sol: 878:16:4
- Found in contracts/KeccakContract.sol: 584:16:4
- Found in contracts/KeccakContract.sol: 731:16:4
## `ecrecover` is susceptible to signature malleability
The `ecrecover` function is susceptible to signature malleability. This means that the same message can be signed in multiple ways, allowing an attacker to change the message signature without invalidating it. This can lead to unexpected behavior in smart contracts, such as the loss of funds or the ability to bypass access control. Consider using OpenZeppelin's ECDSA library instead of the built-in function.
- Found in contracts/ExtendedInheritance.sol: 705:9:1
## Unsafe ERC20 Operations should not be used
ERC20 functions may not behave as expected. For example: return values are not always meaningful. It is recommended to use OpenZeppelin's SafeERC20 library.
- Found in contracts/Lock.sol: 962:14:5
## Solidity pragma should be specific, not wide
Consider using a specific version of Solidity in your contracts instead of a wide version. For example, instead of `pragma solidity ^0.8.0;`, use `pragma solidity 0.8.0;`
- Found in contracts/InheritanceBase.sol: 32:23:3
- Found in contracts/IContractInheritance.sol: 32:24:2
- Found in contracts/Lock.sol: 39:24:5
- Found in contracts/Counter.sol: 39:24:0
# NC Issues
## Missing checks for `address(0)` when assigning values to address state variables
Assigning values to address state variables without checking for `address(0)`.
- Found in contracts/StateVariables.sol: 2121:14:6
## Functions not used internally could be marked external

- Found in contracts/Counter.sol: 120:80:0
- Found in contracts/StateVariables.sol: 1755:145:6
- Found in contracts/Lock.sol: 516:490:5
- Found in contracts/Lock.sol: 271:239:5
- Found in contracts/StateVariables.sol: 1426:292:6
- Found in contracts/StateVariables.sol: 1906:151:6
- Found in contracts/StateVariables.sol: 2500:376:6
- Found in contracts/StateVariables.sol: 2148:346:6
- Found in contracts/StateVariables.sol: 2063:79:6
## Constants should be defined and used instead of literals

- Found in contracts/Counter.sol: 434:1:0
- Found in contracts/ExtendedInheritance.sol: 466:1:1
## Event is missing `indexed` fields
Index event fields make the field more quickly accessible to off-chain tools that parse events. However, note that each index field costs extra gas during emission, so it's not necessarily best to index the maximum allowed per event (three fields). Each event should use three indexed fields if there are three or more fields, and gas usage is not particularly of concern for the events in question. If there are fewer than three fields, all of the fields should be indexed.
- Found in contracts/InheritanceBase.sol: 150:28:3
- Found in contracts/Lock.sol: 224:41:5
- Found in contracts/ExtendedInheritance.sol: 144:45:1
