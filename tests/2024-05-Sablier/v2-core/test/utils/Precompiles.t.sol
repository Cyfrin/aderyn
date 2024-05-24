// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { LibString } from "solady/src/utils/LibString.sol";

import { Precompiles } from "../../precompiles/Precompiles.sol";
import { ISablierV2LockupDynamic } from "../../src/interfaces/ISablierV2LockupDynamic.sol";
import { ISablierV2LockupLinear } from "../../src/interfaces/ISablierV2LockupLinear.sol";
import { ISablierV2LockupTranched } from "../../src/interfaces/ISablierV2LockupTranched.sol";
import { ISablierV2NFTDescriptor } from "../../src/interfaces/ISablierV2NFTDescriptor.sol";

import { Base_Test } from "../Base.t.sol";

contract Precompiles_Test is Base_Test {
    using LibString for address;

    Precompiles internal precompiles = new Precompiles();

    modifier onlyTestOptimizedProfile() {
        if (isTestOptimizedProfile()) {
            _;
        }
    }

    function test_DeployLockupDynamic() external onlyTestOptimizedProfile {
        address actualLockupDynamic = address(precompiles.deployLockupDynamic(users.admin, nftDescriptor));
        address expectedLockupDynamic =
            address(deployOptimizedLockupDynamic(users.admin, nftDescriptor, defaults.MAX_SEGMENT_COUNT()));
        bytes memory expectedLockupDynamicCode =
            adjustBytecode(expectedLockupDynamic.code, expectedLockupDynamic, actualLockupDynamic);
        assertEq(actualLockupDynamic.code, expectedLockupDynamicCode, "bytecodes mismatch");
    }

    function test_DeployLockupLinear() external onlyTestOptimizedProfile {
        address actualLockupLinear = address(precompiles.deployLockupLinear(users.admin, nftDescriptor));
        address expectedLockupLinear = address(deployOptimizedLockupLinear(users.admin, nftDescriptor));
        bytes memory expectedLockupLinearCode =
            adjustBytecode(expectedLockupLinear.code, expectedLockupLinear, actualLockupLinear);
        assertEq(actualLockupLinear.code, expectedLockupLinearCode, "bytecodes mismatch");
    }

    function test_DeployLockupTranched() external onlyTestOptimizedProfile {
        address actualLockupTranched = address(precompiles.deployLockupTranched(users.admin, nftDescriptor));
        address expectedLockupTranched =
            address(deployOptimizedLockupTranched(users.admin, nftDescriptor, defaults.MAX_TRANCHE_COUNT()));
        bytes memory expectedLockupTranchedCode =
            adjustBytecode(expectedLockupTranched.code, expectedLockupTranched, actualLockupTranched);
        assertEq(actualLockupTranched.code, expectedLockupTranchedCode, "bytecodes mismatch");
    }

    function test_DeployNFTDescriptor() external onlyTestOptimizedProfile {
        address actualNFTDescriptor = address(precompiles.deployNFTDescriptor());
        address expectedNFTDescriptor = address(deployOptimizedNFTDescriptor());
        assertEq(actualNFTDescriptor.code, expectedNFTDescriptor.code, "bytecodes mismatch");
    }

    function test_DeployCore() external onlyTestOptimizedProfile {
        (
            ISablierV2LockupDynamic actualLockupDynamic,
            ISablierV2LockupLinear actualLockupLinear,
            ISablierV2LockupTranched actualLockupTranched,
            ISablierV2NFTDescriptor actualNFTDescriptor
        ) = precompiles.deployCore(users.admin);

        (
            ISablierV2LockupDynamic expectedLockupDynamic,
            ISablierV2LockupLinear expectedLockupLinear,
            ISablierV2LockupTranched expectedLockupTranched,
            ISablierV2NFTDescriptor expectedNFTDescriptor
        ) = deployOptimizedCore(users.admin, defaults.MAX_SEGMENT_COUNT(), defaults.MAX_TRANCHE_COUNT());

        bytes memory expectedLockupDynamicCode = adjustBytecode(
            address(expectedLockupDynamic).code, address(expectedLockupDynamic), address(actualLockupDynamic)
        );

        bytes memory expectedLockupLinearCode = adjustBytecode(
            address(expectedLockupLinear).code, address(expectedLockupLinear), address(actualLockupLinear)
        );

        bytes memory expectedLockupTranchedCode = adjustBytecode(
            address(expectedLockupTranched).code, address(expectedLockupTranched), address(actualLockupTranched)
        );

        assertEq(address(actualLockupDynamic).code, expectedLockupDynamicCode, "bytecodes mismatch");
        assertEq(address(actualLockupLinear).code, expectedLockupLinearCode, "bytecodes mismatch");
        assertEq(address(actualLockupTranched).code, expectedLockupTranchedCode, "bytecodes mismatch");
        assertEq(address(actualNFTDescriptor).code, address(expectedNFTDescriptor).code, "bytecodes mismatch");
    }

    /// @dev The expected bytecode has to be adjusted because {SablierV2Lockup} inherits from {NoDelegateCall}, which
    /// saves the contract's own address in storage.
    function adjustBytecode(
        bytes memory bytecode,
        address expectedAddress,
        address actualAddress
    )
        internal
        pure
        returns (bytes memory)
    {
        return vm.parseBytes(
            vm.replace({
                input: vm.toString(bytecode),
                from: expectedAddress.toHexStringNoPrefix(),
                to: actualAddress.toHexStringNoPrefix()
            })
        );
    }
}
