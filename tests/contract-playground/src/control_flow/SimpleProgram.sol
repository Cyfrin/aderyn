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

    function function5(
        uint256 num,
        uint256 increment,
        uint256 max
    ) external returns (uint256) {
        uint256 i = num;
        while (num < max) {
            i += increment;
        }
        num *= num;
        while (--num > max) return i;
        i *= increment;
    }

    event Hoorah(uint256 indexed f);

    function function6(
        uint256 num,
        uint256 increment,
        uint256 max
    ) external returns (uint256) {
        uint256 i = num;
        while (num < max) {
            if (i + increment < max) {
                emit Hoorah(i);
                i += increment;
            } else {
                i += 1;
            }
        }
        return i;
    }

    function function7() external {
        uint256 total = 0;
        for (uint256 i = 0; i < 100; ++i) {
            total += i;
        }
        emit Hoorah(total);
    }

    function function8() external {
        uint256 total = 0;
        for (uint256 i = 0; i < 100; ++i) {
            if (total % i == 0) {
                continue;
            }
            for (uint256 j = 0; j < i; ++j) {
                total += i;
            }
        }
    }

    function function9() external {
        uint256 total = 0;
        do {
            total *= 2;
        } while (total < 10000);
    }

    function function10() external {
        uint256 i;
        unchecked {
            i -= 319;
            i -= 2 * i;
        }
    }
}
