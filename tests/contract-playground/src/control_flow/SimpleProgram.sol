// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract SimpleProgram {
    uint256 public constant USEME = 100;

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
        if (mod > USEME) mod += times;
        mod = start - times;
        mod = start - times;
        mod = start - times;
        if (mod > USEME) mod += times;
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
        for (uint256 i = 0; i < USEME; ++i) {
            total += i;
        }
        emit Hoorah(total);
    }

    function function8() external {
        uint256 total = 0;
        for (uint256 i = 0; i < USEME; ++i) {
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
            total *= USEME;
        } while (total < USEME);
    }

    function function10() external {
        uint256 i;
        unchecked {
            i -= USEME;
            i -= USEME * i;
        }
    }

    function function11() external {
        uint256 total = 0;
        do {
            total *= USEME;
            if (total < USEME) break;
            total *= USEME;
        } while (total < USEME);
    }

    function function12() external {
        uint256 total = 0;
        do {
            total *= USEME;
            for (uint256 i = 0; i < USEME; ++i) {
                if (total < USEME) break;
                total -= USEME;
            }
            total *= USEME;
        } while (total < USEME);
    }

    function function13() external {
        uint256 total = 0;
        do {
            total *= USEME;
            while (total < USEME) {
                if (total < USEME) break;
                total -= USEME;
            }
            total *= USEME;
        } while (total < USEME);
    }

    function function14() external {
        uint256 total = 0;
        do {
            total *= USEME;
            while (total < USEME) {
                if (total < USEME) continue;
                total -= USEME;
            }
            if (total == USEME) continue;
            total *= USEME;
        } while (total < USEME);
    }

    function function15() external {
        uint256 total = USEME;
        do {
            total *= USEME;
            while (total < USEME) {
                if (total < USEME) break;
                total -= USEME;
            }
            if (total == USEME) continue;
            total *= USEME;
            for (;;) {
                if (total == USEME) {
                    continue;
                }
            }
        } while (total < USEME);
    }

    function function16() external {
        uint256 total = 0;
        do {
            total *= USEME;
            while (total < USEME) {
                if (total < USEME) break;
                total -= USEME;
            }
            if (total == USEME) continue;
            total *= USEME;

            for (;;) {
                if (total == USEME) {
                    return;
                } else if (total == USEME) {
                    continue;
                }
            }
        } while (total < USEME);
    }
}
