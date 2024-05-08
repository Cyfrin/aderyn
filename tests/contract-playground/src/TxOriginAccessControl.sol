// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

contract OriginControlledTransfer {
    address payable private _owner;

    event TransferMade(address indexed sender, address indexed receiver, uint256 amount);

    constructor() {
        _owner = payable(msg.sender);
    }

    modifier onlyOwner() {
        require(tx.origin == _owner, "Caller is not the owner");
        _;
    }

    function changeOwner(address payable newOwner) public onlyOwner {
        _owner = newOwner;
    }

    function executeTransfer(address payable recipient, uint256 value) public onlyOwner {
        recipient.transfer(value);
        emit TransferMade(msg.sender, recipient, value);
    }

    function safeSend(address payable receiver, uint amount) public {
        require(tx.origin == _owner, "Caller is not the owner");
        receiver.transfer(amount);
    }
    receive() external payable {}
}
