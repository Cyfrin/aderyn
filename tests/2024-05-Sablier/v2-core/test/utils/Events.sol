// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { ISablierV2NFTDescriptor } from "../../src/interfaces/ISablierV2NFTDescriptor.sol";
import { Lockup, LockupDynamic, LockupLinear, LockupTranched } from "../../src/types/DataTypes.sol";

/// @notice Abstract contract containing all the events emitted by the protocol.
abstract contract Events {
    /*//////////////////////////////////////////////////////////////////////////
                                      ERC-721
    //////////////////////////////////////////////////////////////////////////*/

    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);

    /*//////////////////////////////////////////////////////////////////////////
                                      ERC-4906
    //////////////////////////////////////////////////////////////////////////*/

    event MetadataUpdate(uint256 _tokenId);

    event BatchMetadataUpdate(uint256 _fromTokenId, uint256 _toTokenId);

    /*//////////////////////////////////////////////////////////////////////////
                                      GENERICS
    //////////////////////////////////////////////////////////////////////////*/

    event TransferAdmin(address indexed oldAdmin, address indexed newAdmin);

    /*//////////////////////////////////////////////////////////////////////////
                                 SABLIER-V2-LOCKUP
    //////////////////////////////////////////////////////////////////////////*/

    event CancelLockupStream(
        uint256 streamId,
        address indexed sender,
        address indexed recipient,
        IERC20 indexed asset,
        uint128 senderAmount,
        uint128 recipientAmount
    );

    event RenounceLockupStream(uint256 indexed streamId);

    event SetNFTDescriptor(
        address indexed admin, ISablierV2NFTDescriptor oldNFTDescriptor, ISablierV2NFTDescriptor newNFTDescriptor
    );

    event WithdrawFromLockupStream(uint256 indexed streamId, address indexed to, IERC20 indexed asset, uint128 amount);

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-LOCKUP-DYNAMIC
    //////////////////////////////////////////////////////////////////////////*/

    event CreateLockupDynamicStream(
        uint256 streamId,
        address funder,
        address indexed sender,
        address indexed recipient,
        Lockup.CreateAmounts amounts,
        IERC20 indexed asset,
        bool cancelable,
        bool transferable,
        LockupDynamic.Segment[] segments,
        LockupDynamic.Timestamps timestamps,
        address broker
    );

    /*//////////////////////////////////////////////////////////////////////////
                              SABLIER-V2-LOCKUP-LINEAR
    //////////////////////////////////////////////////////////////////////////*/

    event CreateLockupLinearStream(
        uint256 streamId,
        address funder,
        address indexed sender,
        address indexed recipient,
        Lockup.CreateAmounts amounts,
        IERC20 indexed asset,
        bool cancelable,
        bool transferable,
        LockupLinear.Timestamps timestamps,
        address broker
    );

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-LOCKUP-TRANCHED
    //////////////////////////////////////////////////////////////////////////*/

    event CreateLockupTranchedStream(
        uint256 streamId,
        address funder,
        address indexed sender,
        address indexed recipient,
        Lockup.CreateAmounts amounts,
        IERC20 indexed asset,
        bool cancelable,
        bool transferable,
        LockupTranched.Tranche[] tranches,
        LockupTranched.Timestamps timestamps,
        address broker
    );
}
