// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Errors } from "src/libraries/Errors.sol";

import { Adminable_Unit_Shared_Test } from "../shared/Adminable.t.sol";

contract TransferAdmin_Unit_Fuzz_Test is Adminable_Unit_Shared_Test {
    function testFuzz_RevertWhen_CallerNotAdmin(address eve) external {
        vm.assume(eve != address(0) && eve != users.admin);
        assumeNotPrecompile(eve);

        // Make Eve the caller in this test.
        resetPrank(eve);

        // Run the test.
        vm.expectRevert(abi.encodeWithSelector(Errors.CallerNotAdmin.selector, users.admin, eve));
        adminableMock.transferAdmin(eve);
    }

    modifier whenCallerAdmin() {
        _;
    }

    function testFuzz_TransferAdmin(address newAdmin) external whenCallerAdmin {
        vm.assume(newAdmin != address(0));

        // Expect the relevant event to be emitted.
        vm.expectEmit({ emitter: address(adminableMock) });
        emit TransferAdmin({ oldAdmin: users.admin, newAdmin: newAdmin });

        // Transfer the admin.
        adminableMock.transferAdmin(newAdmin);

        // Assert that the admin has been transferred.
        address actualAdmin = adminableMock.admin();
        address expectedAdmin = newAdmin;
        assertEq(actualAdmin, expectedAdmin, "admin");
    }
}
