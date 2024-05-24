// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Lockup_Integration_Shared_Test } from "./Lockup.t.sol";

abstract contract WithdrawMaxAndTransfer_Integration_Shared_Test is Lockup_Integration_Shared_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override {
        defaultStreamId = createDefaultStream();
        resetPrank({ msgSender: users.recipient });
    }

    modifier givenNFTNotBurned() {
        _;
    }

    modifier givenNotNull() {
        _;
    }

    modifier givenStreamTransferable() {
        _;
    }

    modifier givenWithdrawableAmountNotZero() {
        _;
    }

    modifier whenCallerCurrentRecipient() {
        _;
    }

    modifier whenNotDelegateCalled() {
        _;
    }
}
