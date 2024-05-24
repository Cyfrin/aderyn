// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup_Integration_Shared_Test } from "./Lockup.t.sol";

abstract contract Withdraw_Integration_Shared_Test is Lockup_Integration_Shared_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override {
        defaultStreamId = createDefaultStream();
        resetPrank({ msgSender: users.recipient });
    }

    modifier givenNotNull() {
        _;
    }

    modifier givenRecipientContract() {
        _;
    }

    modifier givenSenderContract() {
        _;
    }

    modifier givenStreamNotDepleted() {
        vm.warp({ newTimestamp: defaults.START_TIME() });
        _;
    }

    modifier whenCallerRecipient() {
        _;
    }

    modifier whenNoOverdraw() {
        _;
    }

    modifier whenNotDelegateCalled() {
        _;
    }

    modifier whenStreamHasNotBeenCanceled() {
        _;
    }

    modifier whenToNonZeroAddress() {
        _;
    }

    modifier whenWithdrawalAddressIsRecipient() {
        _;
    }

    modifier whenWithdrawalAddressNotRecipient() {
        _;
    }

    modifier whenWithdrawAmountNotZero() {
        _;
    }
}
