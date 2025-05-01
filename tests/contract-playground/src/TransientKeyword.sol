// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

contract TransientStorageExample {
    uint256 transient tempValue;

    function setTemp(uint256 _val) external {
        tempValue = _val;
    }

    function getTemp() external view returns (uint256) {
        return tempValue;
    }
}
