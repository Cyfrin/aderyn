// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Arrays } from "@openzeppelin/contracts/utils/Arrays.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { Lockup, LockupTranched } from "@sablier/v2-core/src/types/DataTypes.sol";

import { ISablierV2MerkleLT } from "src/interfaces/ISablierV2MerkleLT.sol";
import { MerkleLockup } from "src/types/DataTypes.sol";

import { MerkleBuilder } from "../../utils/MerkleBuilder.sol";
import { Fork_Test } from "../Fork.t.sol";

abstract contract MerkleLT_Fork_Test is Fork_Test {
    using MerkleBuilder for uint256[];

    constructor(IERC20 asset_) Fork_Test(asset_) { }

    function setUp() public virtual override {
        Fork_Test.setUp();
    }

    /// @dev Encapsulates the data needed to compute a Merkle tree leaf.
    struct LeafData {
        uint256 index;
        uint256 recipientSeed;
        uint128 amount;
    }

    struct Params {
        address admin;
        uint40 expiration;
        LeafData[] leafData;
        uint256 posBeforeSort;
    }

    struct Vars {
        LockupTranched.StreamLT actualStream;
        uint256 actualStreamId;
        LockupTranched.Tranche[] actualTranches;
        uint256 aggregateAmount;
        uint128[] amounts;
        MerkleLockup.ConstructorParams baseParams;
        uint128 clawbackAmount;
        address expectedLT;
        LockupTranched.StreamLT expectedStream;
        uint256 expectedStreamId;
        uint256[] indexes;
        uint256 leafPos;
        uint256 leafToClaim;
        ISablierV2MerkleLT merkleLT;
        bytes32 merkleRoot;
        address[] recipients;
        uint256 recipientCount;
    }

    // We need the leaves as a storage variable so that we can use OpenZeppelin's {Arrays.findUpperBound}.
    uint256[] public leaves;

    function testForkFuzz_MerkleLT(Params memory params) external {
        vm.assume(params.admin != address(0) && params.admin != users.admin);
        vm.assume(params.leafData.length > 1);
        assumeNoBlacklisted({ token: address(FORK_ASSET), addr: params.admin });
        params.posBeforeSort = _bound(params.posBeforeSort, 0, params.leafData.length - 1);

        // The expiration must be either zero or greater than the block timestamp.
        if (params.expiration != 0) {
            params.expiration = boundUint40(params.expiration, getBlockTimestamp() + 1 seconds, MAX_UNIX_TIMESTAMP);
        }

        /*//////////////////////////////////////////////////////////////////////////
                                          CREATE
        //////////////////////////////////////////////////////////////////////////*/

        Vars memory vars;
        vars.recipientCount = params.leafData.length;
        vars.amounts = new uint128[](vars.recipientCount);
        vars.indexes = new uint256[](vars.recipientCount);
        vars.recipients = new address[](vars.recipientCount);
        for (uint256 i = 0; i < vars.recipientCount; ++i) {
            vars.indexes[i] = params.leafData[i].index;

            // Bound each leaf amount so that `aggregateAmount` does not overflow.
            vars.amounts[i] = boundUint128(params.leafData[i].amount, 1, uint128(MAX_UINT128 / vars.recipientCount - 1));
            vars.aggregateAmount += vars.amounts[i];

            // Avoid zero recipient addresses.
            uint256 boundedRecipientSeed = _bound(params.leafData[i].recipientSeed, 1, type(uint160).max);
            vars.recipients[i] = address(uint160(boundedRecipientSeed));
        }

        leaves = new uint256[](vars.recipientCount);
        leaves = MerkleBuilder.computeLeaves(vars.indexes, vars.recipients, vars.amounts);

        // Sort the leaves in ascending order to match the production environment.
        MerkleBuilder.sortLeaves(leaves);
        vars.merkleRoot = getRoot(leaves.toBytes32());

        vars.expectedLT = vm.computeCreateAddress(address(merkleLockupFactory), ++merkleLockupFactoryNonce);

        vars.baseParams = defaults.baseParams({
            admin: params.admin,
            asset_: FORK_ASSET,
            merkleRoot: vars.merkleRoot,
            expiration: params.expiration
        });

        vm.expectEmit({ emitter: address(merkleLockupFactory) });
        emit CreateMerkleLT({
            merkleLT: ISablierV2MerkleLT(vars.expectedLT),
            baseParams: vars.baseParams,
            lockupTranched: lockupTranched,
            tranchesWithPercentages: defaults.tranchesWithPercentages(),
            totalDuration: defaults.TOTAL_DURATION(),
            aggregateAmount: vars.aggregateAmount,
            recipientCount: vars.recipientCount
        });

        vars.merkleLT = merkleLockupFactory.createMerkleLT({
            baseParams: vars.baseParams,
            lockupTranched: lockupTranched,
            tranchesWithPercentages: defaults.tranchesWithPercentages(),
            aggregateAmount: vars.aggregateAmount,
            recipientCount: vars.recipientCount
        });

        // Fund the MerkleLockup contract.
        deal({ token: address(FORK_ASSET), to: address(vars.merkleLT), give: vars.aggregateAmount });

        assertGt(address(vars.merkleLT).code.length, 0, "MerkleLT contract not created");
        assertEq(address(vars.merkleLT), vars.expectedLT, "MerkleLT contract does not match computed address");

        /*//////////////////////////////////////////////////////////////////////////
                                          CLAIM
        //////////////////////////////////////////////////////////////////////////*/

        assertFalse(vars.merkleLT.hasClaimed(vars.indexes[params.posBeforeSort]));

        vars.leafToClaim = MerkleBuilder.computeLeaf(
            vars.indexes[params.posBeforeSort],
            vars.recipients[params.posBeforeSort],
            vars.amounts[params.posBeforeSort]
        );
        vars.leafPos = Arrays.findUpperBound(leaves, vars.leafToClaim);

        vars.expectedStreamId = lockupTranched.nextStreamId();
        emit Claim(
            vars.indexes[params.posBeforeSort],
            vars.recipients[params.posBeforeSort],
            vars.amounts[params.posBeforeSort],
            vars.expectedStreamId
        );
        vars.actualStreamId = vars.merkleLT.claim({
            index: vars.indexes[params.posBeforeSort],
            recipient: vars.recipients[params.posBeforeSort],
            amount: vars.amounts[params.posBeforeSort],
            merkleProof: getProof(leaves.toBytes32(), vars.leafPos)
        });

        vars.actualStream = lockupTranched.getStream(vars.actualStreamId);
        vars.expectedStream = LockupTranched.StreamLT({
            amounts: Lockup.Amounts({ deposited: vars.amounts[params.posBeforeSort], refunded: 0, withdrawn: 0 }),
            asset: FORK_ASSET,
            endTime: getBlockTimestamp() + defaults.TOTAL_DURATION(),
            isCancelable: defaults.CANCELABLE(),
            isDepleted: false,
            isStream: true,
            isTransferable: defaults.TRANSFERABLE(),
            recipient: vars.recipients[params.posBeforeSort],
            sender: params.admin,
            startTime: getBlockTimestamp(),
            tranches: defaults.tranches({ totalAmount: vars.amounts[params.posBeforeSort] }),
            wasCanceled: false
        });

        assertTrue(vars.merkleLT.hasClaimed(vars.indexes[params.posBeforeSort]));
        assertEq(vars.actualStreamId, vars.expectedStreamId);
        assertEq(vars.actualStream, vars.expectedStream);

        /*//////////////////////////////////////////////////////////////////////////
                                        CLAWBACK
        //////////////////////////////////////////////////////////////////////////*/

        if (params.expiration > 0) {
            vars.clawbackAmount = uint128(FORK_ASSET.balanceOf(address(vars.merkleLT)));
            vm.warp({ newTimestamp: uint256(params.expiration) + 1 seconds });

            resetPrank({ msgSender: params.admin });
            expectCallToTransfer({ asset_: address(FORK_ASSET), to: params.admin, amount: vars.clawbackAmount });
            vm.expectEmit({ emitter: address(vars.merkleLT) });
            emit Clawback({ to: params.admin, admin: params.admin, amount: vars.clawbackAmount });
            vars.merkleLT.clawback({ to: params.admin, amount: vars.clawbackAmount });
        }
    }
}
