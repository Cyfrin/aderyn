// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract OnceModifierExample {
 
    modifier onlyOnce() {
        _;
    }

    function perform() external onlyOnce {
        
    }
}
