// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract LtInsteadOfLeExample {
    function lessThan() public pure returns (bool) {
        return 1 < 2;
    }

    function lessThanOrEqual() public pure returns (bool) {
        return 1 <= 2;
    }
}
