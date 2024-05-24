// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { SablierV2LockupDynamic } from "@sablier/v2-core/src/SablierV2LockupDynamic.sol";
import { SablierV2LockupLinear } from "@sablier/v2-core/src/SablierV2LockupLinear.sol";
import { SablierV2LockupTranched } from "@sablier/v2-core/src/SablierV2LockupTranched.sol";
import { SablierV2NFTDescriptor } from "@sablier/v2-core/src/SablierV2NFTDescriptor.sol";
import { BaseScript } from "./Base.s.sol";

import { SablierV2MerkleLockupFactory } from "../src/SablierV2MerkleLockupFactory.sol";
import { SablierV2BatchLockup } from "../src/SablierV2BatchLockup.sol";

/// @notice Deploys the Sablier V2 Protocol.
contract DeployProtocol is BaseScript {
    /// @dev Deploy via Forge.
    function runBroadcast(
        address initialAdmin,
        uint256 maxSegmentCount,
        uint256 maxTrancheCount
    )
        public
        virtual
        broadcast
        returns (
            SablierV2LockupDynamic lockupDynamic,
            SablierV2LockupLinear lockupLinear,
            SablierV2LockupTranched lockupTranched,
            SablierV2NFTDescriptor nftDescriptor,
            SablierV2BatchLockup batchLockup,
            SablierV2MerkleLockupFactory merkleLockupFactory
        )
    {
        (lockupDynamic, lockupLinear, lockupTranched, nftDescriptor, batchLockup, merkleLockupFactory) =
            _run(initialAdmin, maxSegmentCount, maxTrancheCount);
    }

    /// @dev Deploy via Sphinx.
    function runSphinx(
        address initialAdmin,
        uint256 maxSegmentCount,
        uint256 maxTrancheCount
    )
        public
        virtual
        sphinx
        returns (
            SablierV2LockupDynamic lockupDynamic,
            SablierV2LockupLinear lockupLinear,
            SablierV2LockupTranched lockupTranched,
            SablierV2NFTDescriptor nftDescriptor,
            SablierV2BatchLockup batchLockup,
            SablierV2MerkleLockupFactory merkleLockupFactory
        )
    {
        (lockupDynamic, lockupLinear, lockupTranched, nftDescriptor, batchLockup, merkleLockupFactory) =
            _run(initialAdmin, maxSegmentCount, maxTrancheCount);
    }

    function _run(
        address initialAdmin,
        uint256 maxSegmentCount,
        uint256 maxTrancheCount
    )
        internal
        returns (
            SablierV2LockupDynamic lockupDynamic,
            SablierV2LockupLinear lockupLinear,
            SablierV2LockupTranched lockupTranched,
            SablierV2NFTDescriptor nftDescriptor,
            SablierV2BatchLockup batchLockup,
            SablierV2MerkleLockupFactory merkleLockupFactory
        )
    {
        // Deploy V2 Core.
        nftDescriptor = new SablierV2NFTDescriptor();
        lockupDynamic = new SablierV2LockupDynamic(initialAdmin, nftDescriptor, maxSegmentCount);
        lockupLinear = new SablierV2LockupLinear(initialAdmin, nftDescriptor);
        lockupTranched = new SablierV2LockupTranched(initialAdmin, nftDescriptor, maxTrancheCount);

        // Deploy V2 Periphery.
        batchLockup = new SablierV2BatchLockup();
        merkleLockupFactory = new SablierV2MerkleLockupFactory();
    }
}
