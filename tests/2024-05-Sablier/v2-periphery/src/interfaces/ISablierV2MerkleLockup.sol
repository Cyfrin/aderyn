// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IAdminable } from "@sablier/v2-core/src/interfaces/IAdminable.sol";

/// @title ISablierV2MerkleLockup
/// @notice A contract that lets user claim Sablier streams using Merkle proofs. A popular use case for MerkleLockup
/// is airstreams: a portmanteau of "airdrop" and "stream". This is an airdrop model where the tokens are distributed
/// over time, as opposed to all at once.
/// @dev This is the base interface for MerkleLockup. See the Sablier docs for more guidance: https://docs.sablier.com
interface ISablierV2MerkleLockup is IAdminable {
    /*//////////////////////////////////////////////////////////////////////////
                                       EVENTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a recipient claims a stream.
    event Claim(uint256 index, address indexed recipient, uint128 amount, uint256 indexed streamId);

    /// @notice Emitted when the admin claws back the unclaimed tokens.
    event Clawback(address indexed admin, address indexed to, uint128 amount);

    /*//////////////////////////////////////////////////////////////////////////
                                 CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice The ERC-20 asset to distribute.
    /// @dev This is an immutable state variable.
    function ASSET() external returns (IERC20);

    /// @notice A flag indicating whether the streams can be canceled.
    /// @dev This is an immutable state variable.
    function CANCELABLE() external returns (bool);

    /// @notice The cut-off point for the campaign, as a Unix timestamp. A value of zero means there is no expiration.
    /// @dev This is an immutable state variable.
    function EXPIRATION() external returns (uint40);

    /// @notice Returns the timestamp when the first claim is made.
    function getFirstClaimTime() external view returns (uint40);

    /// @notice Returns a flag indicating whether a claim has been made for a given index.
    /// @dev Uses a bitmap to save gas.
    /// @param index The index of the recipient to check.
    function hasClaimed(uint256 index) external returns (bool);

    /// @notice Returns a flag indicating whether the campaign has expired.
    function hasExpired() external view returns (bool);

    /// @notice The content identifier for indexing the campaign on IPFS.
    function ipfsCID() external view returns (string memory);

    /// @notice The root of the Merkle tree used to validate the proofs of inclusion.
    /// @dev This is an immutable state variable.
    function MERKLE_ROOT() external returns (bytes32);

    /// @notice Retrieves the name of the campaign.
    function name() external returns (string memory);

    /// @notice A flag indicating whether the stream NFTs are transferable.
    /// @dev This is an immutable state variable.
    function TRANSFERABLE() external returns (bool);

    /*//////////////////////////////////////////////////////////////////////////
                               NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @notice Claws back the unclaimed tokens from the campaign.
    ///
    /// @dev Emits a {Clawback} event.
    ///
    /// Requirements:
    /// - The caller must be the admin.
    /// - No claim must be made, OR
    ///   The current timestamp must not exceed 7 days after the first claim, OR
    ///   The campaign must be expired.
    ///
    /// @param to The address to receive the tokens.
    /// @param amount The amount of tokens to claw back.
    function clawback(address to, uint128 amount) external;
}
