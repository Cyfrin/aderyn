// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Ownable} from "../lib/openzeppelin-contracts/contracts/access/Ownable.sol";
import {ReentrancyGuard} from "../lib/openzeppelin-contracts/contracts/security/ReentrancyGuard.sol";

contract AdminContract is Ownable, ReentrancyGuard {
    constructor() Ownable() {}

    function setOwner(address _owner) external onlyOwner nonReentrant {
        _transferOwnership(_owner);
    }

    function someOtherImportantThing() external nonReentrant onlyOwner {
        // do something important
    }
}
