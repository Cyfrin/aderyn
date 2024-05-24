// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Address } from "@openzeppelin/contracts/utils/Address.sol";
import { IERC721Errors } from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { UD60x18, ud, ZERO } from "@prb/math/src/UD60x18.sol";
import { stdError } from "forge-std/src/StdError.sol";

import { ISablierV2LockupDynamic } from "src/interfaces/ISablierV2LockupDynamic.sol";
import { Errors } from "src/libraries/Errors.sol";
import { Broker, Lockup, LockupDynamic } from "src/types/DataTypes.sol";

import { CreateWithTimestamps_Integration_Shared_Test } from "../../../shared/lockup/createWithTimestamps.t.sol";
import { LockupDynamic_Integration_Concrete_Test } from "../LockupDynamic.t.sol";

contract CreateWithTimestamps_LockupDynamic_Integration_Concrete_Test is
    LockupDynamic_Integration_Concrete_Test,
    CreateWithTimestamps_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupDynamic_Integration_Concrete_Test, CreateWithTimestamps_Integration_Shared_Test)
    {
        LockupDynamic_Integration_Concrete_Test.setUp();
        CreateWithTimestamps_Integration_Shared_Test.setUp();
    }

    function test_RevertWhen_DelegateCalled() external {
        bytes memory callData =
            abi.encodeCall(ISablierV2LockupDynamic.createWithTimestamps, defaults.createWithTimestampsLD());
        (bool success, bytes memory returnData) = address(lockupDynamic).delegatecall(callData);
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

    function test_RevertWhen_SegmentCountZero()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
    {
        LockupDynamic.Segment[] memory segments;
        vm.expectRevert(Errors.SablierV2LockupDynamic_SegmentCountZero.selector);
        createDefaultStreamWithSegments(segments);
    }

    function test_RevertWhen_SegmentCountTooHigh()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenSegmentCountNotZero
    {
        uint256 segmentCount = defaults.MAX_SEGMENT_COUNT() + 1;
        LockupDynamic.Segment[] memory segments = new LockupDynamic.Segment[](segmentCount);
        vm.expectRevert(
            abi.encodeWithSelector(Errors.SablierV2LockupDynamic_SegmentCountTooHigh.selector, segmentCount)
        );
        createDefaultStreamWithSegments(segments);
    }

    function test_RevertWhen_SegmentAmountsSumOverflows()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenSegmentCountNotZero
        whenSegmentCountNotTooHigh
    {
        LockupDynamic.Segment[] memory segments = defaults.segments();
        segments[0].amount = MAX_UINT128;
        segments[1].amount = 1;
        vm.expectRevert(stdError.arithmeticError);
        createDefaultStreamWithSegments(segments);
    }

    function test_RevertWhen_StartTimeGreaterThanFirstSegmentTimestamp()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenSegmentCountNotZero
        whenSegmentCountNotTooHigh
        whenSegmentAmountsSumDoesNotOverflow
    {
        // Change the timestamp of the first segment.
        LockupDynamic.Segment[] memory segments = defaults.segments();
        segments[0].timestamp = defaults.START_TIME() - 1 seconds;

        // Expect the relevant error to be thrown.
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupDynamic_StartTimeNotLessThanFirstSegmentTimestamp.selector,
                defaults.START_TIME(),
                segments[0].timestamp
            )
        );

        // Create the stream.
        createDefaultStreamWithSegments(segments);
    }

    function test_RevertWhen_StartTimeEqualToFirstSegmentTimestamp()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenSegmentCountNotZero
        whenSegmentCountNotTooHigh
        whenSegmentAmountsSumDoesNotOverflow
    {
        // Change the timestamp of the first segment.
        LockupDynamic.Segment[] memory segments = defaults.segments();
        segments[0].timestamp = defaults.START_TIME();

        // Expect the relevant error to be thrown.
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupDynamic_StartTimeNotLessThanFirstSegmentTimestamp.selector,
                defaults.START_TIME(),
                segments[0].timestamp
            )
        );

        // Create the stream.
        createDefaultStreamWithSegments(segments);
    }

    function test_RevertWhen_SegmentTimestampsNotOrdered()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenSegmentCountNotZero
        whenSegmentCountNotTooHigh
        whenSegmentAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstSegmentTimestamp
    {
        // Swap the segment timestamps.
        LockupDynamic.Segment[] memory segments = defaults.segments();
        (segments[0].timestamp, segments[1].timestamp) = (segments[1].timestamp, segments[0].timestamp);

        // Expect the relevant error to be thrown.
        uint256 index = 1;
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupDynamic_SegmentTimestampsNotOrdered.selector,
                index,
                segments[0].timestamp,
                segments[1].timestamp
            )
        );

        // Create the stream.
        createDefaultStreamWithSegments(segments);
    }

    function test_RevertGiven_EndTimeNotInTheFuture()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenSegmentCountNotZero
        whenSegmentCountNotTooHigh
        whenSegmentAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstSegmentTimestamp
        whenSegmentTimestampsOrdered
    {
        uint40 endTime = defaults.END_TIME();
        vm.warp({ newTimestamp: endTime });
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_EndTimeNotInTheFuture.selector, endTime, endTime));
        createDefaultStream();
    }

    function test_RevertWhen_DepositAmountNotEqualToSegmentAmountsSum()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenSegmentCountNotZero
        whenSegmentCountNotTooHigh
        whenSegmentAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstSegmentTimestamp
        whenSegmentTimestampsOrdered
        whenEndTimeInTheFuture
    {
        UD60x18 brokerFee = ZERO;
        resetPrank({ msgSender: users.sender });

        // Adjust the default deposit amount.
        uint128 defaultDepositAmount = defaults.DEPOSIT_AMOUNT();
        uint128 depositAmount = defaultDepositAmount + 100;

        // Prepare the params.
        LockupDynamic.CreateWithTimestamps memory params = defaults.createWithTimestampsLD();
        params.broker = Broker({ account: address(0), fee: brokerFee });
        params.totalAmount = depositAmount;

        // Expect the relevant error to be thrown.
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupDynamic_DepositAmountNotEqualToSegmentAmountsSum.selector,
                depositAmount,
                defaultDepositAmount
            )
        );

        // Create the stream.
        lockupDynamic.createWithTimestamps(params);
    }

    function test_RevertWhen_BrokerFeeTooHigh()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenSegmentCountNotZero
        whenSegmentCountNotTooHigh
        whenSegmentAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstSegmentTimestamp
        whenSegmentTimestampsOrdered
        whenEndTimeInTheFuture
        whenDepositAmountEqualToSegmentAmountsSum
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
        whenSegmentCountNotZero
        whenSegmentCountNotTooHigh
        whenSegmentAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstSegmentTimestamp
        whenSegmentTimestampsOrdered
        whenEndTimeInTheFuture
        whenDepositAmountEqualToSegmentAmountsSum
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
        whenSegmentCountNotZero
        whenSegmentCountNotTooHigh
        whenSegmentAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstSegmentTimestamp
        whenSegmentTimestampsOrdered
        whenEndTimeInTheFuture
        whenDepositAmountEqualToSegmentAmountsSum
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
        whenSegmentCountNotZero
        whenSegmentCountNotTooHigh
        whenSegmentAmountsSumDoesNotOverflow
        whenStartTimeLessThanFirstSegmentTimestamp
        whenSegmentTimestampsOrdered
        whenEndTimeInTheFuture
        whenDepositAmountEqualToSegmentAmountsSum
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

        // Expect the assets to be transferred from the funder to {SablierV2LockupDynamic}.
        expectCallToTransferFrom({
            asset: IERC20(asset),
            from: funder,
            to: address(lockupDynamic),
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
        vm.expectEmit({ emitter: address(lockupDynamic) });
        emit MetadataUpdate({ _tokenId: streamId });
        vm.expectEmit({ emitter: address(lockupDynamic) });
        emit CreateLockupDynamicStream({
            streamId: streamId,
            funder: funder,
            sender: users.sender,
            recipient: users.recipient,
            amounts: defaults.lockupCreateAmounts(),
            segments: defaults.segments(),
            asset: IERC20(asset),
            cancelable: true,
            transferable: true,
            timestamps: defaults.lockupDynamicTimestamps(),
            broker: users.broker
        });

        // Create the stream.
        streamId = createDefaultStreamWithAsset(IERC20(asset));

        // Assert that the stream has been created.
        LockupDynamic.StreamLD memory actualStream = lockupDynamic.getStream(streamId);
        LockupDynamic.StreamLD memory expectedStream = defaults.lockupDynamicStream();
        expectedStream.asset = IERC20(asset);
        assertEq(actualStream, expectedStream);

        // Assert that the stream's status is "PENDING".
        Lockup.Status actualStatus = lockupDynamic.statusOf(streamId);
        Lockup.Status expectedStatus = Lockup.Status.PENDING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the next stream ID has been bumped.
        uint256 actualNextStreamId = lockupDynamic.nextStreamId();
        uint256 expectedNextStreamId = streamId + 1;
        assertEq(actualNextStreamId, expectedNextStreamId, "nextStreamId");

        // Assert that the NFT has been minted.
        address actualNFTOwner = lockupDynamic.ownerOf({ tokenId: streamId });
        address expectedNFTOwner = users.recipient;
        assertEq(actualNFTOwner, expectedNFTOwner, "NFT owner");
    }
}
