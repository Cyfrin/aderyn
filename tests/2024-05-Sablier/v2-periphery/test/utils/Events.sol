// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { LockupLinear } from "@sablier/v2-core/src/types/DataTypes.sol";
import { ISablierV2LockupLinear } from "@sablier/v2-core/src/interfaces/ISablierV2LockupLinear.sol";
import { ISablierV2LockupTranched } from "@sablier/v2-core/src/interfaces/ISablierV2LockupTranched.sol";

import { ISablierV2MerkleLL } from "src/interfaces/ISablierV2MerkleLL.sol";
import { ISablierV2MerkleLT } from "src/interfaces/ISablierV2MerkleLT.sol";
import { MerkleLockup, MerkleLT } from "src/types/DataTypes.sol";

/// @notice Abstract contract containing all the events emitted by the protocol.
abstract contract Events {
    event Claim(uint256 index, address indexed recipient, uint128 amount, uint256 indexed streamId);
    event Clawback(address indexed admin, address indexed to, uint128 amount);
    event CreateMerkleLL(
        ISablierV2MerkleLL indexed merkleLL,
        MerkleLockup.ConstructorParams baseParams,
        ISablierV2LockupLinear lockupLinear,
        LockupLinear.Durations streamDurations,
        uint256 aggregateAmount,
        uint256 recipientCount
    );
    event CreateMerkleLT(
        ISablierV2MerkleLT indexed merkleLT,
        MerkleLockup.ConstructorParams baseParams,
        ISablierV2LockupTranched lockupTranched,
        MerkleLT.TrancheWithPercentage[] tranchesWithPercentages,
        uint256 totalDuration,
        uint256 aggregateAmount,
        uint256 recipientCount
    );
}
