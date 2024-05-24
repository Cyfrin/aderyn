// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { BaseScript } from "./Base.s.sol";

import { SablierV2BatchLockup } from "../src/SablierV2BatchLockup.sol";

/// @notice Deploys {SablierV2BatchLockup} at a deterministic address across chains.
/// @dev Reverts if the contract has already been deployed.
contract DeployDeterministicBatchLockup is BaseScript {
    /// @dev Deploy via Forge.
    function runBroadcast() public virtual broadcast returns (SablierV2BatchLockup batchLockup) {
        batchLockup = _run();
    }

    /// @dev Deploy via Sphinx.
    function runSphinx() public virtual sphinx returns (SablierV2BatchLockup batchLockup) {
        batchLockup = _run();
    }

    function _run() internal returns (SablierV2BatchLockup batchLockup) {
        bytes32 salt = constructCreate2Salt();
        batchLockup = new SablierV2BatchLockup{ salt: salt }();
    }
}
