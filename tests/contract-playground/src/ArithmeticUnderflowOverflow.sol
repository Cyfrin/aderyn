// SPDX-License-Identifier: MIT
pragma solidity <0.8.0;
 
contract ArithmeticUnderflowOverflow {
    uint256 public counter;

    // bad
    function add(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      result = num1 + num2;
    }

    // bad
    function sub(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      result = num1 - num2;
    }

    // bad
    function mul(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      result = num1 * num2;
    }

    // bad
    function div(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      result = num1 / num2;
    }

    // bad
    function incrementCounter() public {
      counter += 1;
    }

    // bad
    function decrementCounter() public {
      counter += 1;
    }

    // bad
    function incrementCounter2() public {
      counter++;
    }

    // bad
    function decrementCounter2() public {
      counter--;
    }

    // good
    function subWithCheck(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      require(num1 >= num2, "Subtraction underflow");
      result = num1 - num2;
    }

    // good
    function subWithCheck2(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      if (num1 < num2) {
        revert("Subtraction underflow");
      }
      result = num1 - num2;
    }

    // good
    function addWithCheck(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      uint256 result = num1 + num2;
      require(result >= num1, "Addition overflow");
      return result;
    }

    // good
    function addWithCheck2(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      uint256 result = num1 + num2;
      if (result < num1) {
        revert("Addition overflow");
      }
      return result;
    }

    // good
    function mulWithCheck(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      if (num1 == 0 || num2 == 0) {
        return 0;
      }
      result = num1 * num2;
      require(result / num1 == num2, "Multiplication overflow");
    }

    // good
    function mulWithCheck2(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      if (num1 == 0 || num2 == 0) {
        return 0;
      }
      result = num1 * num2;
      if (result / num1 != num2) {
        revert("Multiplication overflow");
      }
    }

    // good
    function divWithCheck(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      require(num2 != 0, "Division by zero");
      result = num1 / num2;
    }

    // good
    function divWithCheck2(uint256 num1, uint256 num2) public pure returns (uint256 result) {
      if (num2 == 0) {
        revert("Division by zero");
      }
      result = num1 / num2;
    }

    // good
    function incrementCounter3() public {
      require(counter < type(uint256).max, "Counter overflow");
      counter += 1;
    }

    // good
    function decrementCounter3() public {
      require(counter > 0, "Counter underflow");
      counter -= 1;
    }

    // good
    function incrementCounter4() public {
      if (counter == type(uint256).max) {
        revert("Counter overflow");
      }
      counter++;
    }

    // good
    function decrementCounter4() public {
      if (counter == 0) {
        revert("Counter underflow");
      }
      counter--;
    }
}