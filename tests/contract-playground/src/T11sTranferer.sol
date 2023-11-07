// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {ERC20, SafeTransferLib} from "../lib/solmate/src/utils/SafeTransferLib.sol";

contract T11sTranferer {
    using SafeTransferLib for ERC20;

    function sendSomeTokens(ERC20 token, address to, uint256 amount) external {
        token.safeTransfer(to, amount);
    }
}
