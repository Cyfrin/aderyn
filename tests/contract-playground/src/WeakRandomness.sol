// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract WeakRandomness {
    function getRandomNumber1() external view returns (uint256) {
        uint256 randomNumber = uint256(keccak256(abi.encodePacked(msg.sender, block.number, block.timestamp)));
        return randomNumber;
    }

    function getRandomNumber2() external view returns (uint256) {
        return uint256(keccak256(abi.encodePacked(block.number)));
    }

    function getRandomNumber3() external view returns (uint256) {
        bytes memory someBytes = abi.encode(block.number, msg.sender);
        return uint256(keccak256(someBytes));
    }

    function getRandomNumber4() external view returns (uint256) {
        bytes memory someBytes = abi.encodePacked(block.number, msg.sender);
        return uint256(keccak256(someBytes));
    }

    function getRandomNumberUsingModulo1() external view returns (uint256) {
        return block.timestamp % 10;
    }

    function getRandomNumberUsingModulo2() external view returns (uint256) {
        uint256 a = block.number;
        uint256 b = 123;
        return a % b;
    }

    function getRandomNumberUsingModulo3() external view returns (uint256) {
        uint256 randomNumber = uint256(blockhash(block.number)) % 10;
        return randomNumber;
    }

    function getRandomNumberUsingModulo4() external view returns (uint256) {
        uint256 hash = uint256(blockhash(12345));
        return hash % 10;
    }

    function getRandomNumberUsingPrevrandao() external view returns (uint256) {
        uint256 randomNumber = block.prevrandao;
        return randomNumber;
    }

    // good
    function timestamp() external view returns (uint256) {
        return block.timestamp;
    }

    function number() external view returns (uint256) {
        return block.number;
    }

    function encode() external view returns (bytes memory) {
        return abi.encode(block.timestamp, block.number);
    }

    function encodePacked() external view returns (bytes memory) {
        return abi.encodePacked(block.timestamp, block.number);
    }

    function moduloOperation(uint256 a, uint256 b) external pure returns (uint256) {
        return a % b;
    }

    function getBlockHashAsUint(uint256 blockNumber) external view returns (uint256) {
        return uint256(blockhash(blockNumber));
    }

    function getDifficulty() external view returns (uint256) {
        return block.difficulty;
    }
}
