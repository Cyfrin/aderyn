// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";
import { BatchLockup } from "src/types/DataTypes.sol";

import { Integration_Test } from "../../Integration.t.sol";

contract CreateWithDurationsLT_Integration_Test is Integration_Test {
    function setUp() public virtual override {
        Integration_Test.setUp();
    }

    function test_RevertWhen_BatchSizeZero() external {
        BatchLockup.CreateWithDurationsLT[] memory batchParams = new BatchLockup.CreateWithDurationsLT[](0);
        vm.expectRevert(Errors.SablierV2BatchLockup_BatchSizeZero.selector);
        batchLockup.createWithDurationsLT(lockupTranched, dai, batchParams);
    }

    modifier whenBatchSizeNotZero() {
        _;
    }

    function test_BatchCreateWithDurations() external whenBatchSizeNotZero {
        // Asset flow: Alice → batchLockup → Sablier
        // Expect transfers from Alice to the batchLockup, and then from the batchLockup to the Sablier contract.
        expectCallToTransferFrom({
            from: users.alice,
            to: address(batchLockup),
            amount: defaults.TOTAL_TRANSFER_AMOUNT()
        });
        expectMultipleCallsToCreateWithDurationsLT({
            count: defaults.BATCH_SIZE(),
            params: defaults.createWithDurationsLT()
        });
        expectMultipleCallsToTransferFrom({
            count: defaults.BATCH_SIZE(),
            from: address(batchLockup),
            to: address(lockupTranched),
            amount: defaults.PER_STREAM_AMOUNT()
        });

        // Assert that the batch of streams has been created successfully.
        uint256[] memory actualStreamIds =
            batchLockup.createWithDurationsLT(lockupTranched, dai, defaults.batchCreateWithDurationsLT());
        uint256[] memory expectedStreamIds = defaults.incrementalStreamIds();
        assertEq(actualStreamIds, expectedStreamIds, "stream ids mismatch");
    }
}
