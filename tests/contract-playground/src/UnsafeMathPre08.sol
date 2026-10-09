// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;

contract UnsafeMathPre08 {
    uint256 public x;

    function add(uint256 y) public {
        x = x + y; // should flag
        x += y;    // should flag
        x++;       // should flag
    }

    function sub(uint256 y) public {
        x = x - y; // should flag
        x -= y;    // should flag
        x--;       // should flag
    }

    function mul(uint256 y) public {
        x = x * y; // should flag
        x *= y;    // should flag
    }

    // Edge case: safe math library mock (call should not flag)
    function safeAdd(uint256 a, uint256 b) internal pure returns (uint256) {
        return a + b; // should flag here (inside the function), but the call won't flag
    }

    function doSafeMath(uint256 y) public {
        x = safeAdd(x, y); // should NOT flag
    }
}

contract SafeMath08 {
    // To test the negative case we would need another file or just another pragma,
    // but Solidity allows only one pragma per file for compiler matching usually, 
    // or we can test multiple files.
}
