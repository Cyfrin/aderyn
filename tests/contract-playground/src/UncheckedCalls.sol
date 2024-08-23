// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract UncheckedCallExamples {
    
    function sendEther(address payable recipient) public payable {
        recipient.call{value: msg.value}("");
    }

    function callFunction(address target, bytes memory data) public {
        target.call(data);
    }

    function delegateCallFunction(address target, bytes memory data) public {
        target.delegatecall(data);
    }

    function staticCallFunction(address target, bytes memory data) public view {
        target.staticcall(data);
    }

    function testMultipleUncheckedCalls(address target) public payable {
        target.call{value: msg.value}("");

        target.call(abi.encodeWithSignature("someFunction(uint256)", 123));

        target.delegatecall(abi.encodeWithSignature("someOtherFunction(address)", msg.sender));

        target.staticcall(abi.encodeWithSignature("aViewFunction()"));
    }

    // Copied from Slither
    function my_func(address payable dst) public payable{
        dst.call.value(msg.value)("");
    }
}
