// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract DangerousStrictEquality2 {
    function makeStrictBalanceCheck() external view returns (bool) {
        return address(this).balance == 100 ether;
    }

    function makeStrictBalanceCheck2() external view returns (bool) {
        return payable(address(this)).balance == 100 ether;
    }
}
