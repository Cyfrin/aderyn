// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.28;

contract A {
    fallback() external payable virtual {}

    receive() external payable virtual {}
}

contract B is A {
    fallback() external payable override {}
}

contract C is A {}

// Assume we're faking A and B. Note - neither of them have abc() function
interface FakeIt {
    function abc() external;
}

contract TestIt {
    function test(FakeIt f) public {
        f.abc();
    }
}
