// SPDX-License-Identifier: BUSL-1.1
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { ERC721 } from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

import { SablierV2Lockup } from "./abstracts/SablierV2Lockup.sol";
import { ISablierV2LockupTranched } from "./interfaces/ISablierV2LockupTranched.sol";
import { ISablierV2NFTDescriptor } from "./interfaces/ISablierV2NFTDescriptor.sol";
import { Helpers } from "./libraries/Helpers.sol";
import { Lockup, LockupTranched } from "./types/DataTypes.sol";

/*

███████╗ █████╗ ██████╗ ██╗     ██╗███████╗██████╗     ██╗   ██╗██████╗
██╔════╝██╔══██╗██╔══██╗██║     ██║██╔════╝██╔══██╗    ██║   ██║╚════██╗
███████╗███████║██████╔╝██║     ██║█████╗  ██████╔╝    ██║   ██║ █████╔╝
╚════██║██╔══██║██╔══██╗██║     ██║██╔══╝  ██╔══██╗    ╚██╗ ██╔╝██╔═══╝
███████║██║  ██║██████╔╝███████╗██║███████╗██║  ██║     ╚████╔╝ ███████╗
╚══════╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚═╝╚══════╝╚═╝  ╚═╝      ╚═══╝  ╚══════╝

██╗      ██████╗  ██████╗██╗  ██╗██╗   ██╗██████╗    ████████╗██████╗  █████╗ ███╗   ██╗ ██████╗██╗  ██╗███████╗██████╗
██║     ██╔═══██╗██╔════╝██║ ██╔╝██║   ██║██╔══██╗   ╚══██╔══╝██╔══██╗██╔══██╗████╗  ██║██╔════╝██║  ██║██╔════╝██╔══██╗
██║     ██║   ██║██║     █████╔╝ ██║   ██║██████╔╝      ██║   ██████╔╝███████║██╔██╗ ██║██║     ███████║█████╗  ██║  ██║
██║     ██║   ██║██║     ██╔═██╗ ██║   ██║██╔═══╝       ██║   ██╔══██╗██╔══██║██║╚██╗██║██║     ██╔══██║██╔══╝  ██║  ██║
███████╗╚██████╔╝╚██████╗██║  ██╗╚██████╔╝██║           ██║   ██║  ██║██║  ██║██║ ╚████║╚██████╗██║  ██║███████╗██████╔╝
╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝           ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═════╝

*/

/// @title SablierV2LockupTranched
/// @notice See the documentation in {ISablierV2LockupTranched}.
contract SablierV2LockupTranched is
    ISablierV2LockupTranched, // 5 inherited components
    SablierV2Lockup // 14 inherited components
{
    using SafeERC20 for IERC20;

    /*//////////////////////////////////////////////////////////////////////////
                                  STATE VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2LockupTranched
    uint256 public immutable override MAX_TRANCHE_COUNT;

    /// @dev Stream tranches mapped by stream IDs. This complements the `_streams` mapping in {SablierV2Lockup}.
    mapping(uint256 id => LockupTranched.Tranche[] tranches) internal _tranches;

    /*//////////////////////////////////////////////////////////////////////////
                                     CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Emits a {TransferAdmin} event.
    /// @param initialAdmin The address of the initial contract admin.
    /// @param initialNFTDescriptor The address of the NFT descriptor contract.
    /// @param maxTrancheCount The maximum number of tranches allowed in a stream.
    constructor(
        address initialAdmin,
        ISablierV2NFTDescriptor initialNFTDescriptor,
        uint256 maxTrancheCount
    )
        ERC721("Sablier V2 Lockup Tranched NFT", "SAB-V2-LOCKUP-TRA")
        SablierV2Lockup(initialAdmin, initialNFTDescriptor)
    {
        MAX_TRANCHE_COUNT = maxTrancheCount;
        nextStreamId = 1;
    }

    /*//////////////////////////////////////////////////////////////////////////
                           USER-FACING CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2LockupTranched
    function getStream(uint256 streamId)
        external
        view
        override
        notNull(streamId)
        returns (LockupTranched.StreamLT memory stream)
    {
        // Retrieve the Lockup stream from storage.
        Lockup.Stream memory lockupStream = _streams[streamId];

        // Settled streams cannot be canceled.
        if (_statusOf(streamId) == Lockup.Status.SETTLED) {
            lockupStream.isCancelable = false;
        }

        stream = LockupTranched.StreamLT({
            amounts: lockupStream.amounts,
            asset: lockupStream.asset,
            endTime: lockupStream.endTime,
            isCancelable: lockupStream.isCancelable,
            isDepleted: lockupStream.isDepleted,
            isStream: lockupStream.isStream,
            isTransferable: lockupStream.isTransferable,
            recipient: _ownerOf(streamId),
            sender: lockupStream.sender,
            startTime: lockupStream.startTime,
            tranches: _tranches[streamId],
            wasCanceled: lockupStream.wasCanceled
        });
    }

    /// @inheritdoc ISablierV2LockupTranched
    function getTimestamps(uint256 streamId)
        external
        view
        override
        notNull(streamId)
        returns (LockupTranched.Timestamps memory timestamps)
    {
        timestamps = LockupTranched.Timestamps({ start: _streams[streamId].startTime, end: _streams[streamId].endTime });
    }

    /// @inheritdoc ISablierV2LockupTranched
    function getTranches(uint256 streamId)
        external
        view
        override
        notNull(streamId)
        returns (LockupTranched.Tranche[] memory tranches)
    {
        tranches = _tranches[streamId];
    }

    /*//////////////////////////////////////////////////////////////////////////
                         USER-FACING NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2LockupTranched
    function createWithDurations(LockupTranched.CreateWithDurations calldata params)
        external
        override
        noDelegateCall
        returns (uint256 streamId)
    {
        // Generate the canonical tranches.
        LockupTranched.Tranche[] memory tranches = Helpers.calculateTrancheTimestamps(params.tranches);

        // Checks, Effects and Interactions: create the stream.
        streamId = _create(
            LockupTranched.CreateWithTimestamps({
                sender: params.sender,
                recipient: params.recipient,
                totalAmount: params.totalAmount,
                asset: params.asset,
                cancelable: params.cancelable,
                transferable: params.transferable,
                startTime: uint40(block.timestamp),
                tranches: tranches,
                broker: params.broker
            })
        );
    }

    /// @inheritdoc ISablierV2LockupTranched
    function createWithTimestamps(LockupTranched.CreateWithTimestamps calldata params)
        external
        override
        noDelegateCall
        returns (uint256 streamId)
    {
        // Checks, Effects and Interactions: create the stream.
        streamId = _create(params);
    }

    /*//////////////////////////////////////////////////////////////////////////
                             INTERNAL CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc SablierV2Lockup
    /// @dev The distribution function is:
    ///
    /// $$
    /// f(x) = \Sigma(eta)
    /// $$
    ///
    /// Where:
    ///
    /// - $\Sigma(eta)$ is the sum of all vested tranches' amounts.
    function _calculateStreamedAmount(uint256 streamId) internal view override returns (uint128) {
        uint40 blockTimestamp = uint40(block.timestamp);
        LockupTranched.Tranche[] memory tranches = _tranches[streamId];

        // If the first tranche's timestamp is in the future, return zero.
        if (tranches[0].timestamp > blockTimestamp) {
            return 0;
        }

        // If the end time is not in the future, return the deposited amount.
        if (_streams[streamId].endTime <= blockTimestamp) {
            return _streams[streamId].amounts.deposited;
        }

        // Sum the amounts in all tranches that have already been vested.
        // Using unchecked arithmetic is safe because the sum of the tranche amounts is equal to the total amount
        // at this point.
        uint128 streamedAmount = tranches[0].amount;
        for (uint256 i = 1; i < tranches.length; ++i) {
            // The loop breaks at the first tranche with a timestamp in the future. A tranche is considered vested if
            // its timestamp is less than or equal to the block timestamp.
            if (tranches[i].timestamp > blockTimestamp) {
                break;
            }
            unchecked {
                streamedAmount += tranches[i].amount;
            }
        }

        return streamedAmount;
    }

    /*//////////////////////////////////////////////////////////////////////////
                           INTERNAL NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev See the documentation for the user-facing functions that call this internal function.
    function _create(LockupTranched.CreateWithTimestamps memory params) internal returns (uint256 streamId) {
        // Check: verify the broker fee and calculate the amounts.
        Lockup.CreateAmounts memory createAmounts =
            Helpers.checkAndCalculateBrokerFee(params.totalAmount, params.broker.fee, MAX_BROKER_FEE);

        // Check: validate the user-provided parameters.
        Helpers.checkCreateLockupTranched(createAmounts.deposit, params.tranches, MAX_TRANCHE_COUNT, params.startTime);

        // Load the stream ID in a variable.
        streamId = nextStreamId;

        // Effect: create the stream.
        Lockup.Stream storage stream = _streams[streamId];
        stream.amounts.deposited = createAmounts.deposit;
        stream.asset = params.asset;
        stream.isCancelable = params.cancelable;
        stream.isStream = true;
        stream.isTransferable = params.transferable;
        stream.sender = params.sender;
        stream.startTime = params.startTime;

        unchecked {
            // The tranche count cannot be zero at this point.
            uint256 trancheCount = params.tranches.length;
            stream.endTime = params.tranches[trancheCount - 1].timestamp;

            // Effect: store the tranches. Since Solidity lacks a syntax for copying arrays of structs directly from
            // memory to storage, a manual approach is necessary. See https://github.com/ethereum/solidity/issues/12783.
            for (uint256 i = 0; i < trancheCount; ++i) {
                _tranches[streamId].push(params.tranches[i]);
            }

            // Effect: bump the next stream ID.
            // Using unchecked arithmetic because these calculations cannot realistically overflow, ever.
            nextStreamId = streamId + 1;
        }

        // Effect: mint the NFT to the recipient.
        _mint({ to: params.recipient, tokenId: streamId });

        // Interaction: transfer the deposit amount.
        params.asset.safeTransferFrom({ from: msg.sender, to: address(this), value: createAmounts.deposit });

        // Interaction: pay the broker fee, if not zero.
        if (createAmounts.brokerFee > 0) {
            params.asset.safeTransferFrom({ from: msg.sender, to: params.broker.account, value: createAmounts.brokerFee });
        }

        // Log the newly created stream.
        emit ISablierV2LockupTranched.CreateLockupTranchedStream({
            streamId: streamId,
            funder: msg.sender,
            sender: params.sender,
            recipient: params.recipient,
            amounts: createAmounts,
            asset: params.asset,
            cancelable: params.cancelable,
            transferable: params.transferable,
            tranches: params.tranches,
            timestamps: LockupTranched.Timestamps({ start: stream.startTime, end: stream.endTime }),
            broker: params.broker.account
        });
    }
}
