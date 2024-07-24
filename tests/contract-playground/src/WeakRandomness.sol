// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract WeakRandomness {
    function getRandomNumber() external view returns (uint256) {
        uint256 randomNumber = uint256(keccak256(abi.encodePacked(msg.sender, block.prevrandao, block.timestamp)));
        return randomNumber;
    }
}