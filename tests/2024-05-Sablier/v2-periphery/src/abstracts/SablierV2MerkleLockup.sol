// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { MerkleProof } from "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import { BitMaps } from "@openzeppelin/contracts/utils/structs/BitMaps.sol";
import { Adminable } from "@sablier/v2-core/src/abstracts/Adminable.sol";

import { ISablierV2MerkleLockup } from "../interfaces/ISablierV2MerkleLockup.sol";
import { MerkleLockup } from "../types/DataTypes.sol";
import { Errors } from "../libraries/Errors.sol";

/// @title SablierV2MerkleLockup
/// @notice See the documentation in {ISablierV2MerkleLockup}.
abstract contract SablierV2MerkleLockup is
    ISablierV2MerkleLockup, // 2 inherited component
    Adminable // 1 inherited component
{
    using BitMaps for BitMaps.BitMap;
    using SafeERC20 for IERC20;

    /*//////////////////////////////////////////////////////////////////////////
                                  STATE VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2MerkleLockup
    IERC20 public immutable override ASSET;

    /// @inheritdoc ISablierV2MerkleLockup
    bool public immutable override CANCELABLE;

    /// @inheritdoc ISablierV2MerkleLockup
    uint40 public immutable override EXPIRATION;

    /// @inheritdoc ISablierV2MerkleLockup
    bytes32 public immutable override MERKLE_ROOT;

    /// @dev The name of the campaign stored as bytes32.
    bytes32 internal immutable NAME;

    /// @inheritdoc ISablierV2MerkleLockup
    bool public immutable override TRANSFERABLE;

    /// @inheritdoc ISablierV2MerkleLockup
    string public ipfsCID;

    /// @dev Packed booleans that record the history of claims.
    BitMaps.BitMap internal _claimedBitMap;

    /// @dev The timestamp when the first claim is made.
    uint40 internal _firstClaimTime;

    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Constructs the contract by initializing the immutable state variables.
    constructor(MerkleLockup.ConstructorParams memory params) {
        // Check: the campaign name is not greater than 32 bytes
        if (bytes(params.name).length > 32) {
            revert Errors.SablierV2MerkleLockup_CampaignNameTooLong({
                nameLength: bytes(params.name).length,
                maxLength: 32
            });
        }

        admin = params.initialAdmin;
        ASSET = params.asset;
        CANCELABLE = params.cancelable;
        EXPIRATION = params.expiration;
        ipfsCID = params.ipfsCID;
        MERKLE_ROOT = params.merkleRoot;
        NAME = bytes32(abi.encodePacked(params.name));
        TRANSFERABLE = params.transferable;
    }

    /*//////////////////////////////////////////////////////////////////////////
                           USER-FACING CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2MerkleLockup
    function getFirstClaimTime() external view override returns (uint40) {
        return _firstClaimTime;
    }

    /// @inheritdoc ISablierV2MerkleLockup
    function hasClaimed(uint256 index) public view override returns (bool) {
        return _claimedBitMap.get(index);
    }

    /// @inheritdoc ISablierV2MerkleLockup
    function hasExpired() public view override returns (bool) {
        return EXPIRATION > 0 && EXPIRATION <= block.timestamp;
    }

    /// @inheritdoc ISablierV2MerkleLockup
    function name() external view override returns (string memory) {
        return string(abi.encodePacked(NAME));
    }

    /*//////////////////////////////////////////////////////////////////////////
                         USER-FACING NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2MerkleLockup
    function clawback(address to, uint128 amount) external override onlyAdmin {
        // Check: current timestamp is over the grace period and the campaign has not expired.
        if (_hasGracePeriodPassed() && !hasExpired()) {
            revert Errors.SablierV2MerkleLockup_ClawbackNotAllowed({
                blockTimestamp: block.timestamp,
                expiration: EXPIRATION,
                firstClaimTime: _firstClaimTime
            });
        }

        // Effect: transfer the tokens to the provided address.
        ASSET.safeTransfer(to, amount);

        // Log the clawback.
        emit Clawback(admin, to, amount);
    }

    /*//////////////////////////////////////////////////////////////////////////
                            INTERNAL CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Validates the parameters of the `claim` function, which is implemented by child contracts.
    function _checkClaim(uint256 index, bytes32 leaf, bytes32[] calldata merkleProof) internal {
        // Check: the campaign has not expired.
        if (hasExpired()) {
            revert Errors.SablierV2MerkleLockup_CampaignExpired({
                blockTimestamp: block.timestamp,
                expiration: EXPIRATION
            });
        }

        // Check: the index has not been claimed.
        if (_claimedBitMap.get(index)) {
            revert Errors.SablierV2MerkleLockup_StreamClaimed(index);
        }

        // Check: the input claim is included in the Merkle tree.
        if (!MerkleProof.verify(merkleProof, MERKLE_ROOT, leaf)) {
            revert Errors.SablierV2MerkleLockup_InvalidProof();
        }

        // Effect: set the `_firstClaimTime` if its zero.
        if (_firstClaimTime == 0) {
            _firstClaimTime = uint40(block.timestamp);
        }
    }

    /// @notice Returns a flag indicating whether the grace period has passed.
    /// @dev The grace period is 7 days after the first claim.
    function _hasGracePeriodPassed() internal view returns (bool) {
        return _firstClaimTime > 0 && block.timestamp > _firstClaimTime + 7 days;
    }
}
