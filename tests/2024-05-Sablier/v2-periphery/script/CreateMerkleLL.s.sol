// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2LockupLinear } from "@sablier/v2-core/src/interfaces/ISablierV2LockupLinear.sol";
import { LockupLinear } from "@sablier/v2-core/src/types/DataTypes.sol";

import { BaseScript } from "./Base.s.sol";

import { ISablierV2MerkleLL } from "../src/interfaces/ISablierV2MerkleLL.sol";
import { ISablierV2MerkleLockupFactory } from "../src/interfaces/ISablierV2MerkleLockupFactory.sol";
import { MerkleLockup } from "../src/types/DataTypes.sol";

contract CreateMerkleLL is BaseScript {
    struct Params {
        MerkleLockup.ConstructorParams baseParams;
        ISablierV2LockupLinear lockupLinear;
        LockupLinear.Durations streamDurations;
        uint256 campaignTotalAmount;
        uint256 recipientCount;
    }

    /// @dev Deploy via Forge.
    function runBroadcast(
        ISablierV2MerkleLockupFactory merkleLockupFactory,
        Params calldata params
    )
        public
        virtual
        broadcast
        returns (ISablierV2MerkleLL merkleLL)
    {
        merkleLL = _run(merkleLockupFactory, params);
    }

    /// @dev Deploy via Sphinx.
    function runSphinx(
        ISablierV2MerkleLockupFactory merkleLockupFactory,
        Params calldata params
    )
        public
        virtual
        sphinx
        returns (ISablierV2MerkleLL merkleLL)
    {
        merkleLL = _run(merkleLockupFactory, params);
    }

    function _run(
        ISablierV2MerkleLockupFactory merkleLockupFactory,
        Params calldata params
    )
        internal
        returns (ISablierV2MerkleLL merkleLL)
    {
        merkleLL = merkleLockupFactory.createMerkleLL(
            params.baseParams,
            params.lockupLinear,
            params.streamDurations,
            params.campaignTotalAmount,
            params.recipientCount
        );
    }
}
