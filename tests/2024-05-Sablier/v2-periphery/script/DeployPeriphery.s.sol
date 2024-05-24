// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { BaseScript } from "./Base.s.sol";

import { SablierV2MerkleLockupFactory } from "../src/SablierV2MerkleLockupFactory.sol";
import { SablierV2BatchLockup } from "../src/SablierV2BatchLockup.sol";

/// @notice Deploys all V2 Periphery contract in the following order:
///
/// 1. {SablierV2BatchLockup}
/// 2. {SablierV2MerkleLockupFactory}
contract DeployPeriphery is BaseScript {
    /// @dev Deploy via Forge.
    function runBroadcast()
        public
        virtual
        broadcast
        returns (SablierV2BatchLockup batchLockup, SablierV2MerkleLockupFactory merkleLockupFactory)
    {
        (batchLockup, merkleLockupFactory) = _run();
    }

    /// @dev Deploy via Sphinx.
    function runSphinx()
        public
        virtual
        sphinx
        returns (SablierV2BatchLockup batchLockup, SablierV2MerkleLockupFactory merkleLockupFactory)
    {
        (batchLockup, merkleLockupFactory) = _run();
    }

    function _run()
        internal
        returns (SablierV2BatchLockup batchLockup, SablierV2MerkleLockupFactory merkleLockupFactory)
    {
        batchLockup = new SablierV2BatchLockup();
        merkleLockupFactory = new SablierV2MerkleLockupFactory();
    }
}
