// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Base_Test } from "../../Base.t.sol";
import { AdminableMock } from "../../mocks/AdminableMock.sol";

abstract contract Adminable_Unit_Shared_Test is Base_Test {
    AdminableMock internal adminableMock;

    function setUp() public virtual override {
        Base_Test.setUp();
        deployConditionally();
        resetPrank({ msgSender: users.admin });
    }

    /// @dev Conditionally deploys {AdminableMock} normally or from a source precompiled with `--via-ir`.
    function deployConditionally() internal {
        if (!isTestOptimizedProfile()) {
            adminableMock = new AdminableMock(users.admin);
        } else {
            adminableMock =
                AdminableMock(deployCode("out-optimized/AdminableMock.sol/AdminableMock.json", abi.encode(users.admin)));
        }
        vm.label({ account: address(adminableMock), newLabel: "AdminableMock" });
    }
}
