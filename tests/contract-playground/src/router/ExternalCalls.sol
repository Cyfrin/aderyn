// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.29;

interface A {
    function abc() external returns (uint256);
}

contract B is A {
    uint256 public abc;
}

contract E is A {
    function abc() external virtual returns (uint256) {}
}

contract F is E {
    function abc() public override returns (uint256) {}
}

contract Y is F {}

contract TestA {
    function test(A a) public {
        // If called on B, it is public state variable getter
        // If called on E, it is ext function defined in E
        // If called on F, it is public function defined in F
        a.abc();
    }
}

abstract contract C {
    function abc() external virtual returns (uint256);
}

contract D is C {
    uint256 public override abc;
}

contract TestD {
    function test(D d) public view {
        d.abc();
    }
}
