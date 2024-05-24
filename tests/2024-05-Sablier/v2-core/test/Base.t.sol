// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { ISablierV2LockupDynamic } from "../src/interfaces/ISablierV2LockupDynamic.sol";
import { ISablierV2LockupLinear } from "../src/interfaces/ISablierV2LockupLinear.sol";
import { ISablierV2LockupTranched } from "../src/interfaces/ISablierV2LockupTranched.sol";
import { ISablierV2NFTDescriptor } from "../src/interfaces/ISablierV2NFTDescriptor.sol";
import { SablierV2LockupDynamic } from "../src/SablierV2LockupDynamic.sol";
import { SablierV2LockupLinear } from "../src/SablierV2LockupLinear.sol";
import { SablierV2LockupTranched } from "../src/SablierV2LockupTranched.sol";
import { SablierV2NFTDescriptor } from "../src/SablierV2NFTDescriptor.sol";

import { ERC20Mock } from "./mocks/erc20/ERC20Mock.sol";
import { ERC20MissingReturn } from "./mocks/erc20/ERC20MissingReturn.sol";
import { Noop } from "./mocks/Noop.sol";
import { GoodRecipient } from "./mocks/hooks/GoodRecipient.sol";
import { GoodSender } from "./mocks/hooks/GoodSender.sol";
import { Assertions } from "./utils/Assertions.sol";
import { Calculations } from "./utils/Calculations.sol";
import { Constants } from "./utils/Constants.sol";
import { Defaults } from "./utils/Defaults.sol";
import { DeployOptimized } from "./utils/DeployOptimized.sol";
import { Events } from "./utils/Events.sol";
import { Fuzzers } from "./utils/Fuzzers.sol";
import { Users } from "./utils/Types.sol";

/// @notice Base test contract with common logic needed by all tests.
abstract contract Base_Test is Assertions, Calculations, Constants, DeployOptimized, Events, Fuzzers {
    /*//////////////////////////////////////////////////////////////////////////
                                     VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    Users internal users;

    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    ERC20Mock internal dai;
    Defaults internal defaults;
    GoodRecipient internal goodRecipient;
    GoodSender internal goodSender;
    ISablierV2LockupDynamic internal lockupDynamic;
    ISablierV2LockupLinear internal lockupLinear;
    ISablierV2LockupTranched internal lockupTranched;
    ISablierV2NFTDescriptor internal nftDescriptor;
    Noop internal noop;
    ERC20MissingReturn internal usdt;

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual {
        // Deploy the base test contracts.
        dai = new ERC20Mock("Dai Stablecoin", "DAI");
        goodRecipient = new GoodRecipient();
        goodSender = new GoodSender();
        noop = new Noop();
        usdt = new ERC20MissingReturn("Tether USD", "USDT", 6);

        // Label the base test contracts.
        vm.label({ account: address(dai), newLabel: "DAI" });
        vm.label({ account: address(goodRecipient), newLabel: "Good Recipient" });
        vm.label({ account: address(goodSender), newLabel: "Good Sender" });
        vm.label({ account: address(noop), newLabel: "Noop" });
        vm.label({ account: address(usdt), newLabel: "USDT" });

        // Deploy the defaults contract.
        defaults = new Defaults();
        defaults.setAsset(dai);

        // Create the protocol admin.
        users.admin = payable(makeAddr({ name: "Admin" }));
        vm.startPrank({ msgSender: users.admin });

        // Deploy the V2 Core contracts.
        deployCoreConditionally();

        // Create users for testing.
        users.alice = createUser("Alice");
        users.broker = createUser("Broker");
        users.eve = createUser("Eve");
        users.operator = createUser("Operator");
        users.recipient = createUser("Recipient");
        users.sender = createUser("Sender");

        defaults.setUsers(users);

        // Warp to May 1, 2024 at 00:00 GMT to provide a more realistic testing environment.
        vm.warp({ newTimestamp: MAY_1_2024 });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      HELPERS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Approves all V2 Core contracts to spend assets from the address passed.
    function approveProtocol(address from) internal {
        resetPrank({ msgSender: from });
        dai.approve({ spender: address(lockupLinear), value: MAX_UINT256 });
        dai.approve({ spender: address(lockupDynamic), value: MAX_UINT256 });
        dai.approve({ spender: address(lockupTranched), value: MAX_UINT256 });
        usdt.approve({ spender: address(lockupLinear), value: MAX_UINT256 });
        usdt.approve({ spender: address(lockupDynamic), value: MAX_UINT256 });
        usdt.approve({ spender: address(lockupTranched), value: MAX_UINT256 });
    }

    /// @dev Generates a user, labels its address, funds it with test assets, and approves the protocol contracts.
    function createUser(string memory name) internal returns (address payable) {
        address payable user = payable(makeAddr(name));
        vm.deal({ account: user, newBalance: 100 ether });
        deal({ token: address(dai), to: user, give: 1_000_000e18 });
        deal({ token: address(usdt), to: user, give: 1_000_000e18 });
        approveProtocol({ from: user });
        return user;
    }

    /// @dev Conditionally deploys V2 Core normally or from an optimized source compiled with `--via-ir`.
    /// We cannot use the {DeployCore} script because some tests rely on hard coded addresses for the
    /// deployed contracts. Since the script itself would have to be deployed, using it would bump the
    /// deployer's nonce, which would in turn lead to different addresses (recall that the addresses
    /// for contracts deployed via `CREATE` are based on the caller-and-nonce-hash).
    function deployCoreConditionally() internal {
        if (!isTestOptimizedProfile()) {
            nftDescriptor = new SablierV2NFTDescriptor();
            lockupDynamic = new SablierV2LockupDynamic(users.admin, nftDescriptor, defaults.MAX_SEGMENT_COUNT());
            lockupLinear = new SablierV2LockupLinear(users.admin, nftDescriptor);
            lockupTranched = new SablierV2LockupTranched(users.admin, nftDescriptor, defaults.MAX_TRANCHE_COUNT());
        } else {
            (lockupDynamic, lockupLinear, lockupTranched, nftDescriptor) =
                deployOptimizedCore(users.admin, defaults.MAX_SEGMENT_COUNT(), defaults.MAX_TRANCHE_COUNT());
        }

        vm.label({ account: address(lockupDynamic), newLabel: "LockupDynamic" });
        vm.label({ account: address(lockupLinear), newLabel: "LockupLinear" });
        vm.label({ account: address(lockupTranched), newLabel: "LockupTranched" });
        vm.label({ account: address(nftDescriptor), newLabel: "NFTDescriptor" });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                    CALL EXPECTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Expects a call to {IERC20.transfer}.
    function expectCallToTransfer(address to, uint256 value) internal {
        vm.expectCall({ callee: address(dai), data: abi.encodeCall(IERC20.transfer, (to, value)) });
    }

    /// @dev Expects a call to {IERC20.transfer}.
    function expectCallToTransfer(IERC20 asset, address to, uint256 value) internal {
        vm.expectCall({ callee: address(asset), data: abi.encodeCall(IERC20.transfer, (to, value)) });
    }

    /// @dev Expects a call to {IERC20.transferFrom}.
    function expectCallToTransferFrom(address from, address to, uint256 value) internal {
        vm.expectCall({ callee: address(dai), data: abi.encodeCall(IERC20.transferFrom, (from, to, value)) });
    }

    /// @dev Expects a call to {IERC20.transferFrom}.
    function expectCallToTransferFrom(IERC20 asset, address from, address to, uint256 value) internal {
        vm.expectCall({ callee: address(asset), data: abi.encodeCall(IERC20.transferFrom, (from, to, value)) });
    }
}
