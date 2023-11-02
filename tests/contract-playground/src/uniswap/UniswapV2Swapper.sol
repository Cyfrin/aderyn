// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {IUniswapV2Router01} from "../../lib/v2-periphery/contracts/interfaces/IUniswapV2Router01.sol";
import {IUniswapV2Router02} from "../../lib/v2-periphery/contracts/interfaces/IUniswapV2Router02.sol";

contract UniswapV2Swapper {
    address private s_router;

    constructor(address router) {
        s_router = router;
    }

    function badSwaps(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 amountOut,
        uint256 amountInMax
    ) external {
        IUniswapV2Router01 router1 = IUniswapV2Router01(s_router);
        router1.swapExactTokensForTokens(amountIn, amountOutMin, path, to, block.timestamp);
        router1.swapTokensForExactTokens(amountOut, amountInMax, path, to, block.timestamp);
        router1.swapExactETHForTokens(amountOutMin, path, to, block.timestamp);
        router1.swapTokensForExactETH(amountOut, amountInMax, path, to, block.timestamp);
        router1.swapExactTokensForETH(amountIn, amountOutMin, path, to, block.timestamp);
        router1.swapETHForExactTokens(amountOut, path, to, block.timestamp);

        IUniswapV2Router02 router2 = IUniswapV2Router02(s_router);
        router2.swapExactTokensForTokensSupportingFeeOnTransferTokens(amountIn, amountOutMin, path, to, block.timestamp);
        router2.swapExactETHForTokensSupportingFeeOnTransferTokens(amountOutMin, path, to, block.timestamp);
        router2.swapExactTokensForETHSupportingFeeOnTransferTokens(amountIn, amountOutMin, path, to, block.timestamp);
    }

    function goodSwaps(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 amountOut,
        uint256 amountInMax,
        uint256 deadline
    ) external {
        IUniswapV2Router01 router1 = IUniswapV2Router01(s_router);
        router1.swapExactTokensForTokens(amountIn, amountOutMin, path, to, deadline);
        router1.swapTokensForExactTokens(amountOut, amountInMax, path, to, deadline);
        router1.swapExactETHForTokens(amountOutMin, path, to, deadline);
        router1.swapTokensForExactETH(amountOut, amountInMax, path, to, deadline);
        router1.swapExactTokensForETH(amountIn, amountOutMin, path, to, deadline);
        router1.swapETHForExactTokens(amountOut, path, to, deadline);

        IUniswapV2Router02 router2 = IUniswapV2Router02(s_router);
        router2.swapExactTokensForTokensSupportingFeeOnTransferTokens(amountIn, amountOutMin, path, to, deadline);
        router2.swapExactETHForTokensSupportingFeeOnTransferTokens(amountOutMin, path, to, deadline);
        router2.swapExactTokensForETHSupportingFeeOnTransferTokens(amountIn, amountOutMin, path, to, deadline);
    }
}
