// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { UD2x18 } from "@prb/math/src/UD2x18.sol";
import { ISablierV2Lockup } from "@sablier/v2-core/src/interfaces/ISablierV2Lockup.sol";
import { Broker, LockupDynamic, LockupLinear, LockupTranched } from "@sablier/v2-core/src/types/DataTypes.sol";

library BatchLockup {
    /// @notice A struct encapsulating the lockup contract's address and the stream ids to cancel.
    struct CancelMultiple {
        ISablierV2Lockup lockup;
        uint256[] streamIds;
    }

    /// @notice A struct encapsulating all parameters of {SablierV2LockupDynamic.createWithDurations} except for the
    /// asset.
    struct CreateWithDurationsLD {
        address sender;
        address recipient;
        uint128 totalAmount;
        bool cancelable;
        bool transferable;
        LockupDynamic.SegmentWithDuration[] segments;
        Broker broker;
    }

    /// @notice A struct encapsulating all parameters of {SablierV2LockupLinear.createWithDurations} except for the
    /// asset.
    struct CreateWithDurationsLL {
        address sender;
        address recipient;
        uint128 totalAmount;
        bool cancelable;
        bool transferable;
        LockupLinear.Durations durations;
        Broker broker;
    }

    /// @notice A struct encapsulating all parameters of {SablierV2LockupTranched.createWithDurations} except for the
    /// asset.
    struct CreateWithDurationsLT {
        address sender;
        address recipient;
        uint128 totalAmount;
        bool cancelable;
        bool transferable;
        LockupTranched.TrancheWithDuration[] tranches;
        Broker broker;
    }

    /// @notice A struct encapsulating all parameters of {SablierV2LockupDynamic.createWithTimestamps} except for the
    /// asset.
    struct CreateWithTimestampsLD {
        address sender;
        address recipient;
        uint128 totalAmount;
        bool cancelable;
        bool transferable;
        uint40 startTime;
        LockupDynamic.Segment[] segments;
        Broker broker;
    }

    /// @notice A struct encapsulating all parameters of {SablierV2LockupLinear.createWithTimestamps} except for the
    /// asset.
    struct CreateWithTimestampsLL {
        address sender;
        address recipient;
        uint128 totalAmount;
        bool cancelable;
        bool transferable;
        LockupLinear.Timestamps timestamps;
        Broker broker;
    }

    /// @notice A struct encapsulating all parameters of {SablierV2LockupTranched.createWithTimestamps} except for the
    /// asset.
    struct CreateWithTimestampsLT {
        address sender;
        address recipient;
        uint128 totalAmount;
        bool cancelable;
        bool transferable;
        uint40 startTime;
        LockupTranched.Tranche[] tranches;
        Broker broker;
    }
}

library MerkleLockup {
    /// @notice Struct encapsulating the base constructor parameters of a MerkleLockup campaign.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param cancelable Indicates if the stream will be cancelable after claiming.
    /// @param expiration The expiration of the campaign, as a Unix timestamp.
    /// @param initialAdmin The initial admin of the MerkleLockup campaign.
    /// @param ipfsCID The content identifier for indexing the contract on IPFS.
    /// @param merkleRoot The Merkle root of the claim data.
    /// @param name The name of the campaign.
    /// @param transferable Indicates if the stream will be transferable after claiming.
    struct ConstructorParams {
        IERC20 asset;
        bool cancelable;
        uint40 expiration;
        address initialAdmin;
        string ipfsCID;
        bytes32 merkleRoot;
        string name;
        bool transferable;
    }
}

library MerkleLT {
    /// @notice Struct encapsulating the unlock percentage and duration of a tranche.
    /// @dev Since users may have different amounts allocated, this struct makes it possible to calculate the amounts
    /// at claim time. An 18-decimal format is used to represent percentages: 100% = 1e18. For more information, see
    /// the PRBMath documentation on UD2x18: https://github.com/PaulRBerg/prb-math
    /// @param unlockPercentage The percentage designated to be unlocked in this tranche.
    /// @param duration The time difference in seconds between this tranche and the previous one.
    struct TrancheWithPercentage {
        // slot 0
        UD2x18 unlockPercentage;
        uint40 duration;
    }
}
