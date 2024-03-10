// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract OnceModifierExample {
 
    modifier onlyOnce() {
        _;
    }

    function perform() public onlyOnce {
        
    }
}
