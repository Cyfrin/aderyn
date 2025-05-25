// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.29;

abstract contract A {
    modifier modify() virtual;

    function geez() public modify {

    }
}

contract B is A {

    modifier modify() virtual override  {
        _;
    }

    modifier helper() {
        _;
    }

    function tree() public modify {

    }
}


contract C is B {
    using D for uint256;

    modifier modify() override  {
        _;
    }

    function main() public B.modify modify {
        uint256 a = 10;
        a.show();
    }
}

library D {
    modifier modify() {
        _;
    }

    function show(uint256 a) internal modify {

    }
}
