// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IInbox {
    function createRetryableTicket(
        address to,
        uint256 l2CallValue,
        uint256 maxSubmissionCost,
        address excessFeeRefundAddress,
        address callValueRefundAddress,
        uint256 gasLimit,
        uint256 maxFeePerGas,
        bytes calldata data
    ) external payable returns (uint256);

    function outboundTransferCustomRefund(
        address l1Token,
        address to,
        uint256 amount,
        uint256 maxGas,
        uint256 gasPriceBid,
        bytes calldata data,
        address excessFeeRefundAddress
    ) external payable returns (bytes memory);

    function unsafeCreateRetryableTicket(
        address to,
        uint256 l2CallValue,
        uint256 maxSubmissionCost,
        address excessFeeRefundAddress,
        address callValueRefundAddress,
        uint256 gasLimit,
        uint256 maxFeePerGas,
        bytes calldata data
    ) external payable returns (uint256);
}

contract L1 {
    address public inbox;
    address public l2contract;
    uint256 public maxSubmissionCost;
    uint256 public gasLimit;
    uint256 public maxFeePerGas;
    uint256 public gas;

    constructor(
        address _inbox,
        address _l2contract,
        uint256 _maxSubmissionCost,
        uint256 _gasLimit,
        uint256 _maxFeePerGas,
        uint256 _gas
    ) {
        inbox = _inbox;
        l2contract = _l2contract;
        maxSubmissionCost = _maxSubmissionCost;
        gasLimit = _gasLimit;
        maxFeePerGas = _maxFeePerGas;
        gas = _gas;
    }

    // BAD (2 occurrences)
    function doStuffOnL2() external {
        // Retryable A
        IInbox(inbox).createRetryableTicket({
            to: l2contract,
            l2CallValue: 0,
            maxSubmissionCost: maxSubmissionCost,
            excessFeeRefundAddress: msg.sender,
            callValueRefundAddress: msg.sender,
            gasLimit: gasLimit,
            maxFeePerGas: maxFeePerGas,
            data: abi.encodeWithSelector(L2.claim_rewards.selector)
        });

        // Retryable B
        IInbox(inbox).createRetryableTicket({
            to: l2contract,
            l2CallValue: 0,
            maxSubmissionCost: maxSubmissionCost,
            excessFeeRefundAddress: msg.sender,
            callValueRefundAddress: msg.sender,
            gasLimit: gas,
            maxFeePerGas: maxFeePerGas,
            data: abi.encodeWithSelector(L2.unstake.selector)
        });
    }

    // BAD (2 occurrences)
    function doStuffOnL2Alternative() external {
        // Retryable A
        IInbox(inbox).createRetryableTicket({
            to: l2contract,
            l2CallValue: 0,
            maxSubmissionCost: maxSubmissionCost,
            excessFeeRefundAddress: msg.sender,
            callValueRefundAddress: msg.sender,
            gasLimit: gasLimit,
            maxFeePerGas: maxFeePerGas,
            data: abi.encodeWithSelector(L2.claim_rewards.selector)
        });
        _helper();
    }

    function _helper() internal {
        // Retryable B
        IInbox(inbox).createRetryableTicket({
            to: l2contract,
            l2CallValue: 0,
            maxSubmissionCost: maxSubmissionCost,
            excessFeeRefundAddress: msg.sender,
            callValueRefundAddress: msg.sender,
            gasLimit: gas,
            maxFeePerGas: maxFeePerGas,
            data: abi.encodeWithSelector(L2.unstake.selector)
        });
    }

    // GOOD (1 occurrence only)
    function customRefund(
        address l1Token,
        address to,
        uint256 amount,
        uint256 maxGas,
        uint256 gasPriceBid,
        bytes calldata data,
        address excessFeeRefundAddress
    ) external payable {
        IInbox(inbox).outboundTransferCustomRefund(
            l1Token,
            to,
            amount,
            maxGas,
            gasPriceBid,
            data,
            excessFeeRefundAddress
        );
    }

    // GOOD (1 occurrence only)
    function unsafeRetryable(
        address to,
        uint256 l2CallValue,
        uint256 maxSubmissionCost,
        address excessFeeRefundAddress,
        address callValueRefundAddress,
        uint256 gasLimit,
        uint256 maxFeePerGas,
        bytes calldata data
    ) external payable {
        IInbox(inbox).unsafeCreateRetryableTicket(
            to,
            l2CallValue,
            maxSubmissionCost,
            excessFeeRefundAddress,
            callValueRefundAddress,
            gasLimit,
            maxFeePerGas,
            data
        );
    }
}

contract L2 {
    mapping(address => uint256) public balance;

    function claim_rewards() public {
        // rewards are computed based on balance and staking period
        uint256 unclaimed_rewards = _compute_and_update_rewards(msg.sender);
        // transfer rewards (in ETH, for example) to the caller
        payable(msg.sender).transfer(unclaimed_rewards);
    }

    // Call claim_rewards before unstaking, otherwise you lose your rewards
    function unstake() public {
        _free_rewards(msg.sender); // clean up rewards-related variables
        uint256 user_balance = balance[msg.sender];
        balance[msg.sender] = 0;
        // transfer staked balance (in ETH, for example) to the caller
        payable(msg.sender).transfer(user_balance);
    }

    function _compute_and_update_rewards(
        address user
    ) internal view returns (uint256) {
        // Implementation of reward computation
        // Example: return some computed value based on user's balance and staking period
        uint256 rewards = balance[user] / 10; // Simplified example
        // Update any necessary state here
        return rewards;
    }

    function _free_rewards(address user) internal {
        // Implementation of cleanup logic for rewards
        // Example: reset some state related to the user's rewards
    }

    receive() external payable {
        // Allow the contract to receive ETH
    }
}
