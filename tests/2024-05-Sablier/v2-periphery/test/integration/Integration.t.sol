// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Precompiles as V2CorePrecompiles } from "@sablier/v2-core/precompiles/Precompiles.sol";

import { Defaults } from "../utils/Defaults.sol";
import { Base_Test } from "../Base.t.sol";

/// @notice Common logic needed by all integration tests.
abstract contract Integration_Test is Base_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        // Set up the base test contract.
        Base_Test.setUp();

        // Deploy the external dependencies.
        deployDependencies();

        // Deploy the defaults contract.
        defaults = new Defaults({ users_: users, asset_: dai });

        // Deploy V2 Periphery.
        deployPeripheryConditionally();

        // Label the contracts.
        labelContracts(dai);

        // Approve the BatchLockup contract.
        approveContract({ asset_: dai, from: users.alice, spender: address(batchLockup) });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      HELPERS
    //////////////////////////////////////////////////////////////////////////*/

    function deployDependencies() private {
        (lockupDynamic, lockupLinear, lockupTranched,) = new V2CorePrecompiles().deployCore(users.admin);
    }
}
