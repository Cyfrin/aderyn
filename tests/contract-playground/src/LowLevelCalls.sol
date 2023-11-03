// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract LowLevelCalls {
    address private s_target;

    constructor(address target) {
        s_target = target;
    }

    function badCalls(uint256 amount) external {
        (bool status, bytes memory data) = payable(s_target).call{value: amount}("");
        require(status, "Gov: failed to send eth");

        (status, ) = payable(s_target).call{value: amount}("");
        require(status, "Gov: failed to send eth");
    }

    function goodCalls(uint256 amount) external {
        bool status;
        address target = s_target;
        assembly {
            status := call(gas(), target, amount, 0, 0, 0, 0)
        }
    }
}