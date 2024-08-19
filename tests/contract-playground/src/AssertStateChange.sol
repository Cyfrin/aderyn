// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

// COPIED from Slither
contract AssertUsage {
    uint s_a;

    function bad() public {
        assert((s_a += 1) > 10);
    }

    function good() public {
        s_a += 1;
        assert(s_a > 10);
    }
}
