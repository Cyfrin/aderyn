// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2Lockup } from "src/interfaces/ISablierV2Lockup.sol";

import { LockupTranched_Integration_Shared_Test } from "../../shared/lockup-tranched/LockupTranched.t.sol";
import { Integration_Test } from "../../Integration.t.sol";
import { Burn_Integration_Concrete_Test } from "../lockup/burn/burn.t.sol";
import { Cancel_Integration_Concrete_Test } from "../lockup/cancel/cancel.t.sol";
import { CancelMultiple_Integration_Concrete_Test } from "../lockup/cancel-multiple/cancelMultiple.t.sol";
import { GetAsset_Integration_Concrete_Test } from "../lockup/get-asset/getAsset.t.sol";
import { GetDepositedAmount_Integration_Concrete_Test } from "../lockup/get-deposited-amount/getDepositedAmount.t.sol";
import { GetEndTime_Integration_Concrete_Test } from "../lockup/get-end-time/getEndTime.t.sol";
import { GetRecipient_Integration_Concrete_Test } from "../lockup/get-recipient/getRecipient.t.sol";
import { GetRefundedAmount_Integration_Concrete_Test } from "../lockup/get-refunded-amount/getRefundedAmount.t.sol";
import { GetSender_Integration_Concrete_Test } from "../lockup/get-sender/getSender.t.sol";
import { GetStartTime_Integration_Concrete_Test } from "../lockup/get-start-time/getStartTime.t.sol";
import { GetWithdrawnAmount_Integration_Concrete_Test } from "../lockup/get-withdrawn-amount/getWithdrawnAmount.t.sol";
import { IsCancelable_Integration_Concrete_Test } from "../lockup/is-cancelable/isCancelable.t.sol";
import { IsCold_Integration_Concrete_Test } from "../lockup/is-cold/isCold.t.sol";
import { IsDepleted_Integration_Concrete_Test } from "../lockup/is-depleted/isDepleted.t.sol";
import { IsStream_Integration_Concrete_Test } from "../lockup/is-stream/isStream.t.sol";
import { IsTransferable_Integration_Concrete_Test } from "../lockup/is-transferable/isTransferable.t.sol";
import { IsWarm_Integration_Concrete_Test } from "../lockup/is-warm/isWarm.t.sol";
import { RefundableAmountOf_Integration_Concrete_Test } from "../lockup/refundable-amount-of/refundableAmountOf.t.sol";
import { Renounce_Integration_Concrete_Test } from "../lockup/renounce/renounce.t.sol";
import { SetNFTDescriptor_Integration_Concrete_Test } from "../lockup/set-nft-descriptor/setNFTDescriptor.t.sol";
import { StatusOf_Integration_Concrete_Test } from "../lockup/status-of/statusOf.t.sol";
import { TransferFrom_Integration_Concrete_Test } from "../lockup/transfer-from/transferFrom.t.sol";
import { WasCanceled_Integration_Concrete_Test } from "../lockup/was-canceled/wasCanceled.t.sol";
import { Withdraw_Integration_Concrete_Test } from "../lockup/withdraw/withdraw.t.sol";
import { WithdrawHooks_Integration_Concrete_Test } from "../lockup/withdraw-hooks/withdrawHooks.t.sol";
import { WithdrawMax_Integration_Concrete_Test } from "../lockup/withdraw-max/withdrawMax.t.sol";
import { WithdrawMaxAndTransfer_Integration_Concrete_Test } from
    "../lockup/withdraw-max-and-transfer/withdrawMaxAndTransfer.t.sol";
import { WithdrawMultiple_Integration_Concrete_Test } from "../lockup/withdraw-multiple/withdrawMultiple.t.sol";

/*//////////////////////////////////////////////////////////////////////////
                            NON-SHARED ABSTRACT TEST
//////////////////////////////////////////////////////////////////////////*/

/// @notice Common testing logic needed across {SablierV2LockupTranched} integration concrete tests.
abstract contract LockupTranched_Integration_Concrete_Test is
    Integration_Test,
    LockupTranched_Integration_Shared_Test
{
    function setUp() public virtual override(Integration_Test, LockupTranched_Integration_Shared_Test) {
        // Both of these contracts inherit from {Base_Test}, which is fine because multiple inheritance is
        // allowed in Solidity, and {Base_Test-setUp} will only be called once.
        Integration_Test.setUp();
        LockupTranched_Integration_Shared_Test.setUp();

        // Cast the LockupTranched contract as {ISablierV2Lockup}.
        lockup = ISablierV2Lockup(lockupTranched);
    }
}

/*//////////////////////////////////////////////////////////////////////////
                                SHARED TESTS
//////////////////////////////////////////////////////////////////////////*/

contract Burn_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    Burn_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, Burn_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        Burn_Integration_Concrete_Test.setUp();
    }
}

contract Cancel_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    Cancel_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, Cancel_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        Cancel_Integration_Concrete_Test.setUp();
    }
}

contract CancelMultiple_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    CancelMultiple_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, CancelMultiple_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        CancelMultiple_Integration_Concrete_Test.setUp();
    }
}

contract GetAsset_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    GetAsset_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, GetAsset_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        GetAsset_Integration_Concrete_Test.setUp();
    }
}

contract GetDepositedAmount_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    GetDepositedAmount_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, GetDepositedAmount_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        GetDepositedAmount_Integration_Concrete_Test.setUp();
    }
}

contract GetEndTime_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    GetEndTime_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, GetEndTime_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        GetEndTime_Integration_Concrete_Test.setUp();
    }
}

contract GetRecipient_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    GetRecipient_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, GetRecipient_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        GetRecipient_Integration_Concrete_Test.setUp();
    }
}

contract GetRefundedAmount_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    GetRefundedAmount_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, GetRefundedAmount_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        GetRefundedAmount_Integration_Concrete_Test.setUp();
    }
}

contract GetSender_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    GetSender_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, GetSender_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        GetSender_Integration_Concrete_Test.setUp();
    }
}

contract GetStartTime_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    GetStartTime_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, GetStartTime_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        GetStartTime_Integration_Concrete_Test.setUp();
    }
}

contract GetWithdrawnAmount_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    GetWithdrawnAmount_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, GetWithdrawnAmount_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        GetWithdrawnAmount_Integration_Concrete_Test.setUp();
    }
}

contract IsCancelable_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    IsCancelable_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, IsCancelable_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        IsCancelable_Integration_Concrete_Test.setUp();
    }
}

contract IsCold_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    IsCold_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, IsCold_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        IsCold_Integration_Concrete_Test.setUp();
    }
}

contract IsDepleted_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    IsDepleted_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, IsDepleted_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        IsDepleted_Integration_Concrete_Test.setUp();
    }
}

contract IsStream_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    IsStream_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, IsStream_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        IsStream_Integration_Concrete_Test.setUp();
    }
}

contract IsTransferable_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    IsTransferable_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, IsTransferable_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        IsTransferable_Integration_Concrete_Test.setUp();
    }
}

contract IsWarm_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    IsWarm_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, IsWarm_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        IsWarm_Integration_Concrete_Test.setUp();
    }
}

contract RefundableAmountOf_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    RefundableAmountOf_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, RefundableAmountOf_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        RefundableAmountOf_Integration_Concrete_Test.setUp();
    }
}

contract Renounce_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    Renounce_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, Renounce_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        Renounce_Integration_Concrete_Test.setUp();
    }
}

contract SetNFTDescriptor_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    SetNFTDescriptor_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, SetNFTDescriptor_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        SetNFTDescriptor_Integration_Concrete_Test.setUp();
    }
}

contract StatusOf_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    StatusOf_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, StatusOf_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        StatusOf_Integration_Concrete_Test.setUp();
    }
}

contract TransferFrom_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    TransferFrom_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, TransferFrom_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        TransferFrom_Integration_Concrete_Test.setUp();
    }
}

contract WasCanceled_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    WasCanceled_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, WasCanceled_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        WasCanceled_Integration_Concrete_Test.setUp();
    }
}

contract Withdraw_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    Withdraw_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, Withdraw_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        Withdraw_Integration_Concrete_Test.setUp();
    }
}

contract WithdrawHooks_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    WithdrawHooks_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, WithdrawHooks_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        WithdrawHooks_Integration_Concrete_Test.setUp();
    }
}

contract WithdrawMax_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    WithdrawMax_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, WithdrawMax_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        WithdrawMax_Integration_Concrete_Test.setUp();
    }
}

contract WithdrawMaxAndTransfer_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    WithdrawMaxAndTransfer_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, WithdrawMaxAndTransfer_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        WithdrawMaxAndTransfer_Integration_Concrete_Test.setUp();
    }
}

contract WithdrawMultiple_LockupTranched_Integration_Concrete_Test is
    LockupTranched_Integration_Concrete_Test,
    WithdrawMultiple_Integration_Concrete_Test
{
    function setUp()
        public
        virtual
        override(LockupTranched_Integration_Concrete_Test, WithdrawMultiple_Integration_Concrete_Test)
    {
        LockupTranched_Integration_Concrete_Test.setUp();
        WithdrawMultiple_Integration_Concrete_Test.setUp();
    }
}
