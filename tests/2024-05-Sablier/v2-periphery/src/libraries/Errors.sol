// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

/// @title Errors
/// @notice Library containing all custom errors the protocol may revert with.
library Errors {
    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-BATCH-LOCKUP
    //////////////////////////////////////////////////////////////////////////*/

    error SablierV2BatchLockup_BatchSizeZero();

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-MERKLE-LOCKUP
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Thrown when trying to claim after the campaign has expired.
    error SablierV2MerkleLockup_CampaignExpired(uint256 blockTimestamp, uint40 expiration);

    /// @notice Thrown when trying to create a campaign with a name that is too long.
    error SablierV2MerkleLockup_CampaignNameTooLong(uint256 nameLength, uint256 maxLength);

    /// @notice Thrown when trying to clawback when the current timestamp is over the grace period and the campaign has
    /// not expired.
    error SablierV2MerkleLockup_ClawbackNotAllowed(uint256 blockTimestamp, uint40 expiration, uint40 firstClaimTime);

    /// @notice Thrown when trying to claim with an invalid Merkle proof.
    error SablierV2MerkleLockup_InvalidProof();

    /// @notice Thrown when trying to claim the same stream more than once.
    error SablierV2MerkleLockup_StreamClaimed(uint256 index);

    /*//////////////////////////////////////////////////////////////////////////
                          SABLIER-V2-MERKLE-LOCKUP-FACTORY
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Thrown when the sum of the tranches' unlock percentages does not equal 100%.
    error SablierV2MerkleLockupFactory_TotalPercentageNotOneHundred(uint64 totalPercentage);
}
