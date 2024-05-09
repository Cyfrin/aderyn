// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.19;

import {SafeCast} from "../lib/openzeppelin-contracts/contracts/utils/math/SafeCast.sol";

contract Casting {
    using SafeCast for uint256;
    using SafeCast for int256;

    uint8 public uint8Value = 0x12;
    int8 public int8Value = -0x12;
    bytes1 public bytes1Value = 0x12;

    // All good
    function safeCastingExamples() external pure returns (uint128 b, int128 d, uint128 x, int128 y) {
        uint256 a = 0x1234567890abcdef;
        if (a > type(uint128).max) {
            revert("Value too large for uint128");
        }
        b = uint128(a);

        int256 c = -0x1234567890abcdef;
        require(c >= type(int128).min && c <= type(int128).max, "Value does not fit in int128");
        d = int128(c);

        x = a.toUint128();
        y = c.toInt128();
    }

    function unsafeUintCasting() external {
        uint unspecificUint = 0x1234567890abcdef;
        // Good
        uint256 a = uint256(unspecificUint);
        // Bad - all the way down to uint8
        uint248 b = uint248(a);
        uint240 c = uint240(b);
        uint232 d = uint232(c);
        uint224 e = uint224(d);
        uint216 f = uint216(e);
        uint208 g = uint208(f);
        uint200 h = uint200(g);
        uint192 i = uint192(h);
        uint184 j = uint184(i);
        uint176 k = uint176(j);
        uint168 l = uint168(k);
        uint160 m = uint160(l);
        uint152 n = uint152(m);
        uint144 o = uint144(n);
        uint136 p = uint136(o);
        uint128 q = uint128(p);
        uint120 r = uint120(q);
        uint112 s = uint112(r);
        uint104 t = uint104(s);
        uint96 u = uint96(t);
        uint88 v = uint88(u);
        uint80 w = uint80(v);
        uint72 x = uint72(w);
        uint64 y = uint64(x);
        uint56 z = uint56(y);
        uint48 aa = uint48(z);
        uint40 ab = uint40(aa);
        uint32 ac = uint32(ab);
        uint24 ad = uint24(ac);
        uint16 ae = uint16(ad);
        uint8Value = uint8(ae);
    }

    function unsafeIntCasting() external {
        int unspecificInt = -0x1234567890abcdef;
        // Good
        int256 a = int256(unspecificInt);
        // Bad - all the way down to int8
        int248 b = int248(a);
        int240 c = int240(b);
        int232 d = int232(c);
        int224 e = int224(d);
        int216 f = int216(e);
        int208 g = int208(f);
        int200 h = int200(g);
        int192 i = int192(h);
        int184 j = int184(i);
        int176 k = int176(j);
        int168 l = int168(k);
        int160 m = int160(l);
        int152 n = int152(m);
        int144 o = int144(n);
        int136 p = int136(o);
        int128 q = int128(p);
        int120 r = int120(q);
        int112 s = int112(r);
        int104 t = int104(s);
        int96 u = int96(t);
        int88 v = int88(u);
        int80 w = int80(v);
        int72 x = int72(w);
        int64 y = int64(x);
        int56 z = int56(y);
        int48 aa = int48(z);
        int40 ab = int40(aa);
        int32 ac = int32(ab);
        int24 ad = int24(ac);
        int16 ae = int16(ad);
        int8Value = int8(ae);
    }

    function unsafeBytes32Casting() external {
        bytes32 unspecificBytes32 = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
        // Good
        bytes32 a = bytes32(unspecificBytes32);
        // Bad - all the way down to bytes1
        bytes31 b = bytes31(a);
        bytes30 c = bytes30(b);
        bytes29 d = bytes29(c);
        bytes28 e = bytes28(d);
        bytes27 f = bytes27(e);
        bytes26 g = bytes26(f);
        bytes25 h = bytes25(g);
        bytes24 i = bytes24(h);
        bytes23 j = bytes23(i);
        bytes22 k = bytes22(j);
        bytes21 l = bytes21(k);
        bytes20 m = bytes20(l);
        bytes19 n = bytes19(m);
        bytes18 o = bytes18(n);
        bytes17 p = bytes17(o);
        bytes16 q = bytes16(p);
        bytes15 r = bytes15(q);
        bytes14 s = bytes14(r);
        bytes13 t = bytes13(s);
        bytes12 u = bytes12(t);
        bytes11 v = bytes11(u);
        bytes10 w = bytes10(v);
        bytes9 x = bytes9(w);
        bytes8 y = bytes8(x);
        bytes7 z = bytes7(y);
        bytes6 aa = bytes6(z);
        bytes5 ab = bytes5(aa);
        bytes4 ac = bytes4(ab);
        bytes3 ad = bytes3(ac);
        bytes2 ae = bytes2(ad);
        bytes1Value = bytes1(ae);
    }

}