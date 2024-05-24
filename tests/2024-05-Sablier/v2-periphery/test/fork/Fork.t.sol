// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { Precompiles as V2CorePrecompiles } from "@sablier/v2-core/precompiles/Precompiles.sol";
import { ISablierV2LockupDynamic } from "@sablier/v2-core/src/interfaces/ISablierV2LockupDynamic.sol";
import { ISablierV2LockupLinear } from "@sablier/v2-core/src/interfaces/ISablierV2LockupLinear.sol";
import { ISablierV2LockupTranched } from "@sablier/v2-core/src/interfaces/ISablierV2LockupTranched.sol";

import { Fuzzers as V2CoreFuzzers } from "@sablier/v2-core/test/utils/Fuzzers.sol";

import { Defaults } from "../utils/Defaults.sol";
import { Base_Test } from "../Base.t.sol";

/// @notice Common logic needed by all fork tests.
abstract contract Fork_Test is Base_Test, V2CoreFuzzers {
    /*//////////////////////////////////////////////////////////////////////////
                                  STATE VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    IERC20 internal immutable FORK_ASSET;

    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    constructor(IERC20 forkAsset) {
        FORK_ASSET = forkAsset;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        // Fork Ethereum Mainnet at a specific block number.
        vm.createSelectFork({ blockNumber: 18_821_300, urlOrAlias: "mainnet" });

        // Set up the base test contract.
        Base_Test.setUp();

        // Load the external dependencies.
        // loadDependencies();
        // TODO: Remove this line once the V2 Core contracts are deployed on Mainnet.
        deployDependencies();

        // Deploy the defaults contract and allow it to access cheatcodes.
        defaults = new Defaults({ users_: users, asset_: FORK_ASSET });
        vm.allowCheatcodes(address(defaults));

        // Deploy V2 Periphery.
        deployPeripheryConditionally();

        // Label the contracts.
        labelContracts(FORK_ASSET);

        // Approve the BatchLockup contract.
        approveContract({ asset_: FORK_ASSET, from: users.alice, spender: address(batchLockup) });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      HELPERS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Checks the user assumptions.
    function checkUsers(address user, address recipient) internal virtual {
        // The protocol does not allow the zero address to interact with it.
        vm.assume(user != address(0) && recipient != address(0));

        // The goal is to not have overlapping users because the asset balance tests would fail otherwise.
        vm.assume(user != recipient);
        vm.assume(user != address(lockupDynamic) && recipient != address(lockupDynamic));
        vm.assume(user != address(lockupLinear) && recipient != address(lockupLinear));
        vm.assume(user != address(lockupTranched) && recipient != address(lockupTranched));

        // Avoid users blacklisted by USDC or USDT.
        assumeNoBlacklisted(address(FORK_ASSET), user);
        assumeNoBlacklisted(address(FORK_ASSET), recipient);
    }

    /// @dev Loads all dependencies pre-deployed on Mainnet.
    function loadDependencies() private {
        lockupDynamic = ISablierV2LockupDynamic(0x7CC7e125d83A581ff438608490Cc0f7bDff79127);
        lockupLinear = ISablierV2LockupLinear(0xAFb979d9afAd1aD27C5eFf4E27226E3AB9e5dCC9);
        lockupTranched = ISablierV2LockupTranched(0xAFb979d9afAd1aD27C5eFf4E27226E3AB9e5dCC9);
    }

    /// @dev Deploys the V2 Core dependencies.
    // TODO: Remove this function once the V2 Core contracts are deployed on Mainnet.
    function deployDependencies() private {
        (lockupDynamic, lockupLinear, lockupTranched,) = new V2CorePrecompiles().deployCore(users.admin);
    }
}
