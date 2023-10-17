# Critical Issues
# High Issues
## Using `delegatecall` in loop
When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.
- Found in src/LSSVMPair.sol: 27194:26:93
# Medium Issues
## Centralization Risk for trusted owners
Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.
- Found in src/mocks/Test721.sol: unknown
- Found in src/settings/StandardSettings.sol: 3146:9:141
- Found in src/erc1155/LSSVMPairERC1155.sol: 9304:9:107
- Found in src/erc1155/LSSVMPairERC1155.sol: 9941:9:107
- Found in src/LSSVMPairFactory.sol: unknown
- Found in src/LSSVMPairFactory.sol: 18367:9:96
- Found in src/LSSVMPairFactory.sol: 18741:9:96
- Found in src/LSSVMPairFactory.sol: 19067:9:96
- Found in src/LSSVMPairFactory.sol: 19544:9:96
- Found in src/LSSVMPairFactory.sol: 20104:9:96
- Found in src/LSSVMPairFactory.sol: 20576:9:96
- Found in src/LSSVMPairFactory.sol: 21167:9:96
- Found in src/LSSVMPairERC20.sol: 5141:9:94
- Found in src/LSSVMPairETH.sol: 3476:9:95
- Found in src/LSSVMPairETH.sol: 3887:9:95
- Found in src/LSSVMPairETH.sol: 4163:9:95
- Found in src/LSSVMPair.sol: 22855:9:93
- Found in src/LSSVMPair.sol: 23343:9:93
- Found in src/LSSVMPair.sol: 23891:9:93
- Found in src/LSSVMPair.sol: 24470:9:93
- Found in src/LSSVMPair.sol: 25030:9:93
- Found in src/LSSVMPair.sol: 26885:9:93
- Found in src/lib/OwnableWithTransferCallback.sol: 1801:9:116
- Found in src/mocks/Test1155.sol: unknown
- Found in src/erc721/LSSVMPairERC721.sol: 11359:9:111
- Found in src/erc721/LSSVMPairERC721.sol: 12080:9:111
## Solmate's SafeTransferLib does not check for token contract's existence
There is a subtle difference between the implementation of solmate's SafeTransferLib and OZ's SafeERC20: OZ's SafeERC20 checks if the token is a contract or not, solmate's SafeTransferLib does not.
https://github.com/transmissions11/solmate/blob/main/src/utils/SafeTransferLib.sol#L9 
`@dev Note that none of the functions in this library check that a token has code at all! That responsibility is delegated to the caller`

- Found in src/test/base/RouterRobustSwapWithRoyalties.sol: 2868:24:151
- Found in src/test/mixins/UsingERC20.sol: 1434:19:174
- Found in src/LSSVMRouter.sol: 21356:22:97
- Found in src/test/base/RouterRobustSwapWithRoyalties.sol: 3435:24:151
- Found in src/erc1155/LSSVMPairERC1155.sol: 7159:21:107
- Found in src/erc1155/LSSVMPairERC1155.sol: 8837:21:107
- Found in src/test/base/RouterRobustSwap.sol: 3172:24:149
- Found in src/LSSVMPairERC20.sol: 3730:23:94
- Found in src/VeryFastRouter.sol: 28311:22:99
- Found in src/LSSVMPairERC20.sol: 3504:23:94
- Found in src/LSSVMPairFactory.sol: 25910:23:96
- Found in src/test/base/RouterRobustSwap.sol: 3739:24:149
- Found in src/mocks/MaliciousRouter.sol: 4100:22:117
- Found in src/LSSVMPairETH.sol: 4183:14:95
- Found in src/settings/Splitter.sol: 2032:18:140
- Found in src/LSSVMPairFactory.sol: 18761:18:96
- Found in src/erc1155/LSSVMPairERC1155.sol: 9409:18:107
- Found in src/LSSVMPairFactory.sol: 27769:21:96
- Found in src/LSSVMPairFactory.sol: 29529:20:96
- Found in src/LSSVMPairERC20.sol: 3994:23:94
- Found in src/LSSVMPairERC20.sol: 5161:14:94
- Found in src/LSSVMPairFactory.sol: 27579:23:96
- Found in src/erc721/LSSVMPairERC721.sol: 11464:18:111
- Found in src/LSSVMPairFactory.sol: 26915:21:96
- Found in src/settings/Splitter.sol: 2225:18:140
- Found in src/LSSVMPairFactory.sol: 28942:22:96
- Found in src/test/base/RouterRobustSwapWithRoyalties.sol: 4002:24:151
- Found in src/LSSVMPairERC20.sol: 4453:20:94
- Found in src/LSSVMPairERC20.sol: 4957:20:94
- Found in src/test/base/RouterRobustSwap.sol: 2605:24:149
# Low Issues
# NC Issues
# Gas Issues
