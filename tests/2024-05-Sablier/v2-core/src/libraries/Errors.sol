// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IERC721Metadata } from "@openzeppelin/contracts/token/ERC721/extensions/IERC721Metadata.sol";
import { UD60x18 } from "@prb/math/src/UD60x18.sol";

/// @title Errors
/// @notice Library containing all custom errors the protocol may revert with.
library Errors {
    /*//////////////////////////////////////////////////////////////////////////
                                      GENERICS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Thrown when `msg.sender` is not the admin.
    error CallerNotAdmin(address admin, address caller);

    /// @notice Thrown when trying to delegate call to a function that disallows delegate calls.
    error DelegateCall();

    /*//////////////////////////////////////////////////////////////////////////
                                 SABLIER-V2-LOCKUP
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Thrown when the broker fee exceeds the maximum allowed fee.
    error SablierV2Lockup_BrokerFeeTooHigh(UD60x18 brokerFee, UD60x18 maxBrokerFee);

    /// @notice Thrown when trying to create a stream with a zero deposit amount.
    error SablierV2Lockup_DepositAmountZero();

    /// @notice Thrown when trying to create a stream with an end time not in the future.
    error SablierV2Lockup_EndTimeNotInTheFuture(uint40 blockTimestamp, uint40 endTime);

    /// @notice Thrown when trying to transfer Stream NFT when transferability is disabled.
    error SablierV2Lockup_NotTransferable(uint256 tokenId);

    /// @notice Thrown when the ID references a null stream.
    error SablierV2Lockup_Null(uint256 streamId);

    /// @notice Thrown when trying to withdraw an amount greater than the withdrawable amount.
    error SablierV2Lockup_Overdraw(uint256 streamId, uint128 amount, uint128 withdrawableAmount);

    /// @notice Thrown when trying to create a stream with a zero start time.
    error SablierV2Lockup_StartTimeZero();

    /// @notice Thrown when trying to cancel or renounce a canceled stream.
    error SablierV2Lockup_StreamCanceled(uint256 streamId);

    /// @notice Thrown when trying to cancel, renounce, or withdraw from a depleted stream.
    error SablierV2Lockup_StreamDepleted(uint256 streamId);

    /// @notice Thrown when trying to cancel or renounce a stream that is not cancelable.
    error SablierV2Lockup_StreamNotCancelable(uint256 streamId);

    /// @notice Thrown when trying to burn a stream that is not depleted.
    error SablierV2Lockup_StreamNotDepleted(uint256 streamId);

    /// @notice Thrown when trying to cancel or renounce a settled stream.
    error SablierV2Lockup_StreamSettled(uint256 streamId);

    /// @notice Thrown when `msg.sender` lacks authorization to perform an action.
    error SablierV2Lockup_Unauthorized(uint256 streamId, address caller);

    /// @notice Thrown when trying to withdraw to an address other than the recipient's.
    error SablierV2Lockup_WithdrawalAddressNotRecipient(uint256 streamId, address caller, address to);

    /// @notice Thrown when trying to withdraw zero assets from a stream.
    error SablierV2Lockup_WithdrawAmountZero(uint256 streamId);

    /// @notice Thrown when trying to withdraw from multiple streams and the number of stream IDs does
    /// not match the number of withdraw amounts.
    error SablierV2Lockup_WithdrawArrayCountsNotEqual(uint256 streamIdsCount, uint256 amountsCount);

    /// @notice Thrown when trying to withdraw to the zero address.
    error SablierV2Lockup_WithdrawToZeroAddress(uint256 streamId);

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-LOCKUP-DYNAMIC
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Thrown when trying to create a stream with a deposit amount not equal to the sum of the
    /// segment amounts.
    error SablierV2LockupDynamic_DepositAmountNotEqualToSegmentAmountsSum(
        uint128 depositAmount, uint128 segmentAmountsSum
    );

    /// @notice Thrown when trying to create a stream with more segments than the maximum allowed.
    error SablierV2LockupDynamic_SegmentCountTooHigh(uint256 count);

    /// @notice Thrown when trying to create a stream with no segments.
    error SablierV2LockupDynamic_SegmentCountZero();

    /// @notice Thrown when trying to create a stream with unordered segment timestamps.
    error SablierV2LockupDynamic_SegmentTimestampsNotOrdered(
        uint256 index, uint40 previousTimestamp, uint40 currentTimestamp
    );

    /// @notice Thrown when trying to create a stream with a start time not strictly less than the first
    /// segment timestamp.
    error SablierV2LockupDynamic_StartTimeNotLessThanFirstSegmentTimestamp(
        uint40 startTime, uint40 firstSegmentTimestamp
    );

    /*//////////////////////////////////////////////////////////////////////////
                              SABLIER-V2-LOCKUP-LINEAR
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Thrown when trying to create a stream with a cliff time not strictly less than the end time.
    error SablierV2LockupLinear_CliffTimeNotLessThanEndTime(uint40 cliffTime, uint40 endTime);

    /// @notice Thrown when trying to create a stream with a start time not strictly less than the cliff time, when the
    /// cliff time does not have a zero value.
    error SablierV2LockupLinear_StartTimeNotLessThanCliffTime(uint40 startTime, uint40 cliffTime);

    /// @notice Thrown when trying to create a stream with a start time not strictly less than the end time.
    error SablierV2LockupLinear_StartTimeNotLessThanEndTime(uint40 startTime, uint40 endTime);

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-NFT-DESCRIPTOR
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Thrown when trying to generate the token URI for an unknown ERC-721 NFT contract.
    error SablierV2NFTDescriptor_UnknownNFT(IERC721Metadata nft, string symbol);

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-LOCKUP-TRANCHE
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Thrown when trying to create a stream with a deposit amount not equal to the sum of the
    /// tranche amounts.
    error SablierV2LockupTranched_DepositAmountNotEqualToTrancheAmountsSum(
        uint128 depositAmount, uint128 trancheAmountsSum
    );

    /// @notice Thrown when trying to create a stream with a start time not strictly less than the first
    /// tranche timestamp.
    error SablierV2LockupTranched_StartTimeNotLessThanFirstTrancheTimestamp(
        uint40 startTime, uint40 firstTrancheTimestamp
    );

    /// @notice Thrown when trying to create a stream with more tranches than the maximum allowed.
    error SablierV2LockupTranched_TrancheCountTooHigh(uint256 count);

    /// @notice Thrown when trying to create a stream with no tranches.
    error SablierV2LockupTranched_TrancheCountZero();

    /// @notice Thrown when trying to create a stream with unordered tranche timestamps.
    error SablierV2LockupTranched_TrancheTimestampsNotOrdered(
        uint256 index, uint40 previousTimestamp, uint40 currentTimestamp
    );
}
