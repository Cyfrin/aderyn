// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Address } from "@openzeppelin/contracts/utils/Address.sol";
import { IERC721Errors } from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { UD60x18, ud, ZERO } from "@prb/math/src/UD60x18.sol";
import { stdError } from "forge-std/src/StdError.sol";

import { ISablierV2LockupTranched } from "src/interfaces/ISablierV2LockupTranched.sol";
import { Errors } from "src/libraries/Errors.sol";
import { Broker, Lockup, LockupTranched } from "src/types/DataTypes.sol";

import { CreateWithTimestamps_Integration_Shared_Test } from "../../../shared/lockup/createWithTimestamps.t.sol";
import { LockupTranched_Integration_Concrete_Test } from "../LockupTranched.t.sol";

contract CreateWithTimestamps_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    CreateWithTimestamps_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, CreateWithTimestamps_Integration_Shared_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        CreateWithTimestamps_Integration_Shared_Test.setUp();
    }

    function test_RevertWhen_DelegateCalled() external {
        bytes memory callData =
            abi.encodeCall(ISablierV2LockupTranched.createWithTimestamps, defaults.createWithTimestampsLT());
        (bool success, bytes memory returnData) = address(lockupTranched).delegatecall(callData);
        expectRevertDueToDelegateCall(success, returnData);
    }

    function test_RevertWhen_RecipientZeroAddress() external whenNotDelegateCalled {
        address recipient = address(0);
        vm.expectRevert(abi.encodeWithSelector(IERC721Errors.ERC721InvalidReceiver.selector, recipient));
        createDefaultStreamWithRecipient(recipient);
    }

    function test_RevertWhen_DepositAmountZero() external whenNotDelegateCalled whenRecipientNonZeroAddress {
        // It is not possible to obtain a zero deposit amount from a non-zero total amount, because the `MAX_BROKER_FEE`
        // is hard coded to 10%.
        vm.expectRevert(Errors.SablierV2Lockup_DepositAmountZero.selector);
        uint128 totalAmount = 0;
        createDefaultStreamWithTotalAmount(totalAmount);
    }

    function test_RevertWhen_StartTimeZero()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
    {
        vm.expectRevert(Errors.SablierV2Lockup_StartTimeZero.selector);
        createDefaultStreamWithStartTime(0);
    }

    function test_RevertWhen_TrancheCountZero()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
    {
        LockupTranched.Tranche[] memory tranches;
        vm.expectRevert(Errors.SablierV2LockupTranched_TrancheCountZero.selector);
        createDefaultStreamWithTranches(tranches);
    }

    function test_RevertWhen_TrancheCountTooHigh()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
    {
        uint256 trancheCount = defaults.MAX_TRANCHE_COUNT() + 1;
        LockupTranched.Tranche[] memory tranches = new LockupTranched.Tranche[](trancheCount);
        vm.expectRevert(
            abi.encodeWithSelector(Errors.SablierV2LockupTranched_TrancheCountTooHigh.selector, trancheCount)
        );
        createDefaultStreamWithTranches(tranches);
    }

    function test_RevertWhen_TrancheAmountsSumOverflows()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
    {
        LockupTranched.Tranche[] memory tranches = defaults.tranches();
        tranches[0].amount = MAX_UINT128;
        tranches[1].amount = 1;
        vm.expectRevert(stdError.arithmeticError);
        createDefaultStreamWithTranches(tranches);
    }

    function test_RevertWhen_StartTimeGreaterThanFirstTrancheTimestamp()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
    {
        // Change the timestamp of the first tranche.
        LockupTranched.Tranche[] memory tranches = defaults.tranches();
        tranches[0].timestamp = defaults.START_TIME() - 1 seconds;

        // Expect the relevant error to be thrown.
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupTranched_StartTimeNotLessThanFirstTrancheTimestamp.selector,
                defaults.START_TIME(),
                tranches[0].timestamp
            )
        );

        // Create the stream.
        createDefaultStreamWithTranches(tranches);
    }

    function test_RevertWhen_StartTimeEqualToFirstTrancheTimestamp()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
    {
        // Change the timestamp of the first tranche.
        LockupTranched.Tranche[] memory tranches = defaults.tranches();
        tranches[0].timestamp = defaults.START_TIME();

        // Expect the relevant error to be thrown.
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupTranched_StartTimeNotLessThanFirstTrancheTimestamp.selector,
                defaults.START_TIME(),
                tranches[0].timestamp
            )
        );

        // Create the stream.
        createDefaultStreamWithTranches(tranches);
    }

    function test_RevertWhen_TrancheTimestampsNotOrdered()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstTrancheTimestamp
    {
        // Swap the tranche timestamps.
        LockupTranched.Tranche[] memory tranches = defaults.tranches();
        (tranches[0].timestamp, tranches[1].timestamp) = (tranches[1].timestamp, tranches[0].timestamp);

        // Expect the relevant error to be thrown.
        uint256 index = 1;
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupTranched_TrancheTimestampsNotOrdered.selector,
                index,
                tranches[0].timestamp,
                tranches[1].timestamp
            )
        );

        // Create the stream.
        createDefaultStreamWithTranches(tranches);
    }

    function test_RevertGiven_EndTimeNotInTheFuture()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstTrancheTimestamp
        whenTrancheTimestampsOrdered
    {
        uint40 endTime = defaults.END_TIME();
        vm.warp({ newTimestamp: endTime });
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_EndTimeNotInTheFuture.selector, endTime, endTime));
        createDefaultStream();
    }

    function test_RevertWhen_DepositAmountNotEqualToTrancheAmountsSum()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstTrancheTimestamp
        whenTrancheTimestampsOrdered
        whenEndTimeInTheFuture
    {
        UD60x18 brokerFee = ZERO;
        resetPrank({ msgSender: users.sender });

        // Adjust the default deposit amount.
        uint128 defaultDepositAmount = defaults.DEPOSIT_AMOUNT();
        uint128 depositAmount = defaultDepositAmount + 100;

        // Prepare the params.
        LockupTranched.CreateWithTimestamps memory params = defaults.createWithTimestampsLT();
        params.broker = Broker({ account: address(0), fee: brokerFee });
        params.totalAmount = depositAmount;

        // Expect the relevant error to be thrown.
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupTranched_DepositAmountNotEqualToTrancheAmountsSum.selector,
                depositAmount,
                defaultDepositAmount
            )
        );

        // Create the stream.
        lockupTranched.createWithTimestamps(params);
    }

    function test_RevertWhen_BrokerFeeTooHigh()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstTrancheTimestamp
        whenTrancheTimestampsOrdered
        whenEndTimeInTheFuture
        whenDepositAmountEqualToTrancheAmountsSum
    {
        UD60x18 brokerFee = MAX_BROKER_FEE + ud(1);
        vm.expectRevert(
            abi.encodeWithSelector(Errors.SablierV2Lockup_BrokerFeeTooHigh.selector, brokerFee, MAX_BROKER_FEE)
        );
        createDefaultStreamWithBroker(Broker({ account: users.broker, fee: brokerFee }));
    }

    function test_RevertWhen_AssetNotContract()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstTrancheTimestamp
        whenTrancheTimestampsOrdered
        whenEndTimeInTheFuture
        whenDepositAmountEqualToTrancheAmountsSum
        whenBrokerFeeNotTooHigh
    {
        address nonContract = address(8128);

        resetPrank({ msgSender: users.sender });

        // Run the test.
        vm.expectRevert(abi.encodeWithSelector(Address.AddressEmptyCode.selector, nonContract));
        createDefaultStreamWithAsset(IERC20(nonContract));
    }

    function test_CreateWithTimestamps_AssetMissingReturnValue()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstTrancheTimestamp
        whenTrancheTimestampsOrdered
        whenEndTimeInTheFuture
        whenDepositAmountEqualToTrancheAmountsSum
        whenBrokerFeeNotTooHigh
        whenAssetContract
    {
        testCreateWithTimestamps(address(usdt));
    }

    function test_CreateWithTimestamps()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenTrancheCountNotZero
        whenTrancheCountNotTooHigh
        whenTrancheAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstTrancheTimestamp
        whenTrancheTimestampsOrdered
        whenEndTimeInTheFuture
        whenDepositAmountEqualToTrancheAmountsSum
        whenBrokerFeeNotTooHigh
        whenAssetContract
        whenAssetERC20
    {
        testCreateWithTimestamps(address(dai));
    }

    /// @dev Shared logic between {test_CreateWithTimestamps_AssetMissingReturnValue} and {test_CreateWithTimestamps}.
    function testCreateWithTimestamps(address asset) internal {
        // Make the Sender the stream's funder.
        address funder = users.sender;

        // Expect the assets to be transferred from the funder to {SablierV2LockupTranched}.
        expectCallToTransferFrom({
            asset: IERC20(asset),
            from: funder,
            to: address(lockupTranched),
            value: defaults.DEPOSIT_AMOUNT()
        });

        // Expect the broker fee to be paid to the broker.
        expectCallToTransferFrom({
            asset: IERC20(asset),
            from: funder,
            to: users.broker,
            value: defaults.BROKER_FEE_AMOUNT()
        });

        // Expect the relevant events to be emitted.
        vm.expectEmit({ emitter: address(lockupTranched) });
        emit MetadataUpdate({ _tokenId: streamId });
        vm.expectEmit({ emitter: address(lockupTranched) });
        emit CreateLockupTranchedStream({
            streamId: streamId,
            funder: funder,
            sender: users.sender,
            recipient: users.recipient,
            amounts: defaults.lockupCreateAmounts(),
            tranches: defaults.tranches(),
            asset: IERC20(asset),
            cancelable: true,
            transferable: true,
            timestamps: defaults.lockupTranchedTimestamps(),
            broker: users.broker
        });

        // Create the stream.
        streamId = createDefaultStreamWithAsset(IERC20(asset));

        // Assert that the stream has been created.
        LockupTranched.StreamLT memory actualStream = lockupTranched.getStream(streamId);
        LockupTranched.StreamLT memory expectedStream = defaults.lockupTranchedStream();
        expectedStream.asset = IERC20(asset);
        assertEq(actualStream, expectedStream);

        // Assert that the stream's status is "PENDING".
        Lockup.Status actualStatus = lockupTranched.statusOf(streamId);
        Lockup.Status expectedStatus = Lockup.Status.PENDING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the next stream ID has been bumped.
        uint256 actualNextStreamId = lockupTranched.nextStreamId();
        uint256 expectedNextStreamId = streamId + 1;
        assertEq(actualNextStreamId, expectedNextStreamId, "nextStreamId");

        // Assert that the NFT has been minted.
        address actualNFTOwner = lockupTranched.ownerOf({ tokenId: streamId });
        address expectedNFTOwner = users.recipient;
        assertEq(actualNFTOwner, expectedNFTOwner, "NFT owner");
    }
}
