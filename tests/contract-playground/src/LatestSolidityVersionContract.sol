// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

contract LatestSolidityVersionMsgStore {
    string public message;

    constructor(string memory msg_) {
        message = msg_;
    }

    function setMessage(string memory msg_) external {
        message = msg_;
    }
}
