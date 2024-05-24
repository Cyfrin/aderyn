// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { ISablierV2LockupLinear } from "@sablier/v2-core/src/interfaces/ISablierV2LockupLinear.sol";
import { ISablierV2LockupTranched } from "@sablier/v2-core/src/interfaces/ISablierV2LockupTranched.sol";
import { LockupLinear } from "@sablier/v2-core/src/types/DataTypes.sol";

import { ISablierV2MerkleLL } from "./ISablierV2MerkleLL.sol";
import { ISablierV2MerkleLT } from "./ISablierV2MerkleLT.sol";
import { MerkleLockup, MerkleLT } from "../types/DataTypes.sol";

/// @title ISablierV2MerkleLockupFactory
/// @notice Deploys MerkleLockup campaigns with CREATE.
interface ISablierV2MerkleLockupFactory {
    /*//////////////////////////////////////////////////////////////////////////
                                       EVENTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a {SablierV2MerkleLL} campaign is created.
    event CreateMerkleLL(
        ISablierV2MerkleLL indexed merkleLL,
        MerkleLockup.ConstructorParams baseParams,
        ISablierV2LockupLinear lockupLinear,
        LockupLinear.Durations streamDurations,
        uint256 aggregateAmount,
        uint256 recipientCount
    );

    /// @notice Emitted when a {SablierV2MerkleLT} campaign is created.
    event CreateMerkleLT(
        ISablierV2MerkleLT indexed merkleLT,
        MerkleLockup.ConstructorParams baseParams,
        ISablierV2LockupTranched lockupTranched,
        MerkleLT.TrancheWithPercentage[] tranchesWithPercentages,
        uint256 totalDuration,
        uint256 aggregateAmount,
        uint256 recipientCount
    );

    /*//////////////////////////////////////////////////////////////////////////
                               NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Creates a new MerkleLockup campaign with a LockupLinear distribution.
    /// @dev Emits a {CreateMerkleLL} event.
    /// @param baseParams Struct encapsulating the {SablierV2MerkleLockup} parameters, which are documented in
    /// {DataTypes}.
    /// @param lockupLinear The address of the {SablierV2LockupLinear} contract.
    /// @param streamDurations The durations for each stream.
    /// @param aggregateAmount The total amount of ERC-20 assets to be distributed to all recipients.
    /// @param recipientCount The total number of recipients who are eligible to claim.
    /// @return merkleLL The address of the newly created MerkleLockup contract.
    function createMerkleLL(
        MerkleLockup.ConstructorParams memory baseParams,
        ISablierV2LockupLinear lockupLinear,
        LockupLinear.Durations memory streamDurations,
        uint256 aggregateAmount,
        uint256 recipientCount
    )
        external
        returns (ISablierV2MerkleLL merkleLL);

    /// @notice Creates a new MerkleLockup campaign with a LockupTranched distribution.
    /// @dev Emits a {CreateMerkleLT} event.
    ///
    /// Requirements:
    /// - The sum of the tranches' unlock percentages must equal 100% = 1e18.
    ///
    /// @param baseParams Struct encapsulating the {SablierV2MerkleLockup} parameters, which are documented in
    /// {DataTypes}.
    /// @param lockupTranched The address of the {SablierV2LockupTranched} contract.
    /// @param tranchesWithPercentages The tranches with their respective unlock percentages.
    /// @param aggregateAmount The total amount of ERC-20 assets to be distributed to all recipients.
    /// @param recipientCount The total number of recipients who are eligible to claim.
    /// @return merkleLT The address of the newly created MerkleLockup contract.
    function createMerkleLT(
        MerkleLockup.ConstructorParams memory baseParams,
        ISablierV2LockupTranched lockupTranched,
        MerkleLT.TrancheWithPercentage[] memory tranchesWithPercentages,
        uint256 aggregateAmount,
        uint256 recipientCount
    )
        external
        returns (ISablierV2MerkleLT merkleLT);
}
