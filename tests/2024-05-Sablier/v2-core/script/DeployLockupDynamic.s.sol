// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2NFTDescriptor } from "../src/interfaces/ISablierV2NFTDescriptor.sol";
import { SablierV2LockupDynamic } from "../src/SablierV2LockupDynamic.sol";

import { BaseScript } from "./Base.s.sol";

contract DeployLockupDynamic is BaseScript {
    /// @dev Deploy via Forge.
    function runBroadcast(
        address initialAdmin,
        ISablierV2NFTDescriptor initialNFTDescriptor
    )
        public
        virtual
        broadcast
        returns (SablierV2LockupDynamic lockupDynamic)
    {
        lockupDynamic = _run(initialAdmin, initialNFTDescriptor);
    }

    /// @dev Deploy via Sphinx.
    function runSphinx(
        address initialAdmin,
        ISablierV2NFTDescriptor initialNFTDescriptor
    )
        public
        virtual
        sphinx
        returns (SablierV2LockupDynamic lockupDynamic)
    {
        lockupDynamic = _run(initialAdmin, initialNFTDescriptor);
    }

    function _run(
        address initialAdmin,
        ISablierV2NFTDescriptor initialNFTDescriptor
    )
        internal
        returns (SablierV2LockupDynamic lockupDynamic)
    {
        lockupDynamic = new SablierV2LockupDynamic(initialAdmin, initialNFTDescriptor, maxSegmentCount);
    }
}
