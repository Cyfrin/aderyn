// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract WeakRandomness {
    function getRandomNumber1() external view returns (uint256) {
        uint256 randomNumber = uint256(keccak256(abi.encodePacked(msg.sender, block.prevrandao, block.timestamp)));
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
}
