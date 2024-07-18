// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract YulReturn {

    function hasYulReturn() external pure returns(uint256) {
        assembly {
            return(0, 0)
        }
    }

}