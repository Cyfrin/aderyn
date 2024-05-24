// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

import { Base_Test } from "../Base.t.sol";

/// @notice Common logic needed by all fork tests.
abstract contract Fork_Test is Base_Test {
    /*//////////////////////////////////////////////////////////////////////////
                                  STATE VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    IERC20 internal immutable ASSET;
    address internal immutable HOLDER;
    uint256 internal initialHolderBalance;

    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    constructor(IERC20 asset, address holder) {
        ASSET = asset;
        HOLDER = holder;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual override {
        // Fork Ethereum Mainnet at a specific block number.
        vm.createSelectFork({ blockNumber: 19_000_000, urlOrAlias: "mainnet" });

        // The base is set up after the fork is selected so that the base test contracts are deployed on the fork.
        Base_Test.setUp();

        // Label the contracts.
        labelContracts();

        // Make the ASSET HOLDER the caller in this test suite.
        resetPrank({ msgSender: HOLDER });

        // Query the initial balance of the ASSET HOLDER.
        initialHolderBalance = ASSET.balanceOf(HOLDER);
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      HELPERS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Checks the user assumptions.
    function checkUsers(address sender, address recipient, address broker, address sablierContract) internal virtual {
        // The protocol does not allow the zero address to interact with it.
        vm.assume(sender != address(0) && recipient != address(0) && broker != address(0));

        // The goal is to not have overlapping users because the ASSET balance tests would fail otherwise.
        vm.assume(sender != recipient && sender != broker && recipient != broker);
        vm.assume(sender != HOLDER && recipient != HOLDER && broker != HOLDER);
        vm.assume(sender != sablierContract && recipient != sablierContract && broker != sablierContract);

        // Avoid users blacklisted by USDC or USDT.
        assumeNoBlacklisted(address(ASSET), sender);
        assumeNoBlacklisted(address(ASSET), recipient);
        assumeNoBlacklisted(address(ASSET), broker);
    }

    /// @dev Labels the most relevant contracts.
    function labelContracts() internal {
        vm.label({ account: address(ASSET), newLabel: IERC20Metadata(address(ASSET)).symbol() });
        vm.label({ account: HOLDER, newLabel: "HOLDER" });
    }
}
