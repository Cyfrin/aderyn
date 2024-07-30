// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;

contract SendExample {

    function send1(address payable recipient, uint256 amount) external {
        // GOOD
        bool success = recipient.send(amount); // parent of Send FunctionCall is VariableDeclarationStatement
        require(success, "Send successful!");
    }

    function send2(address payable recipient, uint256 amount) external {
        // GOOD
        doSomething(recipient.send(amount)); // parent of Send FunctionCall is another FunctionCall
    }

    function send3(address payable recipient, uint256 amount) external returns(bool) {
        // GOOD
        return recipient.send(amount); // parent of Send FunctionCall is return
    }

    function send4(address payable recipient, uint256 amount) external {
        // BAD
        recipient.send(amount); // parent of Send FunctionCall is Block (return value is unused)
    }

    function doSomething(bool success) internal pure {
        
    }

}
