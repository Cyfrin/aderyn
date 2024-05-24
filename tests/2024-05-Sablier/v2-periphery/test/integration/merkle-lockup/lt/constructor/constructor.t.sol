// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { SablierV2MerkleLT } from "src/SablierV2MerkleLT.sol";
import { MerkleLT } from "src/types/DataTypes.sol";

import { MerkleLockup_Integration_Test } from "../../MerkleLockup.t.sol";

contract Constructor_MerkleLT_Integration_Test is MerkleLockup_Integration_Test {
    /// @dev Needed to prevent "Stack too deep" error
    struct Vars {
        address actualAdmin;
        uint256 actualAllowance;
        address actualAsset;
        string actualIpfsCID;
        string actualName;
        bool actualCancelable;
        uint40 actualExpiration;
        address actualLockupTranched;
        bytes32 actualMerkleRoot;
        MerkleLT.TrancheWithPercentage[] actualTranchesWithPercentages;
        bool actualTransferable;
        address expectedAdmin;
        uint256 expectedAllowance;
        address expectedAsset;
        bool expectedCancelable;
        string expectedIpfsCID;
        uint40 expectedExpiration;
        address expectedLockupTranched;
        bytes32 expectedMerkleRoot;
        bytes32 expectedName;
        MerkleLT.TrancheWithPercentage[] expectedTranchesWithPercentages;
        bool expectedTransferable;
    }

    function test_Constructor() external {
        SablierV2MerkleLT constructedLT =
            new SablierV2MerkleLT(defaults.baseParams(), lockupTranched, defaults.tranchesWithPercentages());

        Vars memory vars;

        vars.actualAdmin = constructedLT.admin();
        vars.expectedAdmin = users.admin;
        assertEq(vars.actualAdmin, vars.expectedAdmin, "admin");

        vars.actualAllowance = dai.allowance(address(constructedLT), address(lockupTranched));
        vars.expectedAllowance = MAX_UINT256;
        assertEq(vars.actualAllowance, vars.expectedAllowance, "allowance");

        vars.actualAsset = address(constructedLT.ASSET());
        vars.expectedAsset = address(dai);
        assertEq(vars.actualAsset, vars.expectedAsset, "asset");

        vars.actualCancelable = constructedLT.CANCELABLE();
        vars.expectedCancelable = defaults.CANCELABLE();
        assertEq(vars.actualCancelable, vars.expectedCancelable, "cancelable");

        vars.actualExpiration = constructedLT.EXPIRATION();
        vars.expectedExpiration = defaults.EXPIRATION();
        assertEq(vars.actualExpiration, vars.expectedExpiration, "expiration");

        vars.actualIpfsCID = constructedLT.ipfsCID();
        vars.expectedIpfsCID = defaults.IPFS_CID();
        assertEq(vars.actualIpfsCID, vars.expectedIpfsCID, "ipfsCID");

        vars.actualLockupTranched = address(constructedLT.LOCKUP_TRANCHED());
        vars.expectedLockupTranched = address(lockupTranched);
        assertEq(vars.actualLockupTranched, vars.expectedLockupTranched, "lockupTranched");

        vars.actualName = constructedLT.name();
        vars.expectedName = defaults.NAME_BYTES32();
        assertEq(bytes32(abi.encodePacked(vars.actualName)), vars.expectedName, "name");

        vars.actualMerkleRoot = constructedLT.MERKLE_ROOT();
        vars.expectedMerkleRoot = defaults.MERKLE_ROOT();
        assertEq(vars.actualMerkleRoot, vars.expectedMerkleRoot, "merkleRoot");

        vars.actualTranchesWithPercentages = constructedLT.getTranchesWithPercentages();
        vars.expectedTranchesWithPercentages = defaults.tranchesWithPercentages();
        assertEq(vars.actualTranchesWithPercentages, vars.expectedTranchesWithPercentages, "tranchesWithPercentages");

        vars.actualTransferable = constructedLT.TRANSFERABLE();
        vars.expectedTransferable = defaults.TRANSFERABLE();
        assertEq(vars.actualTransferable, vars.expectedTransferable, "transferable");
    }
}
