// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { Lockup_Integration_Shared_Test } from "../../../shared/lockup/Lockup.t.sol";
import { Integration_Test } from "../../../Integration.t.sol";

abstract contract IsTransferable_Integration_Concrete_Test is Integration_Test, Lockup_Integration_Shared_Test {
    uint256 internal defaultStreamId;

    function setUp() public virtual override(Integration_Test, Lockup_Integration_Shared_Test) { }

    function test_RevertGiven_Null() external {
        uint256 nullStreamId = 1729;
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2Lockup_Null.selector, nullStreamId));
        lockup.isTransferable(nullStreamId);
    }

    modifier givenNotNull() {
        defaultStreamId = createDefaultStream();
        _;
    }

    function test_RevertGiven_StreamTransferNotEnabled() external givenNotNull {
        uint256 notTransferableStreamId = createDefaultStreamNotTransferable();
        bool isTransferable = lockup.isTransferable(notTransferableStreamId);
        assertFalse(isTransferable, "isTransferable");
    }

    modifier givenStreamTransferable() {
        _;
    }

    function test_IsTransferable_Stream() external givenNotNull givenStreamTransferable {
        bool isTransferable = lockup.isTransferable(defaultStreamId);
        assertTrue(isTransferable, "isTransferable");
    }
}
