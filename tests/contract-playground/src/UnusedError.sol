// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract UnusedError {
    uint256 public number = 0;
    
    error CannotRenounceWhilePaused(address account);

    function perform() external {
       number++;
    }
}
