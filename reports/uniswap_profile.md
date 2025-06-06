# Aderyn Analysis Report

This report was generated by [Aderyn](https://github.com/Cyfrin/aderyn), a static analysis tool built by [Cyfrin](https://cyfrin.io), a blockchain security company. This report is not a substitute for manual audit or security review. It should not be relied upon for any purpose other than to assist in the identification of potential security vulnerabilities.
# Table of Contents

- [Summary](#summary)
  - [Files Summary](#files-summary)
  - [Files Details](#files-details)
  - [Issue Summary](#issue-summary)
- [Low Issues](#low-issues)
  - [L-1: Address State Variable Set Without Checks](#l-1-address-state-variable-set-without-checks)
  - [L-2: Using `block.timestamp` for swap deadline offers no protection](#l-2-using-blocktimestamp-for-swap-deadline-offers-no-protection)
  - [L-3: PUSH0 Opcode](#l-3-push0-opcode)
  - [L-4: State Variable Could Be Immutable](#l-4-state-variable-could-be-immutable)
  - [L-5: Unchecked Return](#l-5-unchecked-return)


# Summary

## Files Summary

| Key | Value |
| --- | --- |
| .sol Files | 2 |
| Total nSLOC | 200 |


## Files Details

| Filepath | nSLOC |
| --- | --- |
| src/uniswap/UniswapV2Swapper.sol | 50 |
| src/uniswap/UniswapV3Swapper.sol | 150 |
| **Total** | **200** |


## Issue Summary

| Category | No. of Issues |
| --- | --- |
| High | 0 |
| Low | 5 |


# Low Issues

## L-1: Address State Variable Set Without Checks

Check for `address(0)` when assigning values to address state variables.

<details><summary>1 Found Instances</summary>


- Found in src/uniswap/UniswapV2Swapper.sol [Line: 11](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L11)

	```solidity
	        s_router = router;
	```

</details>



## L-2: Using `block.timestamp` for swap deadline offers no protection

In the PoS model, proposers know well in advance if they will propose one or consecutive blocks ahead of time. In such a scenario, a malicious validator can hold back the transaction and execute it at a more favourable block number.Consider allowing function caller to specify swap deadline input parameter.

<details><summary>16 Found Instances</summary>


- Found in src/uniswap/UniswapV2Swapper.sol [Line: 23](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L23)

	```solidity
	        router1.swapExactTokensForTokens(amountIn, amountOutMin, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 24](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L24)

	```solidity
	        router1.swapTokensForExactTokens(amountOut, amountInMax, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 25](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L25)

	```solidity
	        router1.swapExactETHForTokens(amountOutMin, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 26](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L26)

	```solidity
	        router1.swapTokensForExactETH(amountOut, amountInMax, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 27](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L27)

	```solidity
	        router1.swapExactTokensForETH(amountIn, amountOutMin, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 31](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L31)

	```solidity
	        router2.swapExactTokensForTokensSupportingFeeOnTransferTokens(amountIn, amountOutMin, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 32](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L32)

	```solidity
	        router2.swapExactETHForTokensSupportingFeeOnTransferTokens(amountOutMin, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 33](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L33)

	```solidity
	        router2.swapExactTokensForETHSupportingFeeOnTransferTokens(amountIn, amountOutMin, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV3Swapper.sol [Line: 52](../tests/contract-playground/src/uniswap/UniswapV3Swapper.sol#L52)

	```solidity
	        ExactInputSingleParams memory exactInputSingleParams = ExactInputSingleParams(
	```

- Found in src/uniswap/UniswapV3Swapper.sol [Line: 55](../tests/contract-playground/src/uniswap/UniswapV3Swapper.sol#L55)

	```solidity
	        exactInputSingleParams = ExactInputSingleParams({
	```

- Found in src/uniswap/UniswapV3Swapper.sol [Line: 66](../tests/contract-playground/src/uniswap/UniswapV3Swapper.sol#L66)

	```solidity
	        ExactInputParams memory exactInputParams = ExactInputParams(
	```

- Found in src/uniswap/UniswapV3Swapper.sol [Line: 69](../tests/contract-playground/src/uniswap/UniswapV3Swapper.sol#L69)

	```solidity
	        exactInputParams = ExactInputParams({
	```

- Found in src/uniswap/UniswapV3Swapper.sol [Line: 77](../tests/contract-playground/src/uniswap/UniswapV3Swapper.sol#L77)

	```solidity
	        ExactOutputSingleParams memory exactOutputSingleParams = ExactOutputSingleParams(
	```

- Found in src/uniswap/UniswapV3Swapper.sol [Line: 80](../tests/contract-playground/src/uniswap/UniswapV3Swapper.sol#L80)

	```solidity
	        exactOutputSingleParams = ExactOutputSingleParams({
	```

- Found in src/uniswap/UniswapV3Swapper.sol [Line: 91](../tests/contract-playground/src/uniswap/UniswapV3Swapper.sol#L91)

	```solidity
	        ExactOutputParams memory exactOutputParams = ExactOutputParams(
	```

- Found in src/uniswap/UniswapV3Swapper.sol [Line: 94](../tests/contract-playground/src/uniswap/UniswapV3Swapper.sol#L94)

	```solidity
	        exactOutputParams = ExactOutputParams({
	```

</details>



## L-3: PUSH0 Opcode

Solc compiler version 0.8.20 switches the default target EVM version to Shanghai, which means that the generated bytecode will include PUSH0 opcodes. Be sure to select the appropriate EVM version in case you intend to deploy on a chain other than mainnet like L2 chains that may not support PUSH0, otherwise deployment of your contracts will fail.

<details><summary>2 Found Instances</summary>


- Found in src/uniswap/UniswapV2Swapper.sol [Line: 2](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L2)

	```solidity
	pragma solidity 0.8.20;
	```

- Found in src/uniswap/UniswapV3Swapper.sol [Line: 2](../tests/contract-playground/src/uniswap/UniswapV3Swapper.sol#L2)

	```solidity
	pragma solidity 0.8.20;
	```

</details>



## L-4: State Variable Could Be Immutable

State variables that are only changed in the constructor should be declared immutable to save gas. Add the `immutable` attribute to state variables that are only changed in the constructor

<details><summary>1 Found Instances</summary>


- Found in src/uniswap/UniswapV2Swapper.sol [Line: 8](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L8)

	```solidity
	    address private s_router;
	```

</details>



## L-5: Unchecked Return

Function returns a value but it is ignored. Consider checking the return value.

<details><summary>11 Found Instances</summary>


- Found in src/uniswap/UniswapV2Swapper.sol [Line: 23](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L23)

	```solidity
	        router1.swapExactTokensForTokens(amountIn, amountOutMin, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 24](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L24)

	```solidity
	        router1.swapTokensForExactTokens(amountOut, amountInMax, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 25](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L25)

	```solidity
	        router1.swapExactETHForTokens(amountOutMin, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 26](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L26)

	```solidity
	        router1.swapTokensForExactETH(amountOut, amountInMax, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 27](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L27)

	```solidity
	        router1.swapExactTokensForETH(amountIn, amountOutMin, path, to, block.timestamp);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 46](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L46)

	```solidity
	        router1.swapExactTokensForTokens(amountIn, amountOutMin, path, to, deadline);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 47](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L47)

	```solidity
	        router1.swapTokensForExactTokens(amountOut, amountInMax, path, to, deadline);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 48](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L48)

	```solidity
	        router1.swapExactETHForTokens(amountOutMin, path, to, deadline);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 49](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L49)

	```solidity
	        router1.swapTokensForExactETH(amountOut, amountInMax, path, to, deadline);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 50](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L50)

	```solidity
	        router1.swapExactTokensForETH(amountIn, amountOutMin, path, to, deadline);
	```

- Found in src/uniswap/UniswapV2Swapper.sol [Line: 51](../tests/contract-playground/src/uniswap/UniswapV2Swapper.sol#L51)

	```solidity
	        router1.swapETHForExactTokens(amountOut, path, to, deadline);
	```

</details>



