// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// COPIED from SLITHER Wiki
contract CostlyOperationsInLoop {
    uint loop_count = 100;
    uint state_variable = 0;

    function bad() external {
        for (uint i = 0; i < loop_count; i++) {
            state_variable++;
        }
    }

    function good() external {
        uint local_variable = state_variable;
        for (uint i = 0; i < loop_count; i++) {
            local_variable++;
        }
        state_variable = local_variable;
    }
}
