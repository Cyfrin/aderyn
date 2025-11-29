// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

/// @title CLZ Legacy Implementations Test
/// @notice Tests old gas-inefficient CLZ/MSB implementations that should be replaced
/// @dev L2 Issue: These patterns should be replaced with native CLZ opcode
contract CLZLegacyImplementationsTest {
    // ============================================
    // L2.1: Uniswap/PRBMath Binary Search Style
    // ============================================

    /// @notice LEGACY: Uniswap V3 style mostSignificantBit
    /// @dev This is the most common legacy pattern - found in Uniswap, PRBMath, Frax, 1inch, Sushi
    function legacyMostSignificantBit(uint256 x) public pure returns (uint256 r) {
        require(x > 0, "MSB of zero is undefined");
        if (x >= 0x100000000000000000000000000000000) {
            x >>= 128;
            r += 128;
        }
        if (x >= 0x10000000000000000) {
            x >>= 64;
            r += 64;
        }
        if (x >= 0x100000000) {
            x >>= 32;
            r += 32;
        }
        if (x >= 0x10000) {
            x >>= 16;
            r += 16;
        }
        if (x >= 0x100) {
            x >>= 8;
            r += 8;
        }
        if (x >= 0x10) {
            x >>= 4;
            r += 4;
        }
        if (x >= 0x4) {
            x >>= 2;
            r += 2;
        }
        if (x >= 0x2) {
            r += 1;
        }
    }

    /// @notice LEGACY: Alternative with 2**N notation
    function legacyMostSignificantBitPow2(uint256 x) public pure returns (uint256 r) {
        require(x > 0);
        if (x >= 2**128) {
            x >>= 128;
            r += 128;
        }
        if (x >= 2**64) {
            x >>= 64;
            r += 64;
        }
        if (x >= 2**32) {
            x >>= 32;
            r += 32;
        }
        if (x >= 2**16) {
            x >>= 16;
            r += 16;
        }
        if (x >= 2**8) {
            x >>= 8;
            r += 8;
        }
        if (x >= 2**4) {
            x >>= 4;
            r += 4;
        }
        if (x >= 2**2) {
            x >>= 2;
            r += 2;
        }
        if (x >= 2**1) {
            r += 1;
        }
    }

    /// @notice LEGACY: PRBMath style with different variable names
    function legacyMsb(uint256 x) internal pure returns (uint256 msb) {
        if (x >= 2**128) {
            x >>= 128;
            msb += 128;
        }
        if (x >= 2**64) {
            x >>= 64;
            msb += 64;
        }
        if (x >= 2**32) {
            x >>= 32;
            msb += 32;
        }
        if (x >= 2**16) {
            x >>= 16;
            msb += 16;
        }
        if (x >= 2**8) {
            x >>= 8;
            msb += 8;
        }
        if (x >= 2**4) {
            x >>= 4;
            msb += 4;
        }
        if (x >= 2**2) {
            x >>= 2;
            msb += 2;
        }
        if (x >= 2**1) {
            msb += 1;
        }
    }

    /// @notice LEGACY: Used in log2 calculation
    function legacyLog2(uint256 x) public pure returns (uint256) {
        require(x > 0, "log2(0) undefined");
        return legacyMostSignificantBit(x);
    }

    /// @notice LEGACY: Used in bitLength calculation
    function legacyBitLength(uint256 x) public pure returns (uint256) {
        if (x == 0) return 0;
        return legacyMostSignificantBit(x) + 1;
    }

    // ============================================
    // L2.2: Linear Scan (while loop)
    // ============================================

    /// @notice LEGACY: Solidity by Example style linear scan
    /// @dev O(256) worst case - extremely gas inefficient!
    function legacyMostSignificantBitLinear(uint256 x) public pure returns (uint256 i) {
        while ((x >>= 1) > 0) {
            ++i;
        }
    }

    /// @notice LEGACY: Alternative with for loop
    function legacyMostSignificantBitForLoop(uint256 x) public pure returns (uint256 msb) {
        require(x > 0, "MSB of zero undefined");
        for (uint256 i = 0; i < 256; i++) {
            if (x == 0) break;
            x >>= 1;
            msb = i;
        }
    }

    /// @notice LEGACY: With counter increment
    function legacyBitLengthLinear(uint256 x) public pure returns (uint256 length) {
        if (x == 0) return 0;
        while (x > 0) {
            x >>= 1;
            length++;
        }
    }

    // ============================================
    // L2.3: Assembly Binary Search with Masks
    // ============================================

    /// @notice LEGACY: Solidity by Example assembly optimization
    /// @dev Uses gt() with masks instead of comparisons
    function legacyMostSignificantBitAsm(uint256 x) public pure returns (uint256 msb) {
        assembly {
            let f := shl(7, gt(x, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
            msb := or(msb, f)
            x := shr(f, x)
        }
        assembly {
            let f := shl(6, gt(x, 0xFFFFFFFFFFFFFFFF))
            msb := or(msb, f)
            x := shr(f, x)
        }
        assembly {
            let f := shl(5, gt(x, 0xFFFFFFFF))
            msb := or(msb, f)
            x := shr(f, x)
        }
        assembly {
            let f := shl(4, gt(x, 0xFFFF))
            msb := or(msb, f)
            x := shr(f, x)
        }
        assembly {
            let f := shl(3, gt(x, 0xFF))
            msb := or(msb, f)
            x := shr(f, x)
        }
        assembly {
            let f := shl(2, gt(x, 0xF))
            msb := or(msb, f)
            x := shr(f, x)
        }
        assembly {
            let f := shl(1, gt(x, 0x3))
            msb := or(msb, f)
            x := shr(f, x)
        }
        assembly {
            let f := gt(x, 0x1)
            msb := or(msb, f)
        }
    }

    /// @notice LEGACY: Single assembly block version
    function legacyMostSignificantBitAsmSingle(uint256 x) public pure returns (uint256 msb) {
        assembly {
            let f := shl(7, gt(x, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
            msb := or(msb, f)
            x := shr(f, x)

            f := shl(6, gt(x, 0xFFFFFFFFFFFFFFFF))
            msb := or(msb, f)
            x := shr(f, x)

            f := shl(5, gt(x, 0xFFFFFFFF))
            msb := or(msb, f)
            x := shr(f, x)

            f := shl(4, gt(x, 0xFFFF))
            msb := or(msb, f)
            x := shr(f, x)

            f := shl(3, gt(x, 0xFF))
            msb := or(msb, f)
            x := shr(f, x)

            f := shl(2, gt(x, 0xF))
            msb := or(msb, f)
            x := shr(f, x)

            f := shl(1, gt(x, 0x3))
            msb := or(msb, f)
            x := shr(f, x)

            f := gt(x, 0x1)
            msb := or(msb, f)
        }
    }

    // ============================================
    // L2.4: EIP-7939 Solidity Fallback (Pre-opcode "Fast CLZ")
    // ============================================

    /// @notice LEGACY: EIP-7939 reference "fastest Solidity CLZ" (~184 gas)
    /// @dev Contains magic constants - easy to detect!
    function legacyClzEIP7939(uint256 x) public pure returns (uint256 r) {
        assembly {
            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
            r := or(r, shl(4, lt(0xffff, shr(r, x))))
            r := or(r, shl(3, lt(0xff, shr(r, x))))
            r := add(
                xor(
                    r,
                    byte(
                        and(0x1f, shr(shr(r, x), 0x8421084210842108cc6318c6db6d54be)),
                        0xf8f9f9faf9fdfafbf9fdfcfdfafbfcfe00000000000000000000000000000000
                    )
                ),
                iszero(x)
            )
        }
    }

    /// @notice LEGACY: Simplified version with magic constants
    function legacyClzWithMagicConstants(uint256 x) public pure returns (uint256 r) {
        assembly {
            // Magic constant 1: 0x8421084210842108cc6318c6db6d54be
            let magic1 := 0x8421084210842108cc6318c6db6d54be
            // Magic constant 2 (truncated to 256 bits)
            let magic2 := 0xf8f9f9faf9fdfafbf9fdfcfdfafbfcfe00000000000000000000000000000000

            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))

            // Use magic constants
            r := add(xor(r, byte(and(0x1f, shr(shr(r, x), magic1)), magic2)), iszero(x))
        }
    }

    // ============================================
    // SAFE IMPLEMENTATIONS (Should NOT be detected)
    // These are modern implementations using CLZ opcode
    // ============================================

    /// @notice SAFE: Modern MSB using CLZ opcode
    /// @dev Should NOT be detected - uses native CLZ
    function safeMostSignificantBit(uint256 x) public pure returns (uint256) {
        require(x > 0, "MSB of zero undefined");
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 255 - clzVal;
    }

    /// @notice SAFE: Modern bitLength using CLZ opcode
    /// @dev Should NOT be detected - uses native CLZ
    function safeBitLength(uint256 x) public pure returns (uint256) {
        if (x == 0) return 0;
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 256 - clzVal;
    }

    // ============================================
    // FALSE POSITIVE PREVENTION TESTS
    // These look similar but are NOT MSB implementations
    // ============================================

    /// @notice NOT MSB: Generic bit manipulation without MSB-like name
    /// @dev Should NOT be detected - no MSB-like function name, not enough power-of-two checks
    function processBits(uint256 value) public pure returns (uint256 result) {
        uint256 temp = value >> 2;  // shift (but not for MSB!)
        result += temp;              // accumulate (but not for MSB!)
        return result;               // return
    }

    /// @notice NOT MSB: Bit packing operation
    /// @dev Should NOT be detected - only 1 power-of-two check, not MSB-like name
    function packData(uint256 x) public pure returns (uint256 packed) {
        if (x >= 2**16) {
            x >>= 16;
            packed += 16;
        }
        return packed;
    }

    /// @notice LEGACY: Linear scan pattern
    /// @dev Semantically identical to linear MSB scan - WILL be detected
    function hashValue(uint256 x) public pure returns (uint256 hash) {
        while ((x >>= 1) > 0) {
            hash++;  // Counts all bits, same as MSB!
        }
        return hash;
    }

    /// @notice NOT MSB: Fixed-point scaling
    /// @dev Should NOT be detected - not MSB-like name, only 2 power-of-two checks
    function scaleValue(uint256 x) public pure returns (uint256 scaled) {
        if (x >= 2**64) {
            x >>= 64;
            scaled += 64;
        }
        if (x >= 2**32) {
            x >>= 32;
            scaled += 32;
        }
        return scaled;
    }

    /// @notice NOT MSB: Generic counter without MSB semantics
    /// @dev Should NOT be detected - no MSB-like name, generic counting
    function countSomething(uint256 x) public pure returns (uint256 count) {
        if (x >= 100) {  // NOT a power of two!
            x >>= 1;
            count += 1;
        }
        return count;
    }

    /// @notice NOT MSB: Bit extraction for different purpose
    /// @dev Should NOT be detected - no MSB-like name, not characteristic pattern
    function extractHighBits(uint256 data) public pure returns (uint256 result) {
        uint256 temp = data;
        if (temp >= 0x1000) {  // Random threshold, not power-of-two pattern
            temp >>= 4;
            result += 4;
        }
        return result;
    }

    /// @notice NOT MSB: AMM price calculation
    /// @dev Should NOT be detected - no MSB-like name, financial calculation
    function calculatePrice(uint256 reserve) public pure returns (uint256 price) {
        if (reserve >= 2**96) {
            reserve >>= 96;
            price += 96;
        }
        if (reserve >= 2**48) {
            reserve >>= 48;
            price += 48;
        }
        // Only 2 checks - not enough for MSB pattern
        return price;
    }

    // ============================================
    // MODERN REPLACEMENT (for comparison)
    // ============================================

    /// @notice MODERN: Using native CLZ opcode
    /// @dev This is what all legacy implementations should be replaced with
    function modernMostSignificantBit(uint256 x) public pure returns (uint256) {
        require(x > 0, "MSB of zero undefined");
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 255 - clzVal;
    }

    /// @notice MODERN: CLZ opcode directly
    function modernClz(uint256 x) public pure returns (uint256 r) {
        assembly {
            r := clz(x)
        }
    }

    /// @notice MODERN: bitLength with CLZ
    function modernBitLength(uint256 x) public pure returns (uint256) {
        if (x == 0) return 0;
        uint256 clzVal;
        assembly {
            clzVal := clz(x)
        }
        return 256 - clzVal;
    }

    // ============================================
    // COMPARISON FUNCTIONS
    // ============================================

    /// @notice Compare legacy vs modern implementations
    function compareLegacyVsModern(uint256 x)
        public
        pure
        returns (uint256 legacyResult, uint256 modernResult, bool resultsMatch)
    {
        if (x == 0) return (0, 0, true);

        legacyResult = legacyMostSignificantBit(x);
        modernResult = modernMostSignificantBit(x);
        resultsMatch = (legacyResult == modernResult);
    }

    /// @notice Test all legacy patterns
    function testAllLegacyPatterns(uint256 x)
        public
        pure
        returns (
            uint256 uniswapStyle,
            uint256 linearScan,
            uint256 asmBinarySearch,
            uint256 eip7939,
            uint256 modern
        )
    {
        if (x == 0) return (0, 0, 0, 0, 0);

        uniswapStyle = legacyMostSignificantBit(x);
        linearScan = legacyMostSignificantBitLinear(x);
        asmBinarySearch = legacyMostSignificantBitAsm(x);
        eip7939 = 255 - legacyClzEIP7939(x);
        modern = modernMostSignificantBit(x);
    }

    // ============================================
    // FALSE POSITIVES (Regression Tests)
    // These should NOT be detected
    // ============================================

    function helper(uint256 x) internal pure returns (uint256) {
        return x;
    }

    /// @notice FALSE POSITIVE 1: Wrapper without CLZ logic
    /// @dev Previously detected because it has "BitLength" in name and calls a function
    function fakeBitLength(uint256 x) public pure returns (uint256) {
        return helper(x) + 1;
    }

    /// @notice FALSE POSITIVE 2: Mixed function with assignment and shift
    /// @dev Previously detected because "y = 5" was seen as accumulator and "x >>= 1" as shift
    function mixedFunction(uint256 x) public pure returns (uint256 y) {
        y = 5;
        x >>= 1;
        return y;
    }

    /// @notice FALSE POSITIVE 3: Accumulator false positive
    /// @dev Previously detected because "y = i" was seen as accumulator
    function accumulatorFalsePositive(uint256 i) public pure returns (uint256 y) {
        y = i;
        return y;
    }
    /// @notice FALSE POSITIVE 4: MSB name but generic logic
    /// @dev Should NOT be detected with semantic analysis
    function msbCounter(uint256 x) public pure returns (uint256 r) {
        r = x; // not accumulator of MSB
        x >>= 1; // just some generic shift
        return r;
    }

    /// @notice LEGACY: Linear scan with generic name
    /// @dev WILL be detected - semantically identical to linear MSB
    function scan(uint256 x) public pure returns (uint256 r) {
        while ((x >>= 1) > 0) {
            r++;
        }
    }

    /// @notice FALSE POSITIVE 5: Complex loop logic
    /// @dev Should NOT be detected - x is modified by XOR (not just shift)
    function complexLoop(uint256 x) public pure returns (uint256 r) {
        while ((x >>= 1) > 0) {
            x ^= 0x1234; // Mutation other than shift!
            r++;
        }
    }

    /// @notice FALSE POSITIVE 6: Variable increment
    /// @dev Should NOT be detected - r is incremented by variable, not constant
    function variableIncrement(uint256 x) public pure returns (uint256 r) {
        while ((x >>= 1) > 0) {
            r += x; // Increment by variable!
        }
    }
}