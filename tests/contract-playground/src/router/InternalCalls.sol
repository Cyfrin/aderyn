// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.29;

// Positive examples

// Straightforward internal calls
contract Basic {
    function help1() private {}

    function main() public {
        help1();
        main();
    }
}

// Basic Inheritance
contract PBasic2 {
    function help1() internal virtual {
        help2();
        PBasic2.help2();
    }

    function help2() internal virtual {}
}

contract Basic2 is PBasic2 {
    struct Orange {
        uint256 age;
    }

    function main() public {
        help1();
        help2();
        PBasic2.help2();
        Basic2.help2();
    }

    function help2() internal override {}

    function g(Orange memory f) public virtual {}
}

contract Basic2Child is Basic2 {
    function g(Basic2.Orange memory f) public override {}

    function gcall() public {
        Orange memory o = Orange(20);
        g(o);
    }
}

// Diamond inheritance (super calls)
contract Basic3Top {
    function live() public virtual {}

    function help() public virtual {
        live();
    }
}

contract Basic3Left is Basic3Top {
    function help() public virtual override {
        super.help();
    }
}

contract Basic3Right is Basic3Top {
    function help() public virtual override {
        super.help();
    }
}

contract Basic3Down1 is Basic3Top, Basic3Left, Basic3Right {
    function help()
        public
        virtual
        override(Basic3Top, Basic3Right, Basic3Left)
    {
        super.help();
    }
}

contract Basic3Down2 is Basic3Left, Basic3Right {
    function live() public virtual override {}

    function help() public virtual override(Basic3Right, Basic3Left) {
        super.help();
        super.live();
        Basic3Left.help();
    }
}

// Libraries

library Basic4Lib {
    function help1(uint256 a, mapping(uint256 => string) storage ref) internal {
        ref[a] = "hello world!";
    }

    function ext1(uint256 a) external {}

    function ext2(uint256 a) public {}
}

contract Basic4 {
    using Basic4Lib for *;
    mapping(uint256 => string) ref;

    function main() public {
        uint256 a = 10;
        uint8 b = 5;

        // Internal calls
        a.help1(ref);
        b.help1(ref);
        Basic4Lib.help1(a, ref);
        Basic4Lib.help1(b, ref);
        priv();
        // External calls
        a.ext1();
        a.ext2();
        this.main();
    }

    function priv() private {}
}

// Getter function

contract PBasic5 {
    function d() external virtual returns (uint256, bool, uint256) {}
}

contract Basic5 is PBasic5 {
    Data public override d;
}

struct Data {
    uint256 a;
    mapping(uint256 => bytes) b; // Will be skipped as return value in getter function
    bool c;
    uint256 d;
}
