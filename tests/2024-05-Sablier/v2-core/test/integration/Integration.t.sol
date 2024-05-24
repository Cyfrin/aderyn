// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { Base_Test } from "../Base.t.sol";
import { ReentrantRecipient } from "../mocks/hooks/ReentrantRecipient.sol";
import { ReentrantSender } from "../mocks/hooks/ReentrantSender.sol";
import { RevertingRecipient } from "../mocks/hooks/RevertingRecipient.sol";
import { RevertingSender } from "../mocks/hooks/RevertingSender.sol";

/// @notice Common logic needed by all integration tests, both concrete and fuzz tests.
abstract contract Integration_Test is Base_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    ReentrantRecipient internal reentrantRecipient = new ReentrantRecipient();
    ReentrantSender internal reentrantSender = new ReentrantSender();
    RevertingRecipient internal revertingRecipient = new RevertingRecipient();
    RevertingSender internal revertingSender = new RevertingSender();

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        Base_Test.setUp();

        // Label the contracts.
        labelContracts();

        // Make the Admin the default caller in this test suite.
        resetPrank({ msgSender: users.admin });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      HELPERS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Labels the most relevant contracts.
    function labelContracts() internal {
        vm.label({ account: address(reentrantRecipient), newLabel: "Reentrant Lockup Recipient" });
        vm.label({ account: address(reentrantSender), newLabel: "Reentrant Lockup Sender" });
        vm.label({ account: address(revertingRecipient), newLabel: "Reverting Lockup Recipient" });
        vm.label({ account: address(revertingSender), newLabel: "Reverting Lockup Sender" });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                    EXPECT CALLS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Expects a delegate call error.
    function expectRevertDueToDelegateCall(bool success, bytes memory returnData) internal pure {
        assertFalse(success, "delegatecall success");
        assertEq(returnData, abi.encodeWithSelector(Errors.DelegateCall.selector), "delegatecall return data");
    }
}
