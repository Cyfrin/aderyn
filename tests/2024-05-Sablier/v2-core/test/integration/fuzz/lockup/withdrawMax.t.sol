// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup } from "src/types/DataTypes.sol";

import { WithdrawMax_Integration_Shared_Test } from "../../shared/lockup/withdrawMax.t.sol";
import { Integration_Test } from "../../Integration.t.sol";

abstract contract WithdrawMax_Integration_Fuzz_Test is Integration_Test, WithdrawMax_Integration_Shared_Test {
    function setUp() public virtual override(Integration_Test, WithdrawMax_Integration_Shared_Test) {
        WithdrawMax_Integration_Shared_Test.setUp();
    }

    function testFuzz_WithdrawMax_EndTimeNotInTheFuture(uint256 timeJump) external {
        timeJump = _bound(timeJump, defaults.TOTAL_DURATION(), defaults.TOTAL_DURATION() * 2);

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });

        // Expect the ERC-20 assets to be transferred to the Recipient.
        expectCallToTransfer({ to: users.recipient, value: defaults.DEPOSIT_AMOUNT() });

        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit WithdrawFromLockupStream({
            streamId: defaultStreamId,
            to: users.recipient,
            asset: dai,
            amount: defaults.DEPOSIT_AMOUNT()
        });

        // Make the max withdrawal.
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = defaults.DEPOSIT_AMOUNT();
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");

        // Assert that the stream's status is "DEPLETED".
        Lockup.Status actualStatus = lockup.statusOf(defaultStreamId);
        Lockup.Status expectedStatus = Lockup.Status.DEPLETED;
        assertEq(actualStatus, expectedStatus);

        // Assert that the stream is not cancelable anymore.
        bool isCancelable = lockup.isCancelable(defaultStreamId);
        assertFalse(isCancelable, "isCancelable");

        // Assert that the NFT has not been burned.
        address actualNFTowner = lockup.ownerOf({ tokenId: defaultStreamId });
        address expectedNFTOwner = users.recipient;
        assertEq(actualNFTowner, expectedNFTOwner, "NFT owner");
    }

    function testFuzz_WithdrawMax(uint256 timeJump) external givenEndTimeInTheFuture {
        timeJump = _bound(timeJump, defaults.CLIFF_DURATION(), defaults.TOTAL_DURATION() - 1 seconds);

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + timeJump });

        // Get the withdraw amount.
        uint128 withdrawAmount = lockup.withdrawableAmountOf(defaultStreamId);

        // Expect the assets to be transferred to the Recipient.
        expectCallToTransfer({ to: users.recipient, value: withdrawAmount });

        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(lockup) });
        emit WithdrawFromLockupStream({
            streamId: defaultStreamId,
            to: users.recipient,
            asset: dai,
            amount: withdrawAmount
        });

        // Make the max withdrawal.
        lockup.withdrawMax({ streamId: defaultStreamId, to: users.recipient });

        // Assert that the withdrawn amount has been updated.
        uint128 actualWithdrawnAmount = lockup.getWithdrawnAmount(defaultStreamId);
        uint128 expectedWithdrawnAmount = withdrawAmount;
        assertEq(actualWithdrawnAmount, expectedWithdrawnAmount, "withdrawnAmount");
    }
}
