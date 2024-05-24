// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup, LockupTranched } from "src/types/DataTypes.sol";

import { Withdraw_Integration_Fuzz_Test } from "../lockup/withdraw.t.sol";
import { LockupTranched_Integration_Fuzz_Test } from "./LockupTranched.t.sol";

/// @dev This contract complements the tests in {Withdraw_Integration_Fuzz_Test} by testing the withdraw function
/// against
/// streams created with fuzzed tranches.
contract Withdraw_LockupTranched_Integration_Fuzz_Test is
    LockupTranched_Integration_Fuzz_Test,
    Withdraw_Integration_Fuzz_Test
{
    function setUp() public virtual override(LockupTranched_Integration_Fuzz_Test, Withdraw_Integration_Fuzz_Test) {
        LockupTranched_Integration_Fuzz_Test.setUp();
        Withdraw_Integration_Fuzz_Test.setUp();
    }

    struct Params {
        LockupTranched.Tranche[] tranches;
        uint256 timeJump;
        address to;
    }

    struct Vars {
        Lockup.Status actualStatus;
        uint256 actualWithdrawnAmount;
        Lockup.CreateAmounts createAmounts;
        Lockup.Status expectedStatus;
        uint256 expectedWithdrawnAmount;
        bool isDepleted;
        bool isSettled;
        address funder;
        uint256 streamId;
        uint128 totalAmount;
        uint40 totalDuration;
        uint128 withdrawAmount;
        uint128 withdrawableAmount;
    }

    function testFuzz_Withdraw_TrancheFuzzing(Params memory params)
        external
        whenNotDelegateCalled
        givenNotNull
        whenToNonZeroAddress
        whenWithdrawAmountNotZero
        whenNoOverdraw
    {
        vm.assume(params.tranches.length != 0);
        vm.assume(params.to != address(0));

        // Make the Sender the stream's funder (recall that the Sender is the default caller).
        Vars memory vars;
        vars.funder = users.sender;

        // Fuzz the tranche timestamps.
        fuzzTrancheTimestamps(params.tranches, defaults.START_TIME());

        // Fuzz the tranche amounts.
        (vars.totalAmount, vars.createAmounts) = fuzzTranchedStreamAmounts(params.tranches);

        // Bound the time jump.
        vars.totalDuration = params.tranches[params.tranches.length - 1].timestamp - defaults.START_TIME();
        params.timeJump = _bound(params.timeJump, 1 seconds, vars.totalDuration + 100 seconds);

        // Mint enough assets to the funder.
        deal({ token: address(dai), to: vars.funder, give: vars.totalAmount });

        // Make the Sender the caller.
        resetPrank({ msgSender: users.sender });

        // Create the stream with the fuzzed tranches.
        LockupTranched.CreateWithTimestamps memory createParams = defaults.createWithTimestampsLT();
        createParams.totalAmount = vars.totalAmount;
        createParams.tranches = params.tranches;

        vars.streamId = lockupTranched.createWithTimestamps(createParams);

        // Simulate the passage of time.
        vm.warp({ newTimestamp: defaults.START_TIME() + params.timeJump });

        // Query the withdrawable amount.
        vars.withdrawableAmount = lockupTranched.withdrawableAmountOf(vars.streamId);

        // Halt the test if the withdraw amount is zero.
        if (vars.withdrawableAmount == 0) {
            return;
        }

        // Bound the withdraw amount.
        vars.withdrawAmount = boundUint128(vars.withdrawAmount, 1, vars.withdrawableAmount);

        // Make the Recipient the caller.
        resetPrank({ msgSender: users.recipient });

        // Expect the assets to be transferred to the fuzzed `to` address.
        expectCallToTransfer({ to: params.to, value: vars.withdrawAmount });

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockupTranched) });
        emit WithdrawFromLockupStream({ streamId: vars.streamId, to: params.to, asset: dai, amount: vars.withdrawAmount });
        vm.expectEmit({ emitter: address(lockupTranched) });
        emit MetadataUpdate({ _tokenId: vars.streamId });

        // Make the withdrawal.
        lockupTranched.withdraw({ streamId: vars.streamId, to: params.to, amount: vars.withdrawAmount });

        // Check if the stream is depleted or settled. It is possible for the stream to be just settled
        // and not depleted because the withdraw amount is fuzzed.
        vars.isDepleted = vars.withdrawAmount == vars.createAmounts.deposit;
        vars.isSettled = lockupTranched.refundableAmountOf(vars.streamId) == 0;

        // Assert that the stream's status is correct.
        vars.actualStatus = lockupTranched.statusOf(vars.streamId);
        if (vars.isDepleted) {
            vars.expectedStatus = Lockup.Status.DEPLETED;
        } else if (vars.isSettled) {
            vars.expectedStatus = Lockup.Status.SETTLED;
        } else {
            vars.expectedStatus = Lockup.Status.STREAMING;
        }
        assertEq(vars.actualStatus, vars.expectedStatus);

        // Assert that the withdrawn amount has been updated.
        vars.actualWithdrawnAmount = lockupTranched.getWithdrawnAmount(vars.streamId);
        vars.expectedWithdrawnAmount = vars.withdrawAmount;
        assertEq(vars.actualWithdrawnAmount, vars.expectedWithdrawnAmount, "withdrawnAmount");
    }
}
