// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup_Integration_Shared_Test } from "./Lockup.t.sol";

abstract contract CreateWithTimestamps_Integration_Shared_Test is Lockup_Integration_Shared_Test {
    uint256 internal streamId;

    function setUp() public virtual override {
        streamId = lockup.nextStreamId();
    }

    modifier whenAssetContract() {
        _;
    }

    modifier whenAssetERC20() {
        _;
    }

    modifier whenBrokerFeeNotTooHigh() {
        _;
    }

    modifier whenCliffTimeGreaterThanZero() {
        _;
    }

    modifier whenCliffTimeLessThanEndTime() {
        _;
    }

    modifier whenCliffTimeZero() {
        _;
    }

    modifier whenDepositAmountEqualToSegmentAmountsSum() {
        _;
    }

    modifier whenDepositAmountEqualToTrancheAmountsSum() {
        _;
    }

    modifier whenDepositAmountNotZero() {
        _;
    }

    modifier whenEndTimeInTheFuture() {
        _;
    }

    modifier whenNotDelegateCalled() {
        _;
    }

    modifier whenRecipientNonZeroAddress() {
        _;
    }

    modifier whenSegmentAmountsSumDoesNotOverflow() {
        _;
    }

    modifier whenSegmentCountNotTooHigh() {
        _;
    }

    modifier whenSegmentCountNotZero() {
        _;
    }

    modifier whenSegmentTimestampsOrdered() {
        _;
    }

    modifier whenStartTimeLessThanEndTime() {
        _;
    }

    modifier whenStartTimeLessThanFirstSegmentTimestamp() {
        _;
    }

    modifier whenStartTimeLessThanFirstTrancheTimestamp() {
        _;
    }

    modifier whenStartTimeNotZero() {
        _;
    }

    modifier whenTrancheAmountsSumDoesNotOverflow() {
        _;
    }

    modifier whenTrancheCountNotTooHigh() {
        _;
    }

    modifier whenTrancheCountNotZero() {
        _;
    }

    modifier whenTrancheTimestampsOrdered() {
        _;
    }
}
