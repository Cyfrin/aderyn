// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { SVGElements } from "src/libraries/SVGElements.sol";

import { NFTDescriptor_Unit_Concrete_Test } from "./NFTDescriptor.t.sol";

contract AbbreviateAmount_Unit_Concrete_Test is NFTDescriptor_Unit_Concrete_Test {
    function aa(uint256 amount, uint256 decimals) internal view returns (string memory) {
        return nftDescriptorMock.abbreviateAmount_({ amount: amount, decimals: decimals });
    }

    function ge(string memory abbreviation) internal pure returns (string memory) {
        return string.concat(SVGElements.SIGN_GE, " ", abbreviation);
    }

    function test_AbbreviateAmount_Zero() external view {
        string memory expectedAbbreviation = "0";
        assertEq(aa({ amount: 0, decimals: 0 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 0, decimals: 1 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 0, decimals: 2 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 0, decimals: 18 }), expectedAbbreviation, "abbreviation");
    }

    function test_AbbreviateAmount_Tiny() external view {
        string memory expectedAbbreviation = string.concat(SVGElements.SIGN_LT, " 1");
        assertEq(aa({ amount: 5, decimals: 1 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 9, decimals: 1 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 42, decimals: 2 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 99, decimals: 2 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 1e17 - 1, decimals: 18 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 1e18 - 1, decimals: 18 }), expectedAbbreviation, "abbreviation");
    }

    function test_AbbreviateAmount_Zillions() external view {
        string memory expectedAbbreviation = string.concat(SVGElements.SIGN_GT, " 999.99T");
        assertEq(aa({ amount: 1e15, decimals: 0 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 1e16, decimals: 1 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 1e16 + 1, decimals: 1 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 1e17, decimals: 2 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 1e17 + 1, decimals: 2 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 1e33, decimals: 18 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: 1e33 + 1, decimals: 18 }), expectedAbbreviation, "abbreviation");
        assertEq(aa({ amount: MAX_UINT128, decimals: 18 }), expectedAbbreviation, "abbreviation");
    }

    function test_AbbreviateAmount_NoSuffix() external view {
        assertEq(aa({ amount: 1, decimals: 0 }), ge("1"), "abbreviation");
        assertEq(aa({ amount: 5, decimals: 0 }), ge("5"), "abbreviation");
        assertEq(aa({ amount: 121, decimals: 1 }), ge("12"), "abbreviation");
        assertEq(aa({ amount: 1337, decimals: 2 }), ge("13"), "abbreviation");
        assertEq(aa({ amount: 78_921, decimals: 2 }), ge("789"), "abbreviation");
        assertEq(aa({ amount: 988e18, decimals: 18 }), ge("988"), "abbreviation");
    }

    function test_AbbreviateAmount_Thousands() external view {
        assertEq(aa({ amount: 1337, decimals: 0 }), ge("1.33K"), "abbreviation");
        assertEq(aa({ amount: 1080, decimals: 0 }), ge("1.08K"), "abbreviation");
        assertEq(aa({ amount: 1800, decimals: 0 }), ge("1.80K"), "abbreviation");
        assertEq(aa({ amount: 37_184, decimals: 1 }), ge("3.71K"), "abbreviation");
        assertEq(aa({ amount: 49_137, decimals: 1 }), ge("4.91K"), "abbreviation");
        assertEq(aa({ amount: 600_555, decimals: 2 }), ge("6K"), "abbreviation");
        assertEq(aa({ amount: 8211e18, decimals: 18 }), ge("8.21K"), "abbreviation");
        assertEq(aa({ amount: 201_287e18, decimals: 18 }), ge("201.28K"), "abbreviation");
    }

    function test_AbbreviateAmount_Millions() external view {
        assertEq(aa({ amount: 1_337_081, decimals: 0 }), ge("1.33M"), "abbreviation");
        assertEq(aa({ amount: 2_194_000, decimals: 0 }), ge("2.19M"), "abbreviation");
        assertEq(aa({ amount: 30_448_842, decimals: 1 }), ge("3.04M"), "abbreviation");
        assertEq(aa({ amount: 50_077_231, decimals: 1 }), ge("5M"), "abbreviation");
        assertEq(aa({ amount: 681_408_920, decimals: 2 }), ge("6.81M"), "abbreviation");
        assertEq(aa({ amount: 8_882_108e18, decimals: 18 }), ge("8.88M"), "abbreviation");
        assertEq(aa({ amount: 577_308_003e18, decimals: 18 }), ge("577.30M"), "abbreviation");
    }

    function test_AbbreviateAmount_Billions() external view {
        assertEq(aa({ amount: 1_337_081_132, decimals: 0 }), ge("1.33B"), "abbreviation");
        assertEq(aa({ amount: 2_763_455_030, decimals: 0 }), ge("2.76B"), "abbreviation");
        assertEq(aa({ amount: 30_008_011_215, decimals: 1 }), ge("3B"), "abbreviation");
        assertEq(aa({ amount: 50_450_772_867, decimals: 1 }), ge("5.04B"), "abbreviation");
        assertEq(aa({ amount: 734_730_810_730, decimals: 2 }), ge("7.34B"), "abbreviation");
        assertEq(aa({ amount: 9_927_800_422e18, decimals: 18 }), ge("9.92B"), "abbreviation");
        assertEq(aa({ amount: 699_881_672_021e18, decimals: 18 }), ge("699.88B"), "abbreviation");
    }

    function test_AbbreviateAmount_Trillions() external view {
        assertEq(aa({ amount: 2_578_924_152_034, decimals: 0 }), ge("2.57T"), "abbreviation");
        assertEq(aa({ amount: 3_931_548_209_201, decimals: 0 }), ge("3.93T"), "abbreviation");
        assertEq(aa({ amount: 60_008_233_054_613, decimals: 1 }), ge("6T"), "abbreviation");
        assertEq(aa({ amount: 61_236_342_018_965, decimals: 1 }), ge("6.12T"), "abbreviation");
        assertEq(aa({ amount: 734_730_810_730_992, decimals: 2 }), ge("7.34T"), "abbreviation");
        assertEq(aa({ amount: 9_621_312_102_753e18, decimals: 18 }), ge("9.62T"), "abbreviation");
        assertEq(aa({ amount: 101_076_479_280_188e18, decimals: 18 }), ge("101.07T"), "abbreviation");
    }
}
