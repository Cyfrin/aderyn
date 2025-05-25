// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.29;

contract A {
    function mercy() external virtual returns (uint256) {
    }
}

contract B is A {
    uint256 public constant override mercy = 8;
}

contract C is A {
    uint256 public immutable override mercy;
}

contract D is A {
    uint256 public override mercy;
}

contract E is A {
    uint256 public transient override mercy;
}

contract H is A {
    uint256 public override transient mercy;
}
