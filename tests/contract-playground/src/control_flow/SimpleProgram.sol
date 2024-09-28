// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract SimpleProgram {
    function function1(uint256 start, uint256 times, uint256 mod) external {
        uint256 c = start;
        c = start * times;
        c %= mod;
    }

    function function2(uint256 start, uint256 times, uint256 mod) external {
        uint256 c = start;
        c = start * times;
        c %= mod;
        {
            c = start * times;
            c %= mod;
        }
        mod = start - times;
        c = start * times;
        c = start * times;
    }

    function function3(uint256 start, uint256 times, uint256 mod) external {
        uint256 c = start;
        c = start * times;
        c %= mod;
        {
            c = start * times;
            c %= mod;
        }
        mod = start - times;
    }

    function function4(uint256 start, uint256 times, uint256 mod) external {
        uint256 c = start;
        c = start * times;
        c %= mod;
        if (start != times) {
            c = start * times;
            c %= mod;
        } else c++;
        mod = start - times;
        if (mod > 12) mod += times;
        mod = start - times;
        mod = start - times;
        mod = start - times;
        if (mod > 12) mod += times;
        else {
            mod = start - times;
        }
    }
}
