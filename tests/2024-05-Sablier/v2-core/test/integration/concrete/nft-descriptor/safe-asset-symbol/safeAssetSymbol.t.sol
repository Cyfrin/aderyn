// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { ERC20Mock } from "../../../../mocks/erc20/ERC20Mock.sol";
import { ERC20Bytes32 } from "../../../../mocks/erc20/ERC20Bytes32.sol";
import { NFTDescriptor_Integration_Concrete_Test } from "../NFTDescriptor.t.sol";

contract SafeAssetSymbol_Integration_Concrete_Test is NFTDescriptor_Integration_Concrete_Test {
    function test_SafeAssetSymbol_EOA() external view {
        address eoa = vm.addr({ privateKey: 1 });
        string memory actualSymbol = nftDescriptorMock.safeAssetSymbol_(address(eoa));
        string memory expectedSymbol = "ERC20";
        assertEq(actualSymbol, expectedSymbol, "symbol");
    }

    function test_SafeAssetSymbol_SymbolNotImplemented() external view {
        string memory actualSymbol = nftDescriptorMock.safeAssetSymbol_(address(noop));
        string memory expectedSymbol = "ERC20";
        assertEq(actualSymbol, expectedSymbol, "symbol");
    }

    modifier whenERC20Contract() {
        _;
    }

    function test_SafeAssetSymbol_Bytes32() external whenERC20Contract {
        ERC20Bytes32 asset = new ERC20Bytes32();
        string memory actualSymbol = nftDescriptorMock.safeAssetSymbol_(address(asset));
        string memory expectedSymbol = "ERC20";
        assertEq(actualSymbol, expectedSymbol, "symbol");
    }

    modifier givenSymbolString() {
        _;
    }

    function test_SafeAssetSymbol_LongSymbol() external whenERC20Contract givenSymbolString {
        ERC20Mock asset = new ERC20Mock({
            name: "Token",
            symbol: "This symbol is has more than 30 characters and it should be ignored"
        });
        string memory actualSymbol = nftDescriptorMock.safeAssetSymbol_(address(asset));
        string memory expectedSymbol = "Long Symbol";
        assertEq(actualSymbol, expectedSymbol, "symbol");
    }

    modifier givenSymbolNotLong() {
        _;
    }

    function test_SafeAssetSymbol() external view whenERC20Contract givenSymbolString givenSymbolNotLong {
        string memory actualSymbol = nftDescriptorMock.safeAssetSymbol_(address(dai));
        string memory expectedSymbol = dai.symbol();
        assertEq(actualSymbol, expectedSymbol, "symbol");
    }
}
