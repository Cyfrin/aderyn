// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract TautologicalCompare {
    uint256 public constant f = 7;
    uint256 public constant g = 7;

    // TODO: Blocked until we have Data Flow Analysis.
    // We cannot determine tautology condition on a variable that can
    // change it's value in it's lifetime.
    function check(uint a) external pure returns (bool) {
        // Tautology
        return (a >= a);
    }

    function check2() external pure returns (bool) {
        // Tautology
        return (f >= 7);
    }

    function check3() external pure returns (bool) {
        // Tautology
        return (f < f);
    }

    function check4() external pure returns (bool) {
        // Tautology
        return (f < g);
    }
}

