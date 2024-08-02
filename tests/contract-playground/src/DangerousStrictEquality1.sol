// SPDX-License-Identifier: MIT
pragma solidity ^0.4.0;

contract DangerousStrictEquality1 {
    function makeStrictBalanceCheck() external view returns (bool) {
        return this.balance == 100 ether;
    }
}
