// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { ISablierV2LockupLinear } from "@sablier/v2-core/src/interfaces/ISablierV2LockupLinear.sol";

import { ISablierV2MerkleLockup } from "./ISablierV2MerkleLockup.sol";

/// @title ISablierV2MerkleLL
/// @notice MerkleLockup campaign that creates LockupLinear streams.
interface ISablierV2MerkleLL is ISablierV2MerkleLockup {
    /*//////////////////////////////////////////////////////////////////////////
                                 CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice The address of the {SablierV2LockupLinear} contract.
    function LOCKUP_LINEAR() external view returns (ISablierV2LockupLinear);

    /// @notice The total streaming duration of each stream.
    function streamDurations() external view returns (uint40 cliff, uint40 duration);

    /*//////////////////////////////////////////////////////////////////////////
                               NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Makes the claim by creating a LockupLinear stream to the recipient. A stream NFT is minted to the
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
