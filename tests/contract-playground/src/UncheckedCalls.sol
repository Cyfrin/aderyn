// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract UncheckedCallExamples {
    
    function sendEther(address payable recipient) external payable {
        recipient.call{value: 100}("");
    }

    function callFunction(address target, bytes calldata data) external {
        target.call(data);
    }

    function delegateCallFunction(address target, bytes calldata data) external {
        target.delegatecall(data);
    }

    function staticCallFunction(address target, bytes calldata data) external view {
        target.staticcall(data);
    }

    function testMultipleUncheckedCalls(address target) external payable {
        target.call{value: 100}("");

        target.call(abi.encodeWithSignature("someFunction(uint256)", 123));

        target.delegatecall(abi.encodeWithSignature("someOtherFunction(address)", msg.sender));

        target.staticcall(abi.encodeWithSignature("aViewFunction()"));
    }

    // Copied from Slither
    function my_func(address payable dst) external payable{
        dst.call.value(100)("");
    }
}
