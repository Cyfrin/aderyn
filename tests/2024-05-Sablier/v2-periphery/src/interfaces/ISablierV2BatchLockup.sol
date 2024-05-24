// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { ISablierV2LockupDynamic } from "@sablier/v2-core/src/interfaces/ISablierV2LockupDynamic.sol";
import { ISablierV2LockupLinear } from "@sablier/v2-core/src/interfaces/ISablierV2LockupLinear.sol";
import { ISablierV2LockupTranched } from "@sablier/v2-core/src/interfaces/ISablierV2LockupTranched.sol";

import { BatchLockup } from "../types/DataTypes.sol";

/// @title ISablierV2BatchLockup
/// @notice Helper to batch create Sablier V2 Lockup streams.
interface ISablierV2BatchLockup {
    /*//////////////////////////////////////////////////////////////////////////
                              SABLIER-V2-LOCKUP-LINEAR
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Creates a batch of LockupLinear streams using `createWithDurations`.
    ///
    /// @dev Requirements:
    /// - There must be at least one element in `batch`.
    /// - All requirements from {ISablierV2LockupLinear.createWithDurations} must be met for each stream.
    ///
    /// @param lockupLinear The address of the {SablierV2LockupLinear} contract.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param batch An array of structs, each encapsulating a subset of the parameters of
    /// {SablierV2LockupLinear.createWithDurations}.
    /// @return streamIds The ids of the newly created streams.
    function createWithDurationsLL(
        ISablierV2LockupLinear lockupLinear,
        IERC20 asset,
        BatchLockup.CreateWithDurationsLL[] calldata batch
    )
        external
        returns (uint256[] memory streamIds);

    /// @notice Creates a batch of LockupLinear streams using `createWithTimestamps`.
    ///
    /// @dev Requirements:
    /// - There must be at least one element in `batch`.
    /// - All requirements from {ISablierV2LockupLinear.createWithTimestamps} must be met for each stream.
    ///
    /// @param lockupLinear The address of the {SablierV2LockupLinear} contract.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param batch An array of structs, each encapsulating a subset of the parameters of
    /// {SablierV2LockupLinear.createWithTimestamps}.
    /// @return streamIds The ids of the newly created streams.
    function createWithTimestampsLL(
        ISablierV2LockupLinear lockupLinear,
        IERC20 asset,
        BatchLockup.CreateWithTimestampsLL[] calldata batch
    )
        external
        returns (uint256[] memory streamIds);

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-LOCKUP-DYNAMIC
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Creates a batch of Lockup Dynamic streams using `createWithDurations`.
    ///
    /// @dev Requirements:
    /// - There must be at least one element in `batch`.
    /// - All requirements from {ISablierV2LockupDynamic.createWithDurations} must be met for each stream.
    ///
    /// @param lockupDynamic The address of the {SablierV2LockupDynamic} contract.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param batch An array of structs, each encapsulating a subset of the parameters of
    /// {SablierV2LockupDynamic.createWithDurations}.
    /// @return streamIds The ids of the newly created streams.
    function createWithDurationsLD(
        ISablierV2LockupDynamic lockupDynamic,
        IERC20 asset,
        BatchLockup.CreateWithDurationsLD[] calldata batch
    )
        external
        returns (uint256[] memory streamIds);

    /// @notice Creates a batch of Lockup Dynamic streams using `createWithTimestamps`.
    ///
    /// @dev Requirements:
    /// - There must be at least one element in `batch`.
    /// - All requirements from {ISablierV2LockupDynamic.createWithTimestamps} must be met for each stream.
    ///
    /// @param lockupDynamic The address of the {SablierV2LockupDynamic} contract.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param batch An array of structs, each encapsulating a subset of the parameters of
    /// {SablierV2LockupDynamic.createWithTimestamps}.
    /// @return streamIds The ids of the newly created streams.
    function createWithTimestampsLD(
        ISablierV2LockupDynamic lockupDynamic,
        IERC20 asset,
        BatchLockup.CreateWithTimestampsLD[] calldata batch
    )
        external
        returns (uint256[] memory streamIds);

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-LOCKUP-TRANCHED
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Creates a batch of LockupTranched streams using `createWithDurations`.
    ///
    /// @dev Requirements:
    /// - There must be at least one element in `batch`.
    /// - All requirements from {ISablierV2LockupTranched.createWithDurations} must be met for each stream.
    ///
    /// @param lockupTranched The address of the {SablierV2LockupTranched} contract.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param batch An array of structs, each encapsulating a subset of the parameters of
    /// {SablierV2LockupTranched.createWithDurations}.
    /// @return streamIds The ids of the newly created streams.
    function createWithDurationsLT(
        ISablierV2LockupTranched lockupTranched,
        IERC20 asset,
        BatchLockup.CreateWithDurationsLT[] calldata batch
    )
        external
        returns (uint256[] memory streamIds);

    /// @notice Creates a batch of LockupTranched streams using `createWithTimestamps`.
    ///
    /// @dev Requirements:
    /// - There must be at least one element in `batch`.
    /// - All requirements from {ISablierV2LockupTranched.createWithTimestamps} must be met for each stream.
    ///
    /// @param lockupTranched The address of the {SablierV2LockupTranched} contract.
    /// @param asset The contract address of the ERC-20 asset to be distributed.
    /// @param batch An array of structs, each encapsulating a subset of the parameters of
    /// {SablierV2LockupTranched.createWithTimestamps}.
    /// @return streamIds The ids of the newly created streams.
    function createWithTimestampsLT(
        ISablierV2LockupTranched lockupTranched,
        IERC20 asset,
        BatchLockup.CreateWithTimestampsLT[] calldata batch
    )
        external
        returns (uint256[] memory streamIds);
}
