// SPDX-License-Identifier: UNLICENSED
// solhint-disable max-states-count
pragma solidity >=0.8.22 <0.9.0;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { ISablierV2LockupDynamic } from "@sablier/v2-core/src/interfaces/ISablierV2LockupDynamic.sol";
import { ISablierV2LockupLinear } from "@sablier/v2-core/src/interfaces/ISablierV2LockupLinear.sol";
import { ISablierV2LockupTranched } from "@sablier/v2-core/src/interfaces/ISablierV2LockupTranched.sol";
import { LockupDynamic, LockupLinear, LockupTranched } from "@sablier/v2-core/src/types/DataTypes.sol";

import { Assertions as V2CoreAssertions } from "@sablier/v2-core/test/utils/Assertions.sol";
import { Constants as V2CoreConstants } from "@sablier/v2-core/test/utils/Constants.sol";
import { Utils as V2CoreUtils } from "@sablier/v2-core/test/utils/Utils.sol";

import { ISablierV2BatchLockup } from "src/interfaces/ISablierV2BatchLockup.sol";
import { ISablierV2MerkleLL } from "src/interfaces/ISablierV2MerkleLL.sol";
import { ISablierV2MerkleLockupFactory } from "src/interfaces/ISablierV2MerkleLockupFactory.sol";
import { ISablierV2MerkleLT } from "src/interfaces/ISablierV2MerkleLT.sol";
import { SablierV2BatchLockup } from "src/SablierV2BatchLockup.sol";
import { SablierV2MerkleLockupFactory } from "src/SablierV2MerkleLockupFactory.sol";

import { ERC20Mock } from "./mocks/erc20/ERC20Mock.sol";
import { Assertions } from "./utils/Assertions.sol";
import { Defaults } from "./utils/Defaults.sol";
import { DeployOptimized } from "./utils/DeployOptimized.sol";
import { Events } from "./utils/Events.sol";
import { Merkle } from "./utils/Murky.sol";
import { Users } from "./utils/Types.sol";

/// @notice Base test contract with common logic needed by all tests.
abstract contract Base_Test is
    Assertions,
    DeployOptimized,
    Events,
    Merkle,
    V2CoreConstants,
    V2CoreAssertions,
    V2CoreUtils
{
    /*//////////////////////////////////////////////////////////////////////////
                                     VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    Users internal users;

    /*//////////////////////////////////////////////////////////////////////////
                                   TEST CONTRACTS
    //////////////////////////////////////////////////////////////////////////*/

    ISablierV2BatchLockup internal batchLockup;
    IERC20 internal dai;
    Defaults internal defaults;
    ISablierV2LockupDynamic internal lockupDynamic;
    ISablierV2LockupLinear internal lockupLinear;
    ISablierV2LockupTranched internal lockupTranched;
    ISablierV2MerkleLockupFactory internal merkleLockupFactory;
    uint256 internal merkleLockupFactoryNonce;
    ISablierV2MerkleLL internal merkleLL;
    ISablierV2MerkleLT internal merkleLT;

    /*//////////////////////////////////////////////////////////////////////////
                                  SET-UP FUNCTION
    //////////////////////////////////////////////////////////////////////////*/

    function setUp() public virtual {
        // Deploy the default test asset.
        dai = new ERC20Mock("DAI Stablecoin", "DAI");

        // Create users for testing.
        users.alice = createUser("Alice");
        users.admin = createUser("Admin");
        users.broker = createUser("Broker");
        users.eve = createUser("Eve");
        users.recipient0 = createUser("Recipient");
        users.recipient1 = createUser("Recipient1");
        users.recipient2 = createUser("Recipient2");
        users.recipient3 = createUser("Recipient3");
        users.recipient4 = createUser("Recipient4");
    }

    /*//////////////////////////////////////////////////////////////////////////
                                     HELPERS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Approve `spender` to spend assets from `from`.
    function approveContract(IERC20 asset_, address from, address spender) internal {
        resetPrank({ msgSender: from });
        (bool success,) = address(asset_).call(abi.encodeCall(IERC20.approve, (spender, MAX_UINT256)));
        success;
    }

    /// @dev Generates a user, labels its address, and funds it with ETH.
    function createUser(string memory name) internal returns (address payable) {
        address user = makeAddr(name);
        vm.deal({ account: user, newBalance: 100_000 ether });
        deal({ token: address(dai), to: user, give: 1_000_000e18 });
        return payable(user);
    }

    /// @dev Conditionally deploy V2 Periphery normally or from an optimized source compiled with `--via-ir`.
    function deployPeripheryConditionally() internal {
        if (!isTestOptimizedProfile()) {
            batchLockup = new SablierV2BatchLockup();
            merkleLockupFactory = new SablierV2MerkleLockupFactory();
        } else {
            (batchLockup, merkleLockupFactory) = deployOptimizedPeriphery();
        }
    }

    /// @dev Labels the most relevant contracts.
    function labelContracts(IERC20 asset_) internal {
        vm.label({ account: address(asset_), newLabel: IERC20Metadata(address(asset_)).symbol() });
        vm.label({ account: address(defaults), newLabel: "Defaults" });
        vm.label({ account: address(lockupDynamic), newLabel: "LockupDynamic" });
        vm.label({ account: address(lockupLinear), newLabel: "LockupLinear" });
        vm.label({ account: address(lockupTranched), newLabel: "LockupTranched" });
        vm.label({ account: address(merkleLL), newLabel: "MerkleLL" });
        vm.label({ account: address(merkleLockupFactory), newLabel: "MerkleLockupFactory" });
        vm.label({ account: address(merkleLT), newLabel: "MerkleLT" });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                    CALL EXPECTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Expects a call to {IERC20.transfer}.
    function expectCallToTransfer(address to, uint256 amount) internal {
        expectCallToTransfer(address(dai), to, amount);
    }

    /// @dev Expects a call to {IERC20.transfer}.
    function expectCallToTransfer(address asset_, address to, uint256 amount) internal {
        vm.expectCall({ callee: asset_, data: abi.encodeCall(IERC20.transfer, (to, amount)) });
    }

    /// @dev Expects a call to {IERC20.transferFrom}.
    function expectCallToTransferFrom(address from, address to, uint256 amount) internal {
        expectCallToTransferFrom(address(dai), from, to, amount);
    }

    /// @dev Expects a call to {IERC20.transferFrom}.
    function expectCallToTransferFrom(address asset_, address from, address to, uint256 amount) internal {
        vm.expectCall({ callee: asset_, data: abi.encodeCall(IERC20.transferFrom, (from, to, amount)) });
    }

    /// @dev Expects multiple calls to {ISablierV2LockupDynamic.createWithDurations}, each with the specified
    /// `params`.
    function expectMultipleCallsToCreateWithDurationsLD(
        uint64 count,
        LockupDynamic.CreateWithDurations memory params
    )
        internal
    {
        vm.expectCall({
            callee: address(lockupDynamic),
            count: count,
            data: abi.encodeCall(ISablierV2LockupDynamic.createWithDurations, (params))
        });
    }

    /// @dev Expects multiple calls to {ISablierV2LockupLinear.createWithDurations}, each with the specified
    /// `params`.
    function expectMultipleCallsToCreateWithDurationsLL(
        uint64 count,
        LockupLinear.CreateWithDurations memory params
    )
        internal
    {
        vm.expectCall({
            callee: address(lockupLinear),
            count: count,
            data: abi.encodeCall(ISablierV2LockupLinear.createWithDurations, (params))
        });
    }

    /// @dev Expects multiple calls to {ISablierV2LockupTranched.createWithDurations}, each with the specified
    /// `params`.
    function expectMultipleCallsToCreateWithDurationsLT(
        uint64 count,
        LockupTranched.CreateWithDurations memory params
    )
        internal
    {
        vm.expectCall({
            callee: address(lockupTranched),
            count: count,
            data: abi.encodeCall(ISablierV2LockupTranched.createWithDurations, (params))
        });
    }

    /// @dev Expects multiple calls to {ISablierV2LockupDynamic.createWithTimestamps}, each with the specified
    /// `params`.
    function expectMultipleCallsToCreateWithTimestampsLD(
        uint64 count,
        LockupDynamic.CreateWithTimestamps memory params
    )
        internal
    {
        vm.expectCall({
            callee: address(lockupDynamic),
            count: count,
            data: abi.encodeCall(ISablierV2LockupDynamic.createWithTimestamps, (params))
        });
    }

    /// @dev Expects multiple calls to {ISablierV2LockupLinear.createWithTimestamps}, each with the specified
    /// `params`.
    function expectMultipleCallsToCreateWithTimestampsLL(
        uint64 count,
        LockupLinear.CreateWithTimestamps memory params
    )
        internal
    {
        vm.expectCall({
            callee: address(lockupLinear),
            count: count,
            data: abi.encodeCall(ISablierV2LockupLinear.createWithTimestamps, (params))
        });
    }

    /// @dev Expects multiple calls to {ISablierV2LockupTranched.createWithTimestamps}, each with the specified
    /// `params`.
    function expectMultipleCallsToCreateWithTimestampsLT(
        uint64 count,
        LockupTranched.CreateWithTimestamps memory params
    )
        internal
    {
        vm.expectCall({
            callee: address(lockupTranched),
            count: count,
            data: abi.encodeCall(ISablierV2LockupTranched.createWithTimestamps, (params))
        });
    }

    /// @dev Expects multiple calls to {IERC20.transfer}.
    function expectMultipleCallsToTransfer(uint64 count, address to, uint256 amount) internal {
        vm.expectCall({ callee: address(dai), count: count, data: abi.encodeCall(IERC20.transfer, (to, amount)) });
    }

    /// @dev Expects multiple calls to {IERC20.transferFrom}.
    function expectMultipleCallsToTransferFrom(uint64 count, address from, address to, uint256 amount) internal {
        expectMultipleCallsToTransferFrom(address(dai), count, from, to, amount);
    }

    /// @dev Expects multiple calls to {IERC20.transferFrom}.
    function expectMultipleCallsToTransferFrom(
        address asset_,
        uint64 count,
        address from,
        address to,
        uint256 amount
    )
        internal
    {
        vm.expectCall({ callee: asset_, count: count, data: abi.encodeCall(IERC20.transferFrom, (from, to, amount)) });
    }
}
