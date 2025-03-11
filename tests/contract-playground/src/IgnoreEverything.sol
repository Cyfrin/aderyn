// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract IgnoreEverything {
    // This will be not reported by either of the detectors
    // aderyn-ignore-next-line(unused-public-function,empty-block)
    function f2() public {}

    // This will never be reported
    // aderyn-ignore-next-line
    function f3() public {}
}
