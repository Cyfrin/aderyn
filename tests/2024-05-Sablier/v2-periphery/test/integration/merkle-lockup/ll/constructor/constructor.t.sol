// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { LockupLinear } from "@sablier/v2-core/src/types/DataTypes.sol";

import { SablierV2MerkleLL } from "src/SablierV2MerkleLL.sol";

import { MerkleLockup_Integration_Test } from "../../MerkleLockup.t.sol";

contract Constructor_MerkleLL_Integration_Test is MerkleLockup_Integration_Test {
    /// @dev Needed to prevent "Stack too deep" error
    struct Vars {
        address actualAdmin;
        uint256 actualAllowance;
        address actualAsset;
        string actualIpfsCID;
        string actualName;
        bool actualCancelable;
        LockupLinear.Durations actualDurations;
        uint40 actualExpiration;
        address actualLockupLinear;
        bytes32 actualMerkleRoot;
        bool actualTransferable;
        address expectedAdmin;
        uint256 expectedAllowance;
        address expectedAsset;
        bool expectedCancelable;
        LockupLinear.Durations expectedDurations;
        uint40 expectedExpiration;
        string expectedIpfsCID;
        address expectedLockupLinear;
        bytes32 expectedMerkleRoot;
        bytes32 expectedName;
        bool expectedTransferable;
    }

    function test_Constructor() external {
        SablierV2MerkleLL constructedLL =
            new SablierV2MerkleLL(defaults.baseParams(), lockupLinear, defaults.durations());

        Vars memory vars;

        vars.actualAdmin = constructedLL.admin();
        vars.expectedAdmin = users.admin;
        assertEq(vars.actualAdmin, vars.expectedAdmin, "admin");

        vars.actualAllowance = dai.allowance(address(constructedLL), address(lockupLinear));
        vars.expectedAllowance = MAX_UINT256;
        assertEq(vars.actualAllowance, vars.expectedAllowance, "allowance");

        vars.actualAsset = address(constructedLL.ASSET());
        vars.expectedAsset = address(dai);
        assertEq(vars.actualAsset, vars.expectedAsset, "asset");

        vars.actualCancelable = constructedLL.CANCELABLE();
        vars.expectedCancelable = defaults.CANCELABLE();
        assertEq(vars.actualCancelable, vars.expectedCancelable, "cancelable");

        (vars.actualDurations.cliff, vars.actualDurations.total) = constructedLL.streamDurations();
        vars.expectedDurations = defaults.durations();
        assertEq(vars.actualDurations.cliff, vars.expectedDurations.cliff, "durations.cliff");
        assertEq(vars.actualDurations.total, vars.expectedDurations.total, "durations.total");

        vars.actualExpiration = constructedLL.EXPIRATION();
        vars.expectedExpiration = defaults.EXPIRATION();
        assertEq(vars.actualExpiration, vars.expectedExpiration, "expiration");

        vars.actualIpfsCID = constructedLL.ipfsCID();
        vars.expectedIpfsCID = defaults.IPFS_CID();
        assertEq(vars.actualIpfsCID, vars.expectedIpfsCID, "ipfsCID");

        vars.actualLockupLinear = address(constructedLL.LOCKUP_LINEAR());
        vars.expectedLockupLinear = address(lockupLinear);
        assertEq(vars.actualLockupLinear, vars.expectedLockupLinear, "lockupLinear");

        vars.actualMerkleRoot = constructedLL.MERKLE_ROOT();
        vars.expectedMerkleRoot = defaults.MERKLE_ROOT();
        assertEq(vars.actualMerkleRoot, vars.expectedMerkleRoot, "merkleRoot");

        vars.actualName = constructedLL.name();
        vars.expectedName = defaults.NAME_BYTES32();
        assertEq(bytes32(abi.encodePacked(vars.actualName)), vars.expectedName, "name");

        vars.actualTransferable = constructedLL.TRANSFERABLE();
        vars.expectedTransferable = defaults.TRANSFERABLE();
        assertEq(vars.actualTransferable, vars.expectedTransferable, "transferable");
    }
}
