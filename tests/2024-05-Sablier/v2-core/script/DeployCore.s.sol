// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { SablierV2LockupDynamic } from "../src/SablierV2LockupDynamic.sol";
import { SablierV2LockupLinear } from "../src/SablierV2LockupLinear.sol";
import { SablierV2LockupTranched } from "../src/SablierV2LockupTranched.sol";
import { SablierV2NFTDescriptor } from "../src/SablierV2NFTDescriptor.sol";

import { BaseScript } from "./Base.s.sol";

/// @notice Deploys all V2 Core contract in the following order:
///
/// 1. {SablierV2NFTDescriptor}
/// 2. {SablierV2LockupDynamic}
/// 3. {SablierV2LockupLinear}
/// 4. {SablierV2LockupTranched}
contract DeployCore is BaseScript {
    /// @dev Deploy via Forge.
    function runBroadcast(address initialAdmin)
        public
        virtual
        broadcast
        returns (
            SablierV2LockupDynamic lockupDynamic,
            SablierV2LockupLinear lockupLinear,
            SablierV2LockupTranched lockupTranched,
            SablierV2NFTDescriptor nftDescriptor
        )
    {
        (lockupDynamic, lockupLinear, lockupTranched, nftDescriptor) = _run(initialAdmin);
    }

    /// @dev Deploy via Sphinx.
    function runSphinx(address initialAdmin)
        public
        virtual
        sphinx
        returns (
            SablierV2LockupDynamic lockupDynamic,
            SablierV2LockupLinear lockupLinear,
            SablierV2LockupTranched lockupTranched,
            SablierV2NFTDescriptor nftDescriptor
        )
    {
        (lockupDynamic, lockupLinear, lockupTranched, nftDescriptor) = _run(initialAdmin);
    }

    function _run(address initialAdmin)
        internal
        returns (
            SablierV2LockupDynamic lockupDynamic,
            SablierV2LockupLinear lockupLinear,
            SablierV2LockupTranched lockupTranched,
            SablierV2NFTDescriptor nftDescriptor
        )
    {
        nftDescriptor = new SablierV2NFTDescriptor();
        lockupDynamic = new SablierV2LockupDynamic(initialAdmin, nftDescriptor, maxSegmentCount);
        lockupLinear = new SablierV2LockupLinear(initialAdmin, nftDescriptor);
        lockupTranched = new SablierV2LockupTranched(initialAdmin, nftDescriptor, maxTrancheCount);
    }
}
