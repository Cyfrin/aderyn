// SPDX-License-Identifier: MIT
pragma solidity <0.8.0;
 
contract ArithmeticUnderflowOverflow {
    uint256 public counter;

    function add(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      result = num1 + num2;
    }

    function sub(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      result = num1 - num2;
    }

    function incrementCounter() public {
      counter += 1;
    }

    function decrementCounter() public {
      counter += 1;
    }
}