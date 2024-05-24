// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ISablierV2Lockup } from "src/interfaces/ISablierV2Lockup.sol";

import { LockupDynamic_Integration_Shared_Test } from "../../shared/lockup-dynamic/LockupDynamic.t.sol";
import { Integration_Test } from "../../Integration.t.sol";
import { Cancel_Integration_Fuzz_Test } from "../lockup/cancel.t.sol";
import { CancelMultiple_Integration_Fuzz_Test } from "../lockup/cancelMultiple.t.sol";
import { GetWithdrawnAmount_Integration_Fuzz_Test } from "../lockup/getWithdrawnAmount.t.sol";
import { RefundableAmountOf_Integration_Fuzz_Test } from "../lockup/refundableAmountOf.t.sol";
import { WithdrawMax_Integration_Fuzz_Test } from "../lockup/withdrawMax.t.sol";
import { WithdrawMaxAndTransfer_Integration_Fuzz_Test } from "../lockup/withdrawMaxAndTransfer.t.sol";
import { WithdrawMultiple_Integration_Fuzz_Test } from "../lockup/withdrawMultiple.t.sol";

/*//////////////////////////////////////////////////////////////////////////
                            NON-SHARED ABSTRACT TEST
//////////////////////////////////////////////////////////////////////////*/

/// @notice Common testing logic needed across {SablierV2LockupDynamic} integration fuzz tests.
abstract contract LockupDynamic_Integration_Fuzz_Test is Integration_Test, LockupDynamic_Integration_Shared_Test {
    function setUp() public virtual override(Integration_Test, LockupDynamic_Integration_Shared_Test) {
        // Both of these contracts inherit from {Base_Test}, which is fine because multiple inheritance is
        // allowed in Solidity, and {Base_Test-setUp} will only be called once.
        Integration_Test.setUp();
        LockupDynamic_Integration_Shared_Test.setUp();

        // Cast the LockupDynamic contract as {ISablierV2Lockup}.
        lockup = ISablierV2Lockup(lockupDynamic);
    }
}

/*//////////////////////////////////////////////////////////////////////////
                                SHARED TESTS
//////////////////////////////////////////////////////////////////////////*/

contract Cancel_LockupDynamic_Integration_Fuzz_Test is
    LockupDynamic_Integration_Fuzz_Test,
    Cancel_Integration_Fuzz_Test
{
    function setUp() public virtual override(LockupDynamic_Integration_Fuzz_Test, Cancel_Integration_Fuzz_Test) {
        LockupDynamic_Integration_Fuzz_Test.setUp();
        Cancel_Integration_Fuzz_Test.setUp();
    }
}

contract CancelMultiple_LockupDynamic_Integration_Fuzz_Test is
    LockupDynamic_Integration_Fuzz_Test,
    CancelMultiple_Integration_Fuzz_Test
{
    function setUp()
        public
        virtual
        override(LockupDynamic_Integration_Fuzz_Test, CancelMultiple_Integration_Fuzz_Test)
    {
        LockupDynamic_Integration_Fuzz_Test.setUp();
        CancelMultiple_Integration_Fuzz_Test.setUp();
    }
}

contract RefundableAmountOf_LockupDynamic_Integration_Fuzz_Test is
    LockupDynamic_Integration_Fuzz_Test,
    RefundableAmountOf_Integration_Fuzz_Test
{
    function setUp()
        public
        virtual
        override(LockupDynamic_Integration_Fuzz_Test, RefundableAmountOf_Integration_Fuzz_Test)
    {
        LockupDynamic_Integration_Fuzz_Test.setUp();
        RefundableAmountOf_Integration_Fuzz_Test.setUp();
    }
}

contract GetWithdrawnAmount_LockupDynamic_Integration_Fuzz_Test is
    LockupDynamic_Integration_Fuzz_Test,
    GetWithdrawnAmount_Integration_Fuzz_Test
{
    function setUp()
        public
        virtual
        override(LockupDynamic_Integration_Fuzz_Test, GetWithdrawnAmount_Integration_Fuzz_Test)
    {
        LockupDynamic_Integration_Fuzz_Test.setUp();
        GetWithdrawnAmount_Integration_Fuzz_Test.setUp();
    }
}

contract WithdrawMax_LockupDynamic_Integration_Fuzz_Test is
    LockupDynamic_Integration_Fuzz_Test,
    WithdrawMax_Integration_Fuzz_Test
{
    function setUp() public virtual override(LockupDynamic_Integration_Fuzz_Test, WithdrawMax_Integration_Fuzz_Test) {
        LockupDynamic_Integration_Fuzz_Test.setUp();
        WithdrawMax_Integration_Fuzz_Test.setUp();
    }
}

contract WithdrawMaxAndTransfer_LockupDynamic_Integration_Fuzz_Test is
    LockupDynamic_Integration_Fuzz_Test,
    WithdrawMaxAndTransfer_Integration_Fuzz_Test
{
    function setUp()
        public
        virtual
        override(LockupDynamic_Integration_Fuzz_Test, WithdrawMaxAndTransfer_Integration_Fuzz_Test)
    {
        LockupDynamic_Integration_Fuzz_Test.setUp();
        WithdrawMaxAndTransfer_Integration_Fuzz_Test.setUp();
    }
}

contract WithdrawMultiple_LockupDynamic_Integration_Fuzz_Test is
    LockupDynamic_Integration_Fuzz_Test,
    WithdrawMultiple_Integration_Fuzz_Test
{
    function setUp()
        public
        virtual
        override(LockupDynamic_Integration_Fuzz_Test, WithdrawMultiple_Integration_Fuzz_Test)
    {
        LockupDynamic_Integration_Fuzz_Test.setUp();
        WithdrawMultiple_Integration_Fuzz_Test.setUp();
    }
}
