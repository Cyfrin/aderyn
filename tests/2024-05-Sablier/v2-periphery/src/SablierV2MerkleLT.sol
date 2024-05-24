// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22;

import { BitMaps } from "@openzeppelin/contracts/utils/structs/BitMaps.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { UD60x18, ud60x18, ZERO } from "@prb/math/src/UD60x18.sol";
import { ISablierV2LockupTranched } from "@sablier/v2-core/src/interfaces/ISablierV2LockupTranched.sol";
import { Broker, LockupTranched } from "@sablier/v2-core/src/types/DataTypes.sol";

import { SablierV2MerkleLockup } from "./abstracts/SablierV2MerkleLockup.sol";
import { ISablierV2MerkleLT } from "./interfaces/ISablierV2MerkleLT.sol";
import { MerkleLockup, MerkleLT } from "./types/DataTypes.sol";

/// @title SablierV2MerkleLT
/// @notice See the documentation in {ISablierV2MerkleLT}.
contract SablierV2MerkleLT is
    ISablierV2MerkleLT, // 2 inherited components
    SablierV2MerkleLockup // 4 inherited components
{
    using BitMaps for BitMaps.BitMap;
    using SafeERC20 for IERC20;

    /*//////////////////////////////////////////////////////////////////////////
                                  STATE VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2MerkleLT
    ISablierV2LockupTranched public immutable override LOCKUP_TRANCHED;

    /// @dev The tranches with their respective unlock percentages and durations.
    MerkleLT.TrancheWithPercentage[] internal _tranchesWithPercentages;

    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Constructs the contract by initializing the immutable state variables, and max approving the Sablier
    /// contract.
    constructor(
        MerkleLockup.ConstructorParams memory baseParams,
        ISablierV2LockupTranched lockupTranched,
        MerkleLT.TrancheWithPercentage[] memory tranchesWithPercentages
    )
        SablierV2MerkleLockup(baseParams)
    {
        LOCKUP_TRANCHED = lockupTranched;

        // Since Solidity lacks a syntax for copying arrays of structs directly from memory to storage, a manual
        // approach is necessary. See https://github.com/ethereum/solidity/issues/12783.
        uint256 count = tranchesWithPercentages.length;
        for (uint256 i = 0; i < count; ++i) {
            _tranchesWithPercentages.push(tranchesWithPercentages[i]);
        }

        // Max approve the Sablier contract to spend funds from the MerkleLockup contract.
        ASSET.forceApprove(address(LOCKUP_TRANCHED), type(uint256).max);
    }

    /*//////////////////////////////////////////////////////////////////////////
                           USER-FACING CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2MerkleLT
    function getTranchesWithPercentages() external view override returns (MerkleLT.TrancheWithPercentage[] memory) {
        return _tranchesWithPercentages;
    }

    /*//////////////////////////////////////////////////////////////////////////
                         USER-FACING NON-CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISablierV2MerkleLT
    function claim(
        uint256 index,
        address recipient,
        uint128 amount,
        bytes32[] calldata merkleProof
    )
        external
        override
        returns (uint256 streamId)
    {
        // Generate the Merkle tree leaf by hashing the corresponding parameters. Hashing twice prevents second
        // preimage attacks.
        bytes32 leaf = keccak256(bytes.concat(keccak256(abi.encode(index, recipient, amount))));

        // Check: validate the function.
        _checkClaim(index, leaf, merkleProof);

        // Calculate the tranches based on the unlock percentages.
        LockupTranched.TrancheWithDuration[] memory tranches = _calculateTranches(amount);

        // Effect: mark the index as claimed.
        _claimedBitMap.set(index);

        // Interaction: create the stream via {SablierV2LockupTranched}.
        streamId = LOCKUP_TRANCHED.createWithDurations(
            LockupTranched.CreateWithDurations({
                sender: admin,
                recipient: recipient,
                totalAmount: amount,
                asset: ASSET,
                cancelable: CANCELABLE,
                transferable: TRANSFERABLE,
                tranches: tranches,
                broker: Broker({ account: address(0), fee: ZERO })
            })
        );

        // Log the claim.
        emit Claim(index, recipient, amount, streamId);
    }

    /*//////////////////////////////////////////////////////////////////////////
                            INTERNAL CONSTANT FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Calculates the tranches based on the claim amount and the unlock percentages for each tranche.
    function _calculateTranches(uint128 claimAmount)
        internal
        view
        returns (LockupTranched.TrancheWithDuration[] memory tranches)
    {
        // Load the tranches in memory (to save gas).
        MerkleLT.TrancheWithPercentage[] memory tranchesWithPercentages = _tranchesWithPercentages;

        // Declare the variables needed for calculation.
        uint128 calculatedAmountsSum;
        UD60x18 claimAmountUD = ud60x18(claimAmount);
        uint256 trancheCount = tranchesWithPercentages.length;
        tranches = new LockupTranched.TrancheWithDuration[](trancheCount);

        // Iterate over each tranche to calculate its unlock amount.
        for (uint256 i = 0; i < trancheCount; ++i) {
            // Convert the tranche's percentage from the `UD2x18` to the `UD60x18` type.
            UD60x18 percentage = (tranchesWithPercentages[i].unlockPercentage).intoUD60x18();

            // Calculate the tranche's amount by multiplying the claim amount by the unlock percentage.
            uint128 calculatedAmount = claimAmountUD.mul(percentage).intoUint128();

            // Create the tranche with duration.
            tranches[i] = LockupTranched.TrancheWithDuration({
                amount: calculatedAmount,
                duration: tranchesWithPercentages[i].duration
            });

            // Add the calculated tranche amount.
            calculatedAmountsSum += calculatedAmount;
        }

        // It should never be the case that the sum of the calculated amounts is greater than the claim amount because
        // PRBMath always rounds down.
        assert(calculatedAmountsSum <= claimAmount);

        // Since there can be rounding errors, the last tranche amount needs to be adjusted to ensure the sum of all
        // tranche amounts equals the claim amount.
        if (calculatedAmountsSum < claimAmount) {
            unchecked {
                tranches[trancheCount - 1].amount += claimAmount - calculatedAmountsSum;
            }
        }
    }
}
