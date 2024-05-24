// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { SablierV2NFTDescriptor } from "../src/SablierV2NFTDescriptor.sol";

import { BaseScript } from "./Base.s.sol";

contract DeployNFTDescriptor is BaseScript {
    /// @dev Deploy via Forge.
    function runBroadcast() public virtual broadcast returns (SablierV2NFTDescriptor nftDescriptor) {
        nftDescriptor = _run();
    }

    /// @dev Deploy via Sphinx.
    function runSphinx() public virtual sphinx returns (SablierV2NFTDescriptor nftDescriptor) {
        nftDescriptor = _run();
    }

    function _run() internal returns (SablierV2NFTDescriptor nftDescriptor) {
        nftDescriptor = new SablierV2NFTDescriptor();
    }
}
