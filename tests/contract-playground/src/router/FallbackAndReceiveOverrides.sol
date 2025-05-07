// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.28;

contract A {
    fallback() external payable virtual {}

    receive() external payable virtual {}
}

contract B is A {
    fallback() external payable override {}

    receive() external payable override {}
}
