// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Arrays } from "@openzeppelin/contracts/utils/Arrays.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { Lockup, LockupLinear } from "@sablier/v2-core/src/types/DataTypes.sol";

import { ISablierV2MerkleLL } from "src/interfaces/ISablierV2MerkleLL.sol";
import { MerkleLockup } from "src/types/DataTypes.sol";

import { MerkleBuilder } from "../../utils/MerkleBuilder.sol";
import { Fork_Test } from "../Fork.t.sol";

abstract contract MerkleLL_Fork_Test is Fork_Test {
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
        LockupLinear.StreamLL actualStream;
        uint256 actualStreamId;
        uint256 aggregateAmount;
        uint128[] amounts;
        MerkleLockup.ConstructorParams baseParams;
        uint128 clawbackAmount;
        address expectedLL;
        LockupLinear.StreamLL expectedStream;
        uint256 expectedStreamId;
        uint256[] indexes;
        uint256 leafPos;
        uint256 leafToClaim;
        ISablierV2MerkleLL merkleLL;
        bytes32 merkleRoot;
        address[] recipients;
        uint256 recipientCount;
    }

    // We need the leaves as a storage variable so that we can use OpenZeppelin's {Arrays.findUpperBound}.
    uint256[] public leaves;

    function testForkFuzz_MerkleLL(Params memory params) external {
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

        vars.expectedLL = vm.computeCreateAddress(address(merkleLockupFactory), ++merkleLockupFactoryNonce);

        vars.baseParams = defaults.baseParams({
            admin: params.admin,
            asset_: FORK_ASSET,
            merkleRoot: vars.merkleRoot,
            expiration: params.expiration
        });

        vm.expectEmit({ emitter: address(merkleLockupFactory) });
        emit CreateMerkleLL({
            merkleLL: ISablierV2MerkleLL(vars.expectedLL),
            baseParams: vars.baseParams,
            lockupLinear: lockupLinear,
            streamDurations: defaults.durations(),
            aggregateAmount: vars.aggregateAmount,
            recipientCount: vars.recipientCount
        });

        vars.merkleLL = merkleLockupFactory.createMerkleLL({
            baseParams: vars.baseParams,
            lockupLinear: lockupLinear,
            streamDurations: defaults.durations(),
            aggregateAmount: vars.aggregateAmount,
            recipientCount: vars.recipientCount
        });

        // Fund the MerkleLockup contract.
        deal({ token: address(FORK_ASSET), to: address(vars.merkleLL), give: vars.aggregateAmount });

        assertGt(address(vars.merkleLL).code.length, 0, "MerkleLL contract not created");
        assertEq(address(vars.merkleLL), vars.expectedLL, "MerkleLL contract does not match computed address");

        /*//////////////////////////////////////////////////////////////////////////
                                          CLAIM
        //////////////////////////////////////////////////////////////////////////*/

        assertFalse(vars.merkleLL.hasClaimed(vars.indexes[params.posBeforeSort]));

        vars.leafToClaim = MerkleBuilder.computeLeaf(
            vars.indexes[params.posBeforeSort],
            vars.recipients[params.posBeforeSort],
            vars.amounts[params.posBeforeSort]
        );
        vars.leafPos = Arrays.findUpperBound(leaves, vars.leafToClaim);

        vars.expectedStreamId = lockupLinear.nextStreamId();
        emit Claim(
            vars.indexes[params.posBeforeSort],
            vars.recipients[params.posBeforeSort],
            vars.amounts[params.posBeforeSort],
            vars.expectedStreamId
        );
        vars.actualStreamId = vars.merkleLL.claim({
            index: vars.indexes[params.posBeforeSort],
            recipient: vars.recipients[params.posBeforeSort],
            amount: vars.amounts[params.posBeforeSort],
            merkleProof: getProof(leaves.toBytes32(), vars.leafPos)
        });

        vars.actualStream = lockupLinear.getStream(vars.actualStreamId);
        vars.expectedStream = LockupLinear.StreamLL({
            amounts: Lockup.Amounts({ deposited: vars.amounts[params.posBeforeSort], refunded: 0, withdrawn: 0 }),
            asset: FORK_ASSET,
            cliffTime: getBlockTimestamp() + defaults.CLIFF_DURATION(),
            endTime: getBlockTimestamp() + defaults.TOTAL_DURATION(),
            isCancelable: defaults.CANCELABLE(),
            isDepleted: false,
            isStream: true,
            isTransferable: defaults.TRANSFERABLE(),
            recipient: vars.recipients[params.posBeforeSort],
            sender: params.admin,
            startTime: getBlockTimestamp(),
            wasCanceled: false
        });

        assertTrue(vars.merkleLL.hasClaimed(vars.indexes[params.posBeforeSort]));
        assertEq(vars.actualStreamId, vars.expectedStreamId);
        assertEq(vars.actualStream, vars.expectedStream);

        /*//////////////////////////////////////////////////////////////////////////
                                        CLAWBACK
        //////////////////////////////////////////////////////////////////////////*/

        if (params.expiration > 0) {
            vars.clawbackAmount = uint128(FORK_ASSET.balanceOf(address(vars.merkleLL)));
            vm.warp({ newTimestamp: uint256(params.expiration) + 1 seconds });

            resetPrank({ msgSender: params.admin });
            expectCallToTransfer({ asset_: address(FORK_ASSET), to: params.admin, amount: vars.clawbackAmount });
            vm.expectEmit({ emitter: address(vars.merkleLL) });
            emit Clawback({ to: params.admin, admin: params.admin, amount: vars.clawbackAmount });
            vars.merkleLL.clawback({ to: params.admin, amount: vars.clawbackAmount });
        }
    }
}
