// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract ThisShouldBeExcluded {
    uint256 public number = 0;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
    }

    // TODO
    function callIncrement() external {
        increment();
    }

    /// TODO
    function incrementByTwoMagic() external {
        // TODO
        number += 2;
    }

    uint256 public constant TWO = 2;

    function incrementByTwoConstant() external {
        number += TWO;
    }
}
