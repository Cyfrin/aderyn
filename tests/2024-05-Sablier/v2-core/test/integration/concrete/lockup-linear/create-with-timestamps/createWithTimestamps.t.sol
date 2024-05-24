// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Address } from "@openzeppelin/contracts/utils/Address.sol";
import { IERC721Errors } from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { UD60x18, ud } from "@prb/math/src/UD60x18.sol";

import { ISablierV2LockupLinear } from "src/interfaces/ISablierV2LockupLinear.sol";
import { Errors } from "src/libraries/Errors.sol";
import { Broker, Lockup, LockupLinear } from "src/types/DataTypes.sol";

import { CreateWithTimestamps_Integration_Shared_Test } from "../../../shared/lockup/createWithTimestamps.t.sol";
import { LockupLinear_Integration_Concrete_Test } from "../LockupLinear.t.sol";

contract CreateWithTimestamps_LockupLinear_Integration_Concrete_Test is
    LockupLinear_Integration_Concrete_Test,
    CreateWithTimestamps_Integration_Shared_Test
{
    function setUp()
        public
        virtual
        override(LockupLinear_Integration_Concrete_Test, CreateWithTimestamps_Integration_Shared_Test)
    {
        LockupLinear_Integration_Concrete_Test.setUp();
        CreateWithTimestamps_Integration_Shared_Test.setUp();
    }

    function test_RevertWhen_DelegateCalled() external {
        bytes memory callData =
            abi.encodeCall(ISablierV2LockupLinear.createWithTimestamps, defaults.createWithTimestampsLL());
        (bool success, bytes memory returnData) = address(lockupLinear).delegatecall(callData);
        expectRevertDueToDelegateCall(success, returnData);
    }

    function test_RevertWhen_RecipientZeroAddress() external whenNotDelegateCalled {
        address recipient = address(0);
        vm.expectRevert(abi.encodeWithSelector(IERC721Errors.ERC721InvalidReceiver.selector, recipient));
        createDefaultStreamWithRecipient(recipient);
    }

    /// @dev It is not possible to obtain a zero deposit amount from a non-zero total amount, because the
    /// `MAX_BROKER_FEE` is hard coded to 10%.
    function test_RevertWhen_DepositAmountZero() external whenNotDelegateCalled whenRecipientNonZeroAddress {
        vm.expectRevert(Errors.SablierV2Lockup_DepositAmountZero.selector);
        createDefaultStreamWithTotalAmount(0);
    }

    function test_RevertWhen_StartTimeZero()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
    {
        uint40 cliffTime = defaults.CLIFF_TIME();
        uint40 endTime = defaults.END_TIME();

        vm.expectRevert(Errors.SablierV2Lockup_StartTimeZero.selector);
        createDefaultStreamWithTimestamps(LockupLinear.Timestamps({ start: 0, cliff: cliffTime, end: endTime }));
    }

    function test_RevertWhen_StartTimeNotLessThanEndTime()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenCliffTimeZero
    {
        uint40 startTime = defaults.END_TIME();
        uint40 endTime = defaults.START_TIME();

        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupLinear_StartTimeNotLessThanEndTime.selector, startTime, endTime
            )
        );
        createDefaultStreamWithTimestamps(LockupLinear.Timestamps({ start: startTime, cliff: 0, end: endTime }));
    }

    function test_CreateWithTimestamps_StartTimeLessThanEndTime()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenCliffTimeZero
    {
        createDefaultStreamWithTimestamps(
            LockupLinear.Timestamps({ start: defaults.START_TIME(), cliff: 0, end: defaults.END_TIME() })
        );

        // Assert that the stream has been created.
        LockupLinear.StreamLL memory actualStream = lockupLinear.getStream(streamId);
        LockupLinear.StreamLL memory expectedStream = defaults.lockupLinearStream();
        expectedStream.cliffTime = 0;
        assertEq(actualStream, expectedStream);

        // Assert that the next stream ID has been bumped.
        uint256 actualNextStreamId = lockupLinear.nextStreamId();
        uint256 expectedNextStreamId = streamId + 1;
        assertEq(actualNextStreamId, expectedNextStreamId, "nextStreamId");

        // Assert that the NFT has been minted.
        address actualNFTOwner = lockupLinear.ownerOf({ tokenId: streamId });
        address expectedNFTOwner = users.recipient;
        assertEq(actualNFTOwner, expectedNFTOwner, "NFT owner");
    }

    function test_RevertWhen_StartTimeGreaterThanCliffTime()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenCliffTimeGreaterThanZero
        whenStartTimeLessThanEndTime
    {
        uint40 startTime = defaults.CLIFF_TIME();
        uint40 cliffTime = defaults.START_TIME();
        uint40 endTime = defaults.END_TIME();
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupLinear_StartTimeNotLessThanCliffTime.selector, startTime, cliffTime
            )
        );
        createDefaultStreamWithTimestamps(LockupLinear.Timestamps({ start: startTime, cliff: cliffTime, end: endTime }));
    }

    function test_RevertWhen_CliffTimeNotLessThanEndTime()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenCliffTimeGreaterThanZero
        whenStartTimeLessThanEndTime
    {
        uint40 startTime = defaults.START_TIME();
        uint40 cliffTime = defaults.END_TIME();
        uint40 endTime = defaults.CLIFF_TIME();
        vm.expectRevert(
            abi.encodeWithSelector(
                Errors.SablierV2LockupLinear_CliffTimeNotLessThanEndTime.selector, cliffTime, endTime
            )
        );
        createDefaultStreamWithTimestamps(LockupLinear.Timestamps({ start: startTime, cliff: cliffTime, end: endTime }));
    }

    function test_RevertGiven_EndTimeNotInTheFuture()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenCliffTimeGreaterThanZero
        whenStartTimeLessThanEndTime
        whenCliffTimeLessThanEndTime
        whenEndTimeInTheFuture
    {
        uint40 endTime = defaults.END_TIME();
        vm.warp({ newTimestamp: defaults.END_TIME() });
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_EndTimeNotInTheFuture.selector, endTime, endTime));
        createDefaultStream();
    }

    function test_RevertWhen_BrokerFeeTooHigh()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenCliffTimeGreaterThanZero
        whenStartTimeLessThanEndTime
        whenCliffTimeLessThanEndTime
        whenEndTimeInTheFuture
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
        whenCliffTimeGreaterThanZero
        whenStartTimeLessThanEndTime
        whenCliffTimeLessThanEndTime
        whenEndTimeInTheFuture
        whenBrokerFeeNotTooHigh
    {
        address nonContract = address(8128);
        vm.expectRevert(abi.encodeWithSelector(Address.AddressEmptyCode.selector, nonContract));
        createDefaultStreamWithAsset(IERC20(nonContract));
    }

    function test_CreateWithTimestamps_AssetMissingReturnValue()
        external
        whenNotDelegateCalled
        whenRecipientNonZeroAddress
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenCliffTimeGreaterThanZero
        whenStartTimeLessThanEndTime
        whenCliffTimeLessThanEndTime
        whenEndTimeInTheFuture
        whenBrokerFeeNotTooHigh
        whenAssetContract
    {
        testCreateWithTimestamps(address(usdt));
    }

    function test_CreateWithTimestamps()
        external
        whenNotDelegateCalled
        whenDepositAmountNotZero
        whenStartTimeNotZero
        whenCliffTimeGreaterThanZero
        whenStartTimeLessThanEndTime
        whenCliffTimeLessThanEndTime
        whenEndTimeInTheFuture
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

        // Expect the assets to be transferred from the funder to {SablierV2LockupLinear}.
        expectCallToTransferFrom({
            asset: IERC20(asset),
            from: funder,
            to: address(lockupLinear),
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
        vm.expectEmit({ emitter: address(lockupLinear) });
        emit MetadataUpdate({ _tokenId: streamId });
        vm.expectEmit({ emitter: address(lockupLinear) });
        emit CreateLockupLinearStream({
            streamId: streamId,
            funder: funder,
            sender: users.sender,
            recipient: users.recipient,
            amounts: defaults.lockupCreateAmounts(),
            asset: IERC20(asset),
            cancelable: true,
            transferable: true,
            timestamps: defaults.lockupLinearTimestamps(),
            broker: users.broker
        });

        // Create the stream.
        createDefaultStreamWithAsset(IERC20(asset));

        // Assert that the stream has been created.
        LockupLinear.StreamLL memory actualStream = lockupLinear.getStream(streamId);
        LockupLinear.StreamLL memory expectedStream = defaults.lockupLinearStream();
        expectedStream.asset = IERC20(asset);
        assertEq(actualStream, expectedStream);

        // Assert that the stream's status is "PENDING".
        Lockup.Status actualStatus = lockupLinear.statusOf(streamId);
        Lockup.Status expectedStatus = Lockup.Status.PENDING;
        assertEq(actualStatus, expectedStatus);

        // Assert that the next stream ID has been bumped.
        uint256 actualNextStreamId = lockupLinear.nextStreamId();
        uint256 expectedNextStreamId = streamId + 1;
        assertEq(actualNextStreamId, expectedNextStreamId, "nextStreamId");

        // Assert that the NFT has been minted.
        address actualNFTOwner = lockupLinear.ownerOf({ tokenId: streamId });
        address expectedNFTOwner = users.recipient;
        assertEq(actualNFTOwner, expectedNFTOwner, "NFT owner");
    }
}
