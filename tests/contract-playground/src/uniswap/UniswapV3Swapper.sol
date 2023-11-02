// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

struct ExactInputSingleParams {
    address tokenIn;
    address tokenOut;
    uint24 fee;
    address recipient;
    uint256 deadline;
    uint256 amountIn;
    uint256 amountOutMinimum;
    uint160 sqrtPriceLimitX96;
}

struct ExactInputParams {
    bytes path;
    address recipient;
    uint256 deadline;
    uint256 amountIn;
    uint256 amountOutMinimum;
}

struct ExactOutputSingleParams {
    address tokenIn;
    address tokenOut;
    uint24 fee;
    address recipient;
    uint256 deadline;
    uint256 amountOut;
    uint256 amountInMaximum;
    uint160 sqrtPriceLimitX96;
}

struct ExactOutputParams {
    bytes path;
    address recipient;
    uint256 deadline;
    uint256 amountOut;
    uint256 amountInMaximum;
}

contract UniswapV3Swapper {
    function badSwaps(
        address tokenIn,
        address tokenOut,
        uint24 fee,
        address recipient,
        uint256 amountIn,
        uint256 amountOutMinimum,
        uint160 sqrtPriceLimitX96
    ) external view {
        ExactInputSingleParams memory exactInputSingleParams = ExactInputSingleParams(
            tokenIn, tokenOut, fee, recipient, block.timestamp, amountIn, amountOutMinimum, sqrtPriceLimitX96
        );
        exactInputSingleParams = ExactInputSingleParams({
            tokenIn: tokenIn,
            tokenOut: tokenOut,
            fee: fee,
            recipient: recipient,
            deadline: block.timestamp,
            amountIn: amountIn,
            amountOutMinimum: amountOutMinimum,
            sqrtPriceLimitX96: sqrtPriceLimitX96
        });

        ExactInputParams memory exactInputParams = ExactInputParams(
            abi.encodePacked(tokenIn, tokenOut), recipient, block.timestamp, amountIn, amountOutMinimum
        );
        exactInputParams = ExactInputParams({
            path: abi.encodePacked(tokenIn, tokenOut),
            recipient: recipient,
            deadline: block.timestamp,
            amountIn: amountIn,
            amountOutMinimum: amountOutMinimum
        });

        ExactOutputSingleParams memory exactOutputSingleParams = ExactOutputSingleParams(
            tokenIn, tokenOut, fee, recipient, block.timestamp, amountOutMinimum, amountIn, sqrtPriceLimitX96
        );
        exactOutputSingleParams = ExactOutputSingleParams({
            tokenIn: tokenIn,
            tokenOut: tokenOut,
            fee: fee,
            recipient: recipient,
            deadline: block.timestamp,
            amountOut: amountOutMinimum,
            amountInMaximum: amountIn,
            sqrtPriceLimitX96: sqrtPriceLimitX96
        });

        ExactOutputParams memory exactOutputParams = ExactOutputParams(
            abi.encodePacked(tokenIn, tokenOut), recipient, block.timestamp, amountOutMinimum, amountIn
        );
        exactOutputParams = ExactOutputParams({
            path: abi.encodePacked(tokenIn, tokenOut),
            recipient: recipient,
            deadline: block.timestamp,
            amountOut: amountOutMinimum,
            amountInMaximum: amountIn
        });
        
    }

    function goodSwaps(
        address tokenIn,
        address tokenOut,
        uint24 fee,
        address recipient,
        uint256 amountIn,
        uint256 amountOutMinimum,
        uint160 sqrtPriceLimitX96,
        uint256 deadline
    ) external pure {
        ExactInputSingleParams memory exactInputSingleParams = ExactInputSingleParams(
            tokenIn, tokenOut, fee, recipient, deadline, amountIn, amountOutMinimum, sqrtPriceLimitX96
        );
        exactInputSingleParams = ExactInputSingleParams({
            tokenIn: tokenIn,
            tokenOut: tokenOut,
            fee: fee,
            recipient: recipient,
            deadline: deadline,
            amountIn: amountIn,
            amountOutMinimum: amountOutMinimum,
            sqrtPriceLimitX96: sqrtPriceLimitX96
        });

        ExactInputParams memory exactInputParams = ExactInputParams(
            abi.encodePacked(tokenIn, tokenOut), recipient, deadline, amountIn, amountOutMinimum
        );
        exactInputParams = ExactInputParams({
            path: abi.encodePacked(tokenIn, tokenOut),
            recipient: recipient,
            deadline: deadline,
            amountIn: amountIn,
            amountOutMinimum: amountOutMinimum
        });

        ExactOutputSingleParams memory exactOutputSingleParams = ExactOutputSingleParams(
            tokenIn, tokenOut, fee, recipient, deadline, amountOutMinimum, amountIn, sqrtPriceLimitX96
        );
        exactOutputSingleParams = ExactOutputSingleParams({
            tokenIn: tokenIn,
            tokenOut: tokenOut,
            fee: fee,
            recipient: recipient,
            deadline: deadline,
            amountOut: amountOutMinimum,
            amountInMaximum: amountIn,
            sqrtPriceLimitX96: sqrtPriceLimitX96
        });

        ExactOutputParams memory exactOutputParams = ExactOutputParams(
            abi.encodePacked(tokenIn, tokenOut), recipient, deadline, amountOutMinimum, amountIn
        );
        exactOutputParams = ExactOutputParams({
            path: abi.encodePacked(tokenIn, tokenOut),
            recipient: recipient,
            deadline: deadline,
            amountOut: amountOutMinimum,
            amountInMaximum: amountIn
        });
    }
}
