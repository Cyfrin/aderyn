// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { UD2x18 } from "@prb/math/src/UD2x18.sol";
import { UD60x18 } from "@prb/math/src/UD60x18.sol";

// DataTypes.sol
//
// This file defines all structs used in V2 Core, most of which are organized under three namespaces:
//
// - Lockup
// - LockupDynamic
// - LockupLinear
// - LockupTranched
//
// You will notice that some structs contain "slot" annotations - they are used to indicate the
// storage layout of the struct. It is more gas efficient to group small data types together so
// that they fit in a single 32-byte slot.

/// @notice Struct encapsulating the broker parameters passed to the create functions. Both can be set to zero.
/// @param account The address receiving the broker's fee.
/// @param fee The broker's percentage fee from the total amount, denoted as a fixed-point number where 1e18 is 100%.
struct Broker {
    address account;
    UD60x18 fee;
}

/// @notice Namespace for the structs used in both {SablierV2LockupLinear} and {SablierV2LockupDynamic}.
library Lockup {
    /// @notice Struct encapsulating the deposit, withdrawn, and refunded amounts, both denoted in units of the asset's
    /// decimals.
    /// @dev Because the deposited and the withdrawn amount are often read together, declaring them in the same slot
    /// saves gas.
    /// @param deposited The initial amount deposited in the stream, net of broker fee.
    /// @param withdrawn The cumulative amount withdrawn from the stream.
    /// @param refunded The amount refunded to the sender. Unless the stream was canceled, this is always zero.
    struct Amounts {
        // slot 0
        uint128 deposited;
        uint128 withdrawn;
        // slot 1
        uint128 refunded;
    }

    /// @notice Struct encapsulating the deposit amount and the broker fee amount, both denoted in units of the asset's
    /// decimals.
    /// @param deposit The amount to deposit in the stream.
    /// @param brokerFee The broker fee amount.
    struct CreateAmounts {
        uint128 deposit;
        uint128 brokerFee;
    }

    /// @notice Enum representing the different statuses of a stream.
    /// @custom:value0 PENDING Stream created but not started; assets are in a pending state.
    /// @custom:value1 STREAMING Active stream where assets are currently being streamed.
    /// @custom:value2 SETTLED All assets have been streamed; recipient is due to withdraw them.
    /// @custom:value3 CANCELED Canceled stream; remaining assets await recipient's withdrawal.
    /// @custom:value4 DEPLETED Depleted stream; all assets have been withdrawn and/or refunded.
    enum Status {
        PENDING,
        STREAMING,
        SETTLED,
        CANCELED,
        DEPLETED
    }

    /// @notice A common data structure to be stored in all {SablierV2Lockup} models.
    /// @dev The fields are arranged like this to save gas via tight variable packing.
    /// @param sender The address distributing the assets, with the ability to cancel the stream.
    /// @param startTime The Unix timestamp indicating the stream's start.
    /// @param endTime The Unix timestamp indicating the stream's end.
    /// @param isCancelable Boolean indicating if the stream is cancelable.
    /// @param wasCanceled Boolean indicating if the stream was canceled.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param isDepleted Boolean indicating if the stream is depleted.
    /// @param isStream Boolean indicating if the struct entity exists.
    /// @param isTransferable Boolean indicating if the stream NFT is transferable.
    /// @param amounts Struct containing the deposit, withdrawn, and refunded amounts, both denoted in units of the
    /// asset's decimals.
    struct Stream {
        // slot 0
        address sender;
        uint40 startTime;
        uint40 endTime;
        bool isCancelable;
        bool wasCanceled;
        // slot 1
        IERC20 asset;
        bool isDepleted;
        bool isStream;
        bool isTransferable;
        // slot 2 and 3
        Lockup.Amounts amounts;
    }
}

/// @notice Namespace for the structs used in {SablierV2LockupDynamic}.
library LockupDynamic {
    /// @notice Struct encapsulating the parameters of the {SablierV2LockupDynamic.createWithDurations} function.
    /// @param sender The address distributing the assets, with the ability to cancel the stream. It doesn't have to be
    /// the same as `msg.sender`.
    /// @param recipient The address receiving the assets.
    /// @param totalAmount The total amount of ERC-20 assets to be distributed, including the stream deposit and any
    /// broker fee, both denoted in units of the asset's decimals.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param cancelable Indicates if the stream is cancelable.
    /// @param transferable Indicates if the stream NFT is transferable.
    /// @param segments Segments with durations used to compose the dynamic distribution function. Timestamps are
    /// calculated by starting from `block.timestamp` and adding each duration to the previous timestamp.
    /// @param broker Struct containing (i) the address of the broker assisting in creating the stream, and (ii) the
    /// percentage fee paid to the broker from `totalAmount`, denoted as a fixed-point number. Both can be set to zero.
    struct CreateWithDurations {
        address sender;
        address recipient;
        uint128 totalAmount;
        IERC20 asset;
        bool cancelable;
        bool transferable;
        SegmentWithDuration[] segments;
        Broker broker;
    }

    /// @notice Struct encapsulating the parameters of the {SablierV2LockupDynamic.createWithTimestamps} function.
    /// @param sender The address distributing the assets, with the ability to cancel the stream. It doesn't have to be
    /// the same as `msg.sender`.
    /// @param recipient The address receiving the assets.
    /// @param totalAmount The total amount of ERC-20 assets to be distributed, including the stream deposit and any
    /// broker fee, both denoted in units of the asset's decimals.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param cancelable Indicates if the stream is cancelable.
    /// @param transferable Indicates if the stream NFT is transferable.
    /// @param startTime The Unix timestamp indicating the stream's start.
    /// @param segments Segments used to compose the dynamic distribution function.
    /// @param broker Struct containing (i) the address of the broker assisting in creating the stream, and (ii) the
    /// percentage fee paid to the broker from `totalAmount`, denoted as a fixed-point number. Both can be set to zero.
    struct CreateWithTimestamps {
        address sender;
        address recipient;
        uint128 totalAmount;
        IERC20 asset;
        bool cancelable;
        bool transferable;
        uint40 startTime;
        Segment[] segments;
        Broker broker;
    }

    /// @notice Segment struct used in the Lockup Dynamic stream.
    /// @param amount The amount of assets to be streamed in the segment, denoted in units of the asset's decimals.
    /// @param exponent The exponent of the segment, denoted as a fixed-point number.
    /// @param timestamp The Unix timestamp indicating the segment's end.
    struct Segment {
        // slot 0
        uint128 amount;
        UD2x18 exponent;
        uint40 timestamp;
    }

    /// @notice Segment struct used at runtime in {SablierV2LockupDynamic.createWithDurations}.
    /// @param amount The amount of assets to be streamed in the segment, denoted in units of the asset's decimals.
    /// @param exponent The exponent of the segment, denoted as a fixed-point number.
    /// @param duration The time difference in seconds between the segment and the previous one.
    struct SegmentWithDuration {
        uint128 amount;
        UD2x18 exponent;
        uint40 duration;
    }

    /// @notice Struct encapsulating the full details of a stream.
    /// @dev Extends `Lockup.Stream` by including the recipient and the segments.
    struct StreamLD {
        address sender;
        address recipient;
        uint40 startTime;
        uint40 endTime;
        bool isCancelable;
        bool wasCanceled;
        IERC20 asset;
        bool isDepleted;
        bool isStream;
        bool isTransferable;
        Lockup.Amounts amounts;
        Segment[] segments;
    }

    /// @notice Struct encapsulating the LockupDynamic timestamps.
    /// @param start The Unix timestamp indicating the stream's start.
    /// @param end The Unix timestamp indicating the stream's end.
    struct Timestamps {
        uint40 start;
        uint40 end;
    }
}

/// @notice Namespace for the structs used in {SablierV2LockupLinear}.
library LockupLinear {
    /// @notice Struct encapsulating the parameters of the {SablierV2LockupLinear.createWithDurations} function.
    /// @param sender The address distributing the assets, with the ability to cancel the stream. It doesn't have to be
    /// the same as `msg.sender`.
    /// @param recipient The address receiving the assets.
    /// @param totalAmount The total amount of ERC-20 assets to be distributed, including the stream deposit and any
    /// broker fee, both denoted in units of the asset's decimals.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param cancelable Indicates if the stream is cancelable.
    /// @param transferable Indicates if the stream NFT is transferable.
    /// @param durations Struct containing (i) cliff period duration and (ii) total stream duration, both in seconds.
    /// @param broker Struct containing (i) the address of the broker assisting in creating the stream, and (ii) the
    /// percentage fee paid to the broker from `totalAmount`, denoted as a fixed-point number. Both can be set to zero.
    struct CreateWithDurations {
        address sender;
        address recipient;
        uint128 totalAmount;
        IERC20 asset;
        bool cancelable;
        bool transferable;
        Durations durations;
        Broker broker;
    }

    /// @notice Struct encapsulating the parameters of the {SablierV2LockupLinear.createWithTimestamps} function.
    /// @param sender The address distributing the assets, with the ability to cancel the stream. It doesn't have to be
    /// the same as `msg.sender`.
    /// @param recipient The address receiving the assets.
    /// @param totalAmount The total amount of ERC-20 assets to be distributed, including the stream deposit and any
    /// broker fee, both denoted in units of the asset's decimals.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param cancelable Indicates if the stream is cancelable.
    /// @param transferable Indicates if the stream NFT is transferable.
    /// @param timestamps Struct containing (i) the stream's start time, (ii) cliff time, and (iii) end time, all as
    /// Unix timestamps.
    /// @param broker Struct containing (i) the address of the broker assisting in creating the stream, and (ii) the
    /// percentage fee paid to the broker from `totalAmount`, denoted as a fixed-point number. Both can be set to zero.
    struct CreateWithTimestamps {
        address sender;
        address recipient;
        uint128 totalAmount;
        IERC20 asset;
        bool cancelable;
        bool transferable;
        Timestamps timestamps;
        Broker broker;
    }

    /// @notice Struct encapsulating the cliff duration and the total duration.
    /// @param cliff The cliff duration in seconds.
    /// @param total The total duration in seconds.
    struct Durations {
        uint40 cliff;
        uint40 total;
    }

    /// @notice Struct encapsulating the full details of a stream.
    /// @dev Extends `Lockup.Stream` by including the recipient and the cliff time.
    struct StreamLL {
        address sender;
        address recipient;
        uint40 startTime;
        bool isCancelable;
        bool wasCanceled;
        IERC20 asset;
        uint40 endTime;
        bool isDepleted;
        bool isStream;
        bool isTransferable;
        Lockup.Amounts amounts;
        uint40 cliffTime;
    }

    /// @notice Struct encapsulating the LockupLinear timestamps.
    /// @param start The Unix timestamp for the stream's start.
    /// @param cliff The Unix timestamp for the cliff period's end. A value of zero means there is no cliff.
    /// @param end The Unix timestamp for the stream's end.
    struct Timestamps {
        uint40 start;
        uint40 cliff;
        uint40 end;
    }
}

/// @notice Namespace for the structs used in {SablierV2LockupTranched}.
library LockupTranched {
    /// @notice Struct encapsulating the parameters of the {SablierV2LockupTranched.createWithDurations} function.
    /// @param sender The address distributing the assets, with the ability to cancel the stream. It doesn't have to be
    /// the same as `msg.sender`.
    /// @param recipient The address receiving the assets.
    /// @param totalAmount The total amount of ERC-20 assets to be distributed, including the stream deposit and any
    /// broker fee, both denoted in units of the asset's decimals.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param cancelable Indicates if the stream is cancelable.
    /// @param transferable Indicates if the stream NFT is transferable.
    /// @param tranches Tranches with durations used to compose the tranched distribution function. Timestamps are
    /// calculated by starting from `block.timestamp` and adding each duration to the previous timestamp.
    /// @param broker Struct containing (i) the address of the broker assisting in creating the stream, and (ii) the
    /// percentage fee paid to the broker from `totalAmount`, denoted as a fixed-point number. Both can be set to zero.
    struct CreateWithDurations {
        address sender;
        address recipient;
        uint128 totalAmount;
        IERC20 asset;
        bool cancelable;
        bool transferable;
        TrancheWithDuration[] tranches;
        Broker broker;
    }

    /// @notice Struct encapsulating the parameters of the {SablierV2LockupTranched.createWithTimestamps} function.
    /// @param sender The address distributing the assets, with the ability to cancel the stream. It doesn't have to be
    /// the same as `msg.sender`.
    /// @param recipient The address receiving the assets.
    /// @param totalAmount The total amount of ERC-20 assets to be distributed, including the stream deposit and any
    /// broker fee, both denoted in units of the asset's decimals.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param cancelable Indicates if the stream is cancelable.
    /// @param transferable Indicates if the stream NFT is transferable.
    /// @param startTime The Unix timestamp indicating the stream's start.
    /// @param tranches Tranches used to compose the tranched distribution function.
    /// @param broker Struct containing (i) the address of the broker assisting in creating the stream, and (ii) the
    /// percentage fee paid to the broker from `totalAmount`, denoted as a fixed-point number. Both can be set to zero.
    struct CreateWithTimestamps {
        address sender;
        address recipient;
        uint128 totalAmount;
        IERC20 asset;
        bool cancelable;
        bool transferable;
        uint40 startTime;
        Tranche[] tranches;
        Broker broker;
    }

    /// @notice Struct encapsulating the full details of a stream.
    /// @dev Extends `Lockup.Stream` by including the recipient and the tranches.
    struct StreamLT {
        address sender;
        address recipient;
        uint40 startTime;
        uint40 endTime;
        bool isCancelable;
        bool wasCanceled;
        IERC20 asset;
        bool isDepleted;
        bool isStream;
        bool isTransferable;
        Lockup.Amounts amounts;
        Tranche[] tranches;
    }

    /// @notice Struct encapsulating the LockupTranched timestamps.
    /// @param start The Unix timestamp indicating the stream's start.
    /// @param end The Unix timestamp indicating the stream's end.
    struct Timestamps {
        uint40 start;
        uint40 end;
    }

    /// @notice Tranche struct used in the Lockup Tranched stream.
    /// @param amount The amount of assets to be unlocked in the tranche, denoted in units of the asset's decimals.
    /// @param timestamp The Unix timestamp indicating the tranche's end.
    struct Tranche {
        // slot 0
        uint128 amount;
        uint40 timestamp;
    }

    /// @notice Tranche struct used at runtime in {SablierV2LockupTranched.createWithDurations}.
    /// @param amount The amount of assets to be unlocked in the tranche, denoted in units of the asset's decimals.
    /// @param duration The time difference in seconds between the tranche and the previous one.
    struct TrancheWithDuration {
        uint128 amount;
        uint40 duration;
    }
}
