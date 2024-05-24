// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22;

import { ISablierV2MerkleLL } from "src/interfaces/ISablierV2MerkleLL.sol";
import { ISablierV2MerkleLT } from "src/interfaces/ISablierV2MerkleLT.sol";

import { Integration_Test } from "../Integration.t.sol";

abstract contract MerkleLockup_Integration_Test is Integration_Test {
    function setUp() public virtual override {
        Integration_Test.setUp();

        // Create the default MerkleLockup contracts.
        merkleLL = createMerkleLL();
        merkleLT = createMerkleLT();

        // Fund the MerkleLockup contracts.
        deal({ token: address(dai), to: address(merkleLL), give: defaults.AGGREGATE_AMOUNT() });
        deal({ token: address(dai), to: address(merkleLT), give: defaults.AGGREGATE_AMOUNT() });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                    MERKLE-LL
    //////////////////////////////////////////////////////////////////////////*/

    function claimLL() internal returns (uint256) {
        return merkleLL.claim({
            index: defaults.INDEX1(),
            recipient: users.recipient1,
            amount: defaults.CLAIM_AMOUNT(),
            merkleProof: defaults.index1Proof()
        });
    }

    function createMerkleLL() internal returns (ISablierV2MerkleLL) {
        return createMerkleLL(users.admin, defaults.EXPIRATION());
    }

    function createMerkleLL(address admin) internal returns (ISablierV2MerkleLL) {
        return createMerkleLL(admin, defaults.EXPIRATION());
    }

    function createMerkleLL(uint40 expiration) internal returns (ISablierV2MerkleLL) {
        return createMerkleLL(users.admin, expiration);
    }

    function createMerkleLL(address admin, uint40 expiration) internal returns (ISablierV2MerkleLL) {
        // Increment the CREATE nonce for factory contract.
        ++merkleLockupFactoryNonce;

        return merkleLockupFactory.createMerkleLL({
            baseParams: defaults.baseParams(admin, dai, expiration, defaults.MERKLE_ROOT()),
            lockupLinear: lockupLinear,
            streamDurations: defaults.durations(),
            aggregateAmount: defaults.AGGREGATE_AMOUNT(),
            recipientCount: defaults.RECIPIENT_COUNT()
        });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                    MERKLE-LT
    //////////////////////////////////////////////////////////////////////////*/

    function claimLT() internal returns (uint256) {
        return merkleLT.claim({
            index: defaults.INDEX1(),
            recipient: users.recipient1,
            amount: defaults.CLAIM_AMOUNT(),
            merkleProof: defaults.index1Proof()
        });
    }

    function createMerkleLT() internal returns (ISablierV2MerkleLT) {
        return createMerkleLT(users.admin, defaults.EXPIRATION());
    }

    function createMerkleLT(address admin) internal returns (ISablierV2MerkleLT) {
        return createMerkleLT(admin, defaults.EXPIRATION());
    }

    function createMerkleLT(uint40 expiration) internal returns (ISablierV2MerkleLT) {
        return createMerkleLT(users.admin, expiration);
    }

    function createMerkleLT(address admin, uint40 expiration) internal returns (ISablierV2MerkleLT) {
        // Increment the CREATE nonce for factory contract.
        ++merkleLockupFactoryNonce;

        return merkleLockupFactory.createMerkleLT({
            baseParams: defaults.baseParams(admin, dai, expiration, defaults.MERKLE_ROOT()),
            lockupTranched: lockupTranched,
            tranchesWithPercentages: defaults.tranchesWithPercentages(),
            aggregateAmount: defaults.AGGREGATE_AMOUNT(),
            recipientCount: defaults.RECIPIENT_COUNT()
        });
    }
}
