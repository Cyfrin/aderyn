// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// BAD
contract NoWithdraw {
    // Event to log deposits
    event Deposited(address indexed sender, uint256 amount);

    // Public payable function to receive Ether
    receive() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Public payable fallback function to handle any data sent with Ether
    fallback() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Function to get the balance of the contract
    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }
}

/*
    Even though the `NoWithdraw2` has an internal function that can send eth away, it's not reachable
    by any public / external function hence it is a bad contract.
*/

// BAD
contract NoWithdraw2 {
    // Event to log deposits
    event Deposited(address indexed sender, uint256 amount);

    // Event to log transfers
    event Transferred(address indexed to, uint256 amount);

    // Public payable function to receive Ether
    receive() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Public payable fallback function to handle any data sent with Ether
    fallback() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Function to get the balance of the contract
    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }

    // Internal function to send Ether to a given address
    function _sendEther(address payable recipient, uint256 amount) internal {
        require(address(this).balance >= amount, "Insufficient balance");
        (bool success, ) = recipient.call{value: amount}("");
        require(success, "Transfer failed");
        emit Transferred(recipient, amount);
    }
}

// GOOD
contract CanWithdraw {
    // Event to log deposits
    event Deposited(address indexed sender, uint256 amount);

    // Event to log transfers
    event Transferred(address indexed to, uint256 amount);

    // Public payable function to receive Ether
    receive() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Public payable fallback function to handle any data sent with Ether
    fallback() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Function to get the balance of the contract
    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }

    // Internal function to send Ether to a given address
    function _sendEther(address payable recipient, uint256 amount) internal {
        require(address(this).balance >= amount, "Insufficient balance");
        (bool success, ) = recipient.call{value: amount}("");
        require(success, "Transfer failed");
        emit Transferred(recipient, amount);
    }

    // This function allows for the withdrawal of eth. Hence this contract is a GOOD contract.
    function takeEthBack(uint256 amount) external {
        _sendEther(payable(msg.sender), amount);
    }
}

// GOOD
contract CanWithdraw2 {
    // Event to log deposits
    event Deposited(address indexed sender, uint256 amount);

    // Event to log transfers
    event Transferred(address indexed to, uint256 amount);

    // Public payable function to receive Ether
    receive() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Public payable fallback function to handle any data sent with Ether
    fallback() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Function to get the balance of the contract
    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }

    // Internal function to send Ether to a given address
    function _sendEther(address payable recipient, uint256 amount) internal {
        require(address(this).balance >= amount, "Insufficient balance");
        bool success = recipient.send(amount);
        require(success, "Transfer failed");
        emit Transferred(recipient, amount);
    }

    // This function allows for the withdrawal of eth. Hence this contract is a GOOD contract.
    function takeEthBack(uint256 amount) external {
        _sendEther(payable(msg.sender), amount);
    }
}

// GOOD
contract CanWithdraw3 {
    // Event to log deposits
    event Deposited(address indexed sender, uint256 amount);

    // Event to log transfers
    event Transferred(address indexed to, uint256 amount);

    // Public payable function to receive Ether
    receive() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Public payable fallback function to handle any data sent with Ether
    fallback() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Function to get the balance of the contract
    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }

    // Internal function to send Ether to a given address
    function _sendEther(address payable recipient, uint256 amount) internal {
        require(address(this).balance >= amount, "Insufficient balance");
        recipient.transfer(amount);
        emit Transferred(recipient, amount);
    }

    // This function allows for the withdrawal of eth. Hence this contract is a GOOD contract.
    function takeEthBack(uint256 amount) external {
        _sendEther(payable(msg.sender), amount);
    }
}

// GOOD (no payable functions)
contract CanWithdrawParent {
    // Event to log deposits
    event Deposited(address indexed sender, uint256 amount);

    // Event to log transfers
    event Transferred(address indexed to, uint256 amount);

    // Function to get the balance of the contract
    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }

    // Internal function to send Ether to a given address
    function _sendEther(address payable recipient, uint256 amount) internal {
        require(address(this).balance >= amount, "Insufficient balance");
        (bool success, ) = recipient.call{value: amount}("");
        require(success, "Transfer failed");
        emit Transferred(recipient, amount);
    }

    // This function allows for the withdrawal of eth. Hence this contract is a GOOD contract.
    function takeEthBack(uint256 amount) external {
        _sendEther(payable(msg.sender), amount);
    }
}

// GOOD (It has payable functions, but we can withdraw (look at the parent's contract)
contract CanWithdrawChild is CanWithdrawParent {
    // Public payable function to receive Ether
    receive() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Public payable fallback function to handle any data sent with Ether
    fallback() external payable {
        emit Deposited(msg.sender, msg.value);
    }
}

import "../lib/openzeppelin-contracts/contracts/utils/Address.sol";

// GOOD
contract CanWithdrawOZ {
    using Address for address payable;

    // Event to log deposits
    event Deposited(address indexed sender, uint256 indexed amount);

    // Event to log transfers
    event Transferred(address indexed to, uint256 indexed amount);

    // Public payable function to receive Ether
    receive() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Public payable fallback function to handle any data sent with Ether
    fallback() external payable {
        emit Deposited(msg.sender, msg.value);
    }

    // Internal function to send Ether to a given address
    function _sendEther(address payable recipient, uint256 amount) internal {
        require(address(this).balance >= amount, "Insufficient balance");
        require(recipient != address(0), "Invalid recipient");
        recipient.sendValue(amount);
        emit Transferred(recipient, amount);
    }

    // This function allows for the withdrawal of eth. Hence this contract is a GOOD contract.
    function takeEthBack(uint256 amount) external {
        _sendEther(payable(msg.sender), amount);
    }
}
