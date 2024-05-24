// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { BaseScript } from "./Base.s.sol";

import { SablierV2MerkleLockupFactory } from "../src/SablierV2MerkleLockupFactory.sol";

contract DeployMerkleLockupFactory is BaseScript {
    /// @dev Deploy via Forge.
    function runBroadcast() public virtual broadcast returns (SablierV2MerkleLockupFactory merkleLockupFactory) {
        merkleLockupFactory = _run();
    }

    /// @dev Deploy via Sphinx.
    function runSphinx() public virtual sphinx returns (SablierV2MerkleLockupFactory merkleLockupFactory) {
        merkleLockupFactory = _run();
    }

    function _run() internal returns (SablierV2MerkleLockupFactory merkleLockupFactory) {
        merkleLockupFactory = new SablierV2MerkleLockupFactory();
    }
}
