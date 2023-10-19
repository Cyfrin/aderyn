// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Ownable} from "../lib/openzeppelin-contracts/contracts/access/Ownable.sol";

contract AdminContract is Ownable {

    constructor() Ownable() {}

    function setOwner(address _owner) external onlyOwner {
        _transferOwnership(_owner);
    }
}