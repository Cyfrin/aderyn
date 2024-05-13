// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

interface ExternalContractInterface {
    function externalFunctionCall(address newOwner) external;
}

contract ExternalCalls {
    address private target;
    ExternalContractInterface private targetContract;

    constructor(address setTarget) {
        require(setTarget != address(0), "Invalid target");
        target = setTarget;
        targetContract = ExternalContractInterface(setTarget);
    }

    // target (address variable)

    function rawCallFromStorage(bytes calldata data) external returns (bytes memory) {
        (bool success, bytes memory result) = target.call(data);
        require(success, "External call failed");
        return result;
    }

    function rawCallFromParameter(address myTarget, bytes calldata data) external returns (bytes memory) {
        (bool success, bytes memory result) = myTarget.call(data);
        require(success, "External call failed");
        return result;
    }

    function rawDelegateCallFromStorage(bytes calldata data) external returns (bytes memory) {
        (bool success, bytes memory result) = target.delegatecall(data);
        require(success, "External delegate call failed");
        return result;
    }

    function rawDelegateCallFromParameter(address myTarget, bytes calldata data) external returns (bytes memory) {
        (bool success, bytes memory result) = myTarget.delegatecall(data);
        require(success, "External delegate call failed");
        return result;
    }

    // targetContract (ExternalContractInterface variable)

    function rawCallToInterfaceFromStorage(bytes calldata data) external returns (bytes memory) {
        (bool success, bytes memory result) = address(targetContract).call(data);
        require(success, "External call failed");
        return result;
    }

    function rawCallToInterfaceFromParameter(ExternalContractInterface myTarget, bytes calldata data) external returns (bytes memory) {
        (bool success, bytes memory result) = address(myTarget).call(data);
        require(success, "External call failed");
        return result;
    }

    function rawDelegateCallToInterfaceFromStorage(bytes calldata data) external returns (bytes memory) {
        (bool success, bytes memory result) = address(targetContract).delegatecall(data);
        require(success, "External delegate call failed");
        return result;
    }

    function rawDelegateCallToInterfaceFromParameter(ExternalContractInterface myTarget, bytes calldata data) external returns (bytes memory) {
        (bool success, bytes memory result) = address(myTarget).delegatecall(data);
        require(success, "External delegate call failed");
        return result;
    }

    // explicit function calls

    function externalFunctionCallFromStorage(address newOwner) external {
        targetContract.externalFunctionCall(newOwner);
    }

    function externalFunctionCallFromParameter(ExternalContractInterface myTarget, address newOwner) external {
        myTarget.externalFunctionCall(newOwner);
    }

    function externalFunctionCallUsingAddressFromStorage(address newOwner) external {
        ExternalContractInterface(target).externalFunctionCall(newOwner);
    }

    function externalFunctionCallUsingAddressFromParameter(ExternalContractInterface myTarget, address newOwner) external {
        ExternalContractInterface(myTarget).externalFunctionCall(newOwner);
    }
}