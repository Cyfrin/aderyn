// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.29;

interface A {
    function abc() external returns (uint256);
}

contract B is A {
    uint256 public abc;
}

abstract contract C {
    function abc() external virtual returns (uint256);
}

contract D is C {
    uint256 public override abc;
}
