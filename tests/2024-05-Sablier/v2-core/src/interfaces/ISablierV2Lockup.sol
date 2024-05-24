// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IERC721Metadata } from "@openzeppelin/contracts/token/ERC721/extensions/IERC721Metadata.sol";
import { UD60x18 } from "@prb/math/src/UD60x18.sol";

import { Lockup } from "../types/DataTypes.sol";
import { IAdminable } from "./IAdminable.sol";
import { ISablierV2NFTDescriptor } from "./ISablierV2NFTDescriptor.sol";

/// @title ISablierV2Lockup
/// @notice Common logic between all Sablier V2 Lockup contracts.
interface ISablierV2Lockup is
    IAdminable, // 0 inherited components
    IERC721Metadata // 2 inherited components
{
    /*//////////////////////////////////////////////////////////////////////////
                                       EVENTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a stream is canceled.
    /// @param streamId The ID of the stream.
    /// @param sender The address of the stream's sender.
    /// @param recipient The address of the stream's recipient.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param senderAmount The amount of assets refunded to the stream's sender, denoted in units of the asset's
    /// decimals.
    /// @param recipientAmount The amount of assets left for the stream's recipient to withdraw, denoted in units of the
    /// asset's decimals.
    event CancelLockupStream(
        uint256 streamId,
        address indexed sender,
        address indexed recipient,
        IERC20 indexed asset,
        uint128 senderAmount,
        uint128 recipientAmount
    );

    /// @notice Emitted when a sender gives up the right to cancel a stream.
    /// @param streamId The ID of the stream.
    event RenounceLockupStream(uint256 indexed streamId);

    /// @notice Emitted when the admin sets a new NFT descriptor contract.
    /// @param admin The address of the current contract admin.
    /// @param oldNFTDescriptor The address of the old NFT descriptor contract.
    /// @param newNFTDescriptor The address of the new NFT descriptor contract.
    event SetNFTDescriptor(
        address indexed admin, ISablierV2NFTDescriptor oldNFTDescriptor, ISablierV2NFTDescriptor newNFTDescriptor
    );

    /// @notice Emitted when assets are withdrawn from a stream.
    /// @param streamId The ID of the stream.
    /// @param to The address that has received the withdrawn assets.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param amount The amount of assets withdrawn, denoted in units of the asset's decimals.
    event WithdrawFromLockupStream(uint256 indexed streamId, address indexed to, IERC20 indexed asset, uint128 amount);

    /*//////////////////////////////////////////////////////////////////////////
                                 CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Retrieves the address of the ERC-20 asset to be distributed.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function getAsset(uint256 streamId) external view returns (IERC20 asset);

    /// @notice Retrieves the amount deposited in the stream, denoted in units of the asset's decimals.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function getDepositedAmount(uint256 streamId) external view returns (uint128 depositedAmount);

    /// @notice Retrieves the stream's end time, which is a Unix timestamp.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function getEndTime(uint256 streamId) external view returns (uint40 endTime);

    /// @notice Retrieves the stream's recipient.
    /// @dev Reverts if the NFT has been burned.
    /// @param streamId The stream ID for the query.
    function getRecipient(uint256 streamId) external view returns (address recipient);

    /// @notice Retrieves the amount refunded to the sender after a cancellation, denoted in units of the asset's
    /// decimals. This amount is always zero unless the stream was canceled.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function getRefundedAmount(uint256 streamId) external view returns (uint128 refundedAmount);

    /// @notice Retrieves the stream's sender.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function getSender(uint256 streamId) external view returns (address sender);

    /// @notice Retrieves the stream's start time, which is a Unix timestamp.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function getStartTime(uint256 streamId) external view returns (uint40 startTime);

    /// @notice Retrieves the amount withdrawn from the stream, denoted in units of the asset's decimals.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function getWithdrawnAmount(uint256 streamId) external view returns (uint128 withdrawnAmount);

    /// @notice Retrieves a flag indicating whether the stream can be canceled. When the stream is cold, this
    /// flag is always `false`.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function isCancelable(uint256 streamId) external view returns (bool result);

    /// @notice Retrieves a flag indicating whether the stream is cold, i.e. settled, canceled, or depleted.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function isCold(uint256 streamId) external view returns (bool result);

    /// @notice Retrieves a flag indicating whether the stream is depleted.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function isDepleted(uint256 streamId) external view returns (bool result);

    /// @notice Retrieves a flag indicating whether the stream exists.
    /// @dev Does not revert if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function isStream(uint256 streamId) external view returns (bool result);

    /// @notice Retrieves a flag indicating whether the stream NFT can be transferred.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function isTransferable(uint256 streamId) external view returns (bool result);

    /// @notice Retrieves a flag indicating whether the stream is warm, i.e. either pending or streaming.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function isWarm(uint256 streamId) external view returns (bool result);

    /// @notice Retrieves the maximum broker fee that can be charged by the broker, denoted as a fixed-point
    /// number where 1e18 is 100%.
    /// @dev This value is hard coded as a constant.
    function MAX_BROKER_FEE() external view returns (UD60x18);

    /// @notice Counter for stream IDs, used in the create functions.
    function nextStreamId() external view returns (uint256);

    /// @notice Contract that generates the non-fungible token URI.
    function nftDescriptor() external view returns (ISablierV2NFTDescriptor);

    /// @notice Calculates the amount that the sender would be refunded if the stream were canceled, denoted in units
    /// of the asset's decimals.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function refundableAmountOf(uint256 streamId) external view returns (uint128 refundableAmount);

    /// @notice Retrieves the stream's status.
    /// @param streamId The stream ID for the query.
    function statusOf(uint256 streamId) external view returns (Lockup.Status status);

    /// @notice Calculates the amount streamed to the recipient, denoted in units of the asset's decimals.
    /// @dev Reverts if `streamId` references a null stream.
    ///
    /// Notes:
    /// - Upon cancellation of the stream, the amount streamed is calculated as the difference between the deposited
    /// amount and the refunded amount. Ultimately, when the stream becomes depleted, the streamed amount is equivalent
    /// to the total amount withdrawn.
    ///
    /// @param streamId The stream ID for the query.
    function streamedAmountOf(uint256 streamId) external view returns (uint128 streamedAmount);

    /// @notice Retrieves a flag indicating whether the stream was canceled.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function wasCanceled(uint256 streamId) external view returns (bool result);

    /// @notice Calculates the amount that the recipient can withdraw from the stream, denoted in units of the asset's
    /// decimals.
    /// @dev Reverts if `streamId` references a null stream.
    /// @param streamId The stream ID for the query.
    function withdrawableAmountOf(uint256 streamId) external view returns (uint128 withdrawableAmount);

    /*//////////////////////////////////////////////////////////////////////////
                               NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Burns the NFT associated with the stream.
    ///
    /// @dev Emits a {Transfer} event.
    ///
    /// Requirements:
    /// - Must not be delegate called.
    /// - `streamId` must reference a depleted stream.
    /// - The NFT must exist.
    /// - `msg.sender` must be either the NFT owner or an approved third party.
    ///
    /// @param streamId The ID of the stream NFT to burn.
    function burn(uint256 streamId) external;

    /// @notice Cancels the stream and refunds any remaining assets to the sender.
    ///
    /// @dev Emits a {Transfer}, {CancelLockupStream}, and {MetadataUpdate} event.
    ///
    /// Notes:
    /// - If there any assets left for the recipient to withdraw, the stream is marked as canceled. Otherwise, the
    /// stream is marked as depleted.
    /// - This function attempts to invoke a hook on the recipient, if the resolved address is a contract.
    ///
    /// Requirements:
    /// - Must not be delegate called.
    /// - The stream must be warm and cancelable.
    /// - `msg.sender` must be the stream's sender.
    ///
    /// @param streamId The ID of the stream to cancel.
    function cancel(uint256 streamId) external;

    /// @notice Cancels multiple streams and refunds any remaining assets to the sender.
    ///
    /// @dev Emits multiple {Transfer}, {CancelLockupStream}, and {MetadataUpdate} events.
    ///
    /// Notes:
    /// - Refer to the notes in {cancel}.
    ///
    /// Requirements:
    /// - All requirements from {cancel} must be met for each stream.
    ///
    /// @param streamIds The IDs of the streams to cancel.
    function cancelMultiple(uint256[] calldata streamIds) external;

    /// @notice Removes the right of the stream's sender to cancel the stream.
    ///
    /// @dev Emits a {RenounceLockupStream} and {MetadataUpdate} event.
    ///
    /// Notes:
    /// - This is an irreversible operation.
    /// - This function attempts to invoke a hook on the stream's recipient, provided that the recipient is a contract.
    ///
    /// Requirements:
    /// - Must not be delegate called.
    /// - `streamId` must reference a warm stream.
    /// - `msg.sender` must be the stream's sender.
    /// - The stream must be cancelable.
    ///
    /// @param streamId The ID of the stream to renounce.
    function renounce(uint256 streamId) external;

    /// @notice Sets a new NFT descriptor contract, which produces the URI describing the Sablier stream NFTs.
    ///
    /// @dev Emits a {SetNFTDescriptor} and {BatchMetadataUpdate} event.
    ///
    /// Notes:
    /// - Does not revert if the NFT descriptor is the same.
    ///
    /// Requirements:
    /// - `msg.sender` must be the contract admin.
    ///
    /// @param newNFTDescriptor The address of the new NFT descriptor contract.
    function setNFTDescriptor(ISablierV2NFTDescriptor newNFTDescriptor) external;

    /// @notice Withdraws the provided amount of assets from the stream to the `to` address.
    ///
    /// @dev Emits a {Transfer}, {WithdrawFromLockupStream}, and {MetadataUpdate} event.
    ///
    /// Notes:
    /// - This function attempts to call a hook on the recipient of the stream, unless `msg.sender` is the recipient.
    /// - This function attempts to call a hook on the sender of the stream, unless `msg.sender` is the sender.
    ///
    /// Requirements:
    /// - Must not be delegate called.
    /// - `streamId` must not reference a null or depleted stream.
    /// - `to` must not be the zero address.
    /// - `amount` must be greater than zero and must not exceed the withdrawable amount.
    /// - `to` must be the recipient if `msg.sender` is not the stream's recipient or an approved third party.
    ///
    /// @param streamId The ID of the stream to withdraw from.
    /// @param to The address receiving the withdrawn assets.
    /// @param amount The amount to withdraw, denoted in units of the asset's decimals.
    function withdraw(uint256 streamId, address to, uint128 amount) external;

    /// @notice Withdraws the maximum withdrawable amount from the stream to the provided address `to`.
    ///
    /// @dev Emits a {Transfer}, {WithdrawFromLockupStream}, and {MetadataUpdate} event.
    ///
    /// Notes:
    /// - Refer to the notes in {withdraw}.
    ///
    /// Requirements:
    /// - Refer to the requirements in {withdraw}.
    ///
    /// @param streamId The ID of the stream to withdraw from.
    /// @param to The address receiving the withdrawn assets.
    function withdrawMax(uint256 streamId, address to) external;

    /// @notice Withdraws the maximum withdrawable amount from the stream to the current recipient, and transfers the
    /// NFT to `newRecipient`.
    ///
    /// @dev Emits a {WithdrawFromLockupStream} and a {Transfer} event.
    ///
    /// Notes:
    /// - If the withdrawable amount is zero, the withdrawal is skipped.
    /// - Refer to the notes in {withdraw}.
    ///
    /// Requirements:
    /// - `msg.sender` must be the stream's recipient.
    /// - Refer to the requirements in {withdraw}.
    /// - Refer to the requirements in {IERC721.transferFrom}.
    ///
    /// @param streamId The ID of the stream NFT to transfer.
    /// @param newRecipient The address of the new owner of the stream NFT.
    function withdrawMaxAndTransfer(uint256 streamId, address newRecipient) external;

    /// @notice Withdraws assets from streams to the recipient of each stream.
    ///
    /// @dev Emits multiple {Transfer}, {WithdrawFromLockupStream}, and {MetadataUpdate} events.
    ///
    /// Notes:
    /// - This function attempts to call a hook on the recipient of each stream, unless `msg.sender` is the recipient.
    /// - This function attempts to call a hook on the sender of each stream, unless `msg.sender` is the sender.
    ///
    /// Requirements:
    /// - Must not be delegate called.
    /// - There must be an equal number of `streamIds` and `amounts`.
    /// - Each stream ID in the array must not reference a null or depleted stream.
    /// - Each amount in the array must be greater than zero and must not exceed the withdrawable amount.
    ///
    /// @param streamIds The IDs of the streams to withdraw from.
    /// @param amounts The amounts to withdraw, denoted in units of the asset's decimals.
    function withdrawMultiple(uint256[] calldata streamIds, uint128[] calldata amounts) external;
}
