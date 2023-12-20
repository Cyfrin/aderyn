// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.19;

contract AssemblyExample {
    uint b;
    function f(uint x) public view returns (uint r) {
        assembly {
            // We ignore the storage slot offset, we know it is zero
            // in this special case.
            r := mul(x, sload(b.slot))
        }
    }
}