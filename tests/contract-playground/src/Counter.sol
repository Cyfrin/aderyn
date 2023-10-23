// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number = 0;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
    }

    function callIncrement() external {
        increment();
    }

    function incrementByTwoMagic() external {
        number += 2;
    }

    uint256 constant public TWO = 2;

    function incrementByTwoConstant() external {
        number += TWO;
    }
}
