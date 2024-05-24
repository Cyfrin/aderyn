// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

contract ERC20Bytes32 {
    function symbol() external pure returns (bytes32) {
        return bytes32("ERC20");
    }
}
