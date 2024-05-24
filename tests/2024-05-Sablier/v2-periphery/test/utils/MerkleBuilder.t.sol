// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { StdAssertions } from "forge-std/src/StdAssertions.sol";
import { StdUtils } from "forge-std/src/StdUtils.sol";

import { MerkleBuilder } from "./MerkleBuilder.sol";

contract MerkleBuilder_Test is StdAssertions, StdUtils {
    function testFuzz_ComputeLeaf(uint256 index, address recipient, uint128 amount) external pure {
        uint256 actualLeaf = MerkleBuilder.computeLeaf(index, recipient, amount);
        uint256 expectedLeaf = uint256(keccak256(bytes.concat(keccak256(abi.encode(index, recipient, amount)))));
        assertEq(actualLeaf, expectedLeaf, "computeLeaf");
    }

    /// @dev We declare this struct so that we will not need cheatcodes in the `computeLeaves` test.
    struct LeavesParams {
        uint256 indexes;
        address recipients;
        uint128 amounts;
    }

    function testFuzz_ComputeLeaves(LeavesParams[] memory params) external pure {
        uint256 count = params.length;

        uint256[] memory indexes = new uint256[](count);
        address[] memory recipients = new address[](count);
        uint128[] memory amounts = new uint128[](count);
        for (uint256 i = 0; i < count; ++i) {
            indexes[i] = params[i].indexes;
            recipients[i] = params[i].recipients;
            amounts[i] = params[i].amounts;
        }

        uint256[] memory actualLeaves = new uint256[](count);
        actualLeaves = MerkleBuilder.computeLeaves(indexes, recipients, amounts);

        uint256[] memory expectedLeaves = new uint256[](count);
        for (uint256 i = 0; i < count; ++i) {
            expectedLeaves[i] =
                uint256(keccak256(bytes.concat(keccak256(abi.encode(indexes[i], recipients[i], amounts[i])))));
        }

        assertEq(actualLeaves, expectedLeaves, "computeLeaves");
    }
}
