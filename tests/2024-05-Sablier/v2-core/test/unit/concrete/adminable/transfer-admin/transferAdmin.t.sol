// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { Adminable_Unit_Shared_Test } from "../../../shared/Adminable.t.sol";

contract TransferAdmin_Unit_Concrete_Test is Adminable_Unit_Shared_Test {
    function test_RevertWhen_CallerNotAdmin() external {
        // Make Eve the caller in this test.
        resetPrank(users.eve);

        // Run the test.
        vm.expectRevert(abi.encodeWithSelector(Errors.CallerNotAdmin.selector, users.admin, users.eve));
        adminableMock.transferAdmin(users.eve);
    }

    modifier whenCallerAdmin() {
        _;
    }

    function test_TransferAdmin_SameAdmin() external whenCallerAdmin {
        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(adminableMock) });
        emit TransferAdmin({ oldAdmin: users.admin, newAdmin: users.admin });

        // Transfer the admin.
        adminableMock.transferAdmin(users.admin);

        // Assert that the admin has remained the same.
        address actualAdmin = adminableMock.admin();
        address expectedAdmin = users.admin;
        assertEq(actualAdmin, expectedAdmin, "admin");
    }

    function test_TransferAdmin_ZeroAddress() external whenCallerAdmin {
        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(adminableMock) });
        emit TransferAdmin({ oldAdmin: users.admin, newAdmin: address(0) });

        // Transfer the admin.
        adminableMock.transferAdmin(address(0));

        // Assert that the admin has been transferred.
        address actualAdmin = adminableMock.admin();
        address expectedAdmin = address(0);
        assertEq(actualAdmin, expectedAdmin, "admin");
    }

    modifier whenNotZeroAddress() {
        _;
    }

    function test_TransferAdmin_NewAdmin() external whenCallerAdmin whenNotZeroAddress {
        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(adminableMock) });
        emit TransferAdmin({ oldAdmin: users.admin, newAdmin: users.alice });

        // Transfer the admin.
        adminableMock.transferAdmin(users.alice);

        // Assert that the admin has been transferred.
        address actualAdmin = adminableMock.admin();
        address expectedAdmin = users.alice;
        assertEq(actualAdmin, expectedAdmin, "admin");
    }
}
