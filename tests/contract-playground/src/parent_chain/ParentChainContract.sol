// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Ownable} from "../../lib/openzeppelin-contracts/contracts/access/Ownable.sol";
import {ReentrancyGuard} from "../../lib/openzeppelin-contracts/contracts/security/ReentrancyGuard.sol";

contract ParentChainContract is Ownable, ReentrancyGuard {

    uint256 public number = 0;

    function setNumber(uint256 newNumber) public {
        
    }

    function increment(uint256 newNumber) public {
        if (number < 5 && number > 1) {
            for (uint256 j = 9; j != 0; --j) {
                number = newNumber;
                number++;
            }
        }
    }

    constructor() Ownable() {

    }

    function setOwner(address _owner) external onlyOwner nonReentrant {
        _transferOwnership(_owner);
    }

    function someOtherImportantThing() external nonReentrant onlyOwner {
        // do something important
    }
}


contract AnotherOne {

    function setOwner(address _owner) external {
        
    }

    function someOtherImportantThing() external {
        // do something important
    }

}