// SPDX-License-Identifier: MIT
pragma solidity ^0.5.0;

interface BaseInterface {
    function f1(bool x) external returns (uint);

    function f2(bool a) external returns (uint);
}

interface BaseInterface2 {
    function f3() external returns (uint);
}

contract DerivedContract is BaseInterface, BaseInterface2 {
    function f1(bool x) external returns (uint) {
        return 42;
    }

    function f4() internal pure returns (bool) {
        return true;
    }
}
