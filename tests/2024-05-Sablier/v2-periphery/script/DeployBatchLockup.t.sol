// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { BaseScript } from "./Base.s.sol";

import { SablierV2BatchLockup } from "../src/SablierV2BatchLockup.sol";

contract DeployBatchLockup is BaseScript {
    /// @dev Deploy via Forge.
    function runBroadcast() public virtual broadcast returns (SablierV2BatchLockup batchLockup) {
        batchLockup = _run();
    }

    /// @dev Deploy via Sphinx.
    function runSphinx() public virtual sphinx returns (SablierV2BatchLockup batchLockup) {
        batchLockup = _run();
    }

    function _run() internal returns (SablierV2BatchLockup batchLockup) {
        batchLockup = new SablierV2BatchLockup();
    }
}
