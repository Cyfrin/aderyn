// SPDX-License-Identifier: Unlicensed

pragma solidity 0.8.24;

contract WeakRandomness {
    receive() external payable {}

    function weak1() public returns (uint) {
        uint answer = uint(
            keccak256(abi.encodePacked(blockhash(block.number - 1), block.timestamp))
        );
    
        return answer;
    }

    function weak2() public returns (uint256) {
        uint answer = uint256(
            keccak256(abi.encodePacked(blockhash(block.number - 1), block.timestamp))
        );
    
        return answer;
    }

}             
        