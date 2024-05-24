// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { WithdrawMaxAndTransfer_Integration_Shared_Test } from "../../shared/lockup/withdrawMaxAndTransfer.t.sol";
import { Integration_Test } from "../../Integration.t.sol";

abstract contract WithdrawMaxAndTransfer_Integration_Fuzz_Test is
    Integration_Test,
    WithdrawMaxAndTransfer_Integration_Shared_Test
{
    function setUp() public virtual override(Integration_Test, WithdrawMaxAndTransfer_Integration_Shared_Test) {
        WithdrawMaxAndTransfer_Integration_Shared_Test.setUp();
    }

    /// @dev Given enough fuzz runs, all of the following scenarios will be fuzzed:
    ///
    /// - New recipient same and different from the current one
    /// - Withdrawable amount zero and not zero
    function testFuzz_WithdrawMaxAndTransfer(
        uint256 timeJump,
        address newRecipient
    )
        external
        whenNotDelegateCalled
        givenNotNull
        whenCallerCurrentRecipient
        givenNFTNotBurned
        givenStreamTransferable
    {
        vm.assume(newRecipient != address(0));
        timeJump = _bound(timeJump, 0, defaults.TOTAL_DURATION() * 2);

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });

        // Get the withdraw amount.
        uint128 withdrawAmount = lockup.withdrawableAmountOf(defaultStreamId);

        if (withdrawAmount > 0) {
            // Expect the assets to be transferred to the fuzzed recipient.
            expectCallToTransfer({ to: users.recipient, value: withdrawAmount });

            // Expect the relevant event to be emitted.
            vm.expectEmit({ emitter: address(lockup) });
            emit WithdrawFromLockupStream({
                streamId: defaultStreamId,
                to: users.recipient,
                asset: dai,
                amount: withdrawAmount
            });
        }

        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit Transfer({ from: users.recipient, to: newRecipient, tokenId: defaultStreamId });

        // Make the max withdrawal and transfer the NFT.
        lockup.withdrawMaxAndTransfer({ streamId: defaultStreamId, newRecipient: newRecipient });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = withdrawAmount;
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");

        // Assert that the fuzzed recipient is the new stream recipient (and NFT owner).
        address actualRecipient = lockup.getRecipient(defaultStreamId);
        address expectedRecipient = newRecipient;
        assertEq(actualRecipient, expectedRecipient, "recipient");
    }
}
