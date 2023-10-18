// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract KeccakContract {
    function success0(bytes memory input) external pure returns (bytes32) {
        return keccak256(abi.encodePacked(input));
    }

    function success1(string memory a) external pure returns (bytes32) {
        return keccak256(abi.encodePacked(a));
    }

    function success2(uint[] memory a) external pure returns (bytes32) {
        return keccak256(abi.encodePacked(a));
    }

    function failure0(bytes memory a, bytes memory b) external pure returns (bytes32) {
        return keccak256(abi.encodePacked(a, b));
    }

    function failure1(string memory a, string memory b) external pure returns (bytes32) {
        return keccak256(abi.encodePacked(a, b));
    }

    function failure2(uint[] memory a, uint[] memory b) external pure returns (bytes32) {
        return keccak256(abi.encodePacked(a, b));
    }

}