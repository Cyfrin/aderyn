// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2NFTDescriptor } from "../src/interfaces/ISablierV2NFTDescriptor.sol";
import { SablierV2LockupDynamic } from "../src/SablierV2LockupDynamic.sol";
import { SablierV2LockupLinear } from "../src/SablierV2LockupLinear.sol";
import { SablierV2LockupTranched } from "../src/SablierV2LockupTranched.sol";

import { BaseScript } from "./Base.s.sol";

/// @notice Deploys these contracts at deterministic addresses across chains, in the following order:
///
/// 1. {SablierV2LockupDynamic}
/// 2. {SablierV2LockupLinear}
/// 3. {SablierV2LockupTranched}
///
/// @dev Reverts if any contract has already been deployed.
contract DeployDeterministicCore2 is BaseScript {
    /// @dev Deploy via Forge.
    function runBroadcast(
        address initialAdmin,
        ISablierV2NFTDescriptor nftDescriptor
    )
        public
        virtual
        broadcast
        returns (
            SablierV2LockupDynamic lockupDynamic,
            SablierV2LockupLinear lockupLinear,
            SablierV2LockupTranched lockupTranched
        )
    {
        (lockupDynamic, lockupLinear, lockupTranched) = _run(initialAdmin, nftDescriptor);
    }

    /// @dev Deploy via Sphinx.
    function runSphinx(
        address initialAdmin,
        ISablierV2NFTDescriptor nftDescriptor
    )
        public
        virtual
        sphinx
        returns (
            SablierV2LockupDynamic lockupDynamic,
            SablierV2LockupLinear lockupLinear,
            SablierV2LockupTranched lockupTranched
        )
    {
        (lockupDynamic, lockupLinear, lockupTranched) = _run(initialAdmin, nftDescriptor);
    }

    function _run(
        address initialAdmin,
        ISablierV2NFTDescriptor nftDescriptor
    )
        internal
        returns (
            SablierV2LockupDynamic lockupDynamic,
            SablierV2LockupLinear lockupLinear,
            SablierV2LockupTranched lockupTranched
        )
    {
        bytes32 salt = constructCreate2Salt();
        lockupDynamic = new SablierV2LockupDynamic{ salt: salt }(initialAdmin, nftDescriptor, maxSegmentCount);
        lockupLinear = new SablierV2LockupLinear{ salt: salt }(initialAdmin, nftDescriptor);
        lockupTranched = new SablierV2LockupTranched{ salt: salt }(initialAdmin, nftDescriptor, maxTrancheCount);
    }
}
