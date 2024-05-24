// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { ISablierV2LockupTranched } from "@sablier/v2-core/src/interfaces/ISablierV2LockupTranched.sol";

import { ISablierV2MerkleLockup } from "./ISablierV2MerkleLockup.sol";
import { MerkleLT } from "./../types/DataTypes.sol";

/// @title ISablierV2MerkleLT
/// @notice MerkleLockup campaign that creates LockupTranched streams.
interface ISablierV2MerkleLT is ISablierV2MerkleLockup {
    /*//////////////////////////////////////////////////////////////////////////
                                 CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Retrieves the tranches with their respective unlock percentages and durations.
    function getTranchesWithPercentages() external view returns (MerkleLT.TrancheWithPercentage[] memory);

    /// @notice The address of the {SablierV2LockupTranched} contract.
    function LOCKUP_TRANCHED() external view returns (ISablierV2LockupTranched);

    /*//////////////////////////////////////////////////////////////////////////
                               NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Makes the claim by creating a LockupTranched stream to the recipient. A stream NFT is minted to the
    /// recipient.
    ///
    /// @dev Emits a {Claim} event.
    ///
    /// Requirements:
    /// - The campaign must not have expired.
    /// - The stream must not have been claimed already.
    /// - The Merkle proof must be valid.
    ///
    /// @param index The index of the recipient in the Merkle tree.
    /// @param recipient The address of the stream holder.
    /// @param amount The amount of ERC-20 assets to be distributed via the claimed stream.
    /// @param merkleProof The proof of inclusion in the Merkle tree.
    /// @return streamId The id of the newly created stream.
    function claim(
        uint256 index,
        address recipient,
        uint128 amount,
        bytes32[] calldata merkleProof
    )
        external
        returns (uint256 streamId);
}
