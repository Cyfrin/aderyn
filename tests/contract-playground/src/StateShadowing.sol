// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.5.0;

contract BaseContract{
    address owner;

    modifier isOwner(){
        require(owner == msg.sender);
        _;
    }

}

contract StateShadowingContract is BaseContract{
    address owner;

    constructor() public{
        owner = msg.sender;
    }

    function withdraw() isOwner() external{
        msg.sender.transfer(address(this).balance);
    }
}