// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.22 <0.9.0;

import { Arrays } from "@openzeppelin/contracts/utils/Arrays.sol";
import { Lockup, LockupTranched } from "@sablier/v2-core/src/types/DataTypes.sol";

import { ISablierV2MerkleLT } from "src/interfaces/ISablierV2MerkleLT.sol";
import { Errors } from "src/libraries/Errors.sol";
import { MerkleLockup } from "src/types/DataTypes.sol";

import { MerkleBuilder } from "../../../../utils/MerkleBuilder.sol";
import { Merkle } from "../../../../utils/Murky.sol";

import { MerkleLockup_Integration_Test } from "../../MerkleLockup.t.sol";

contract Claim_Integration_Test is Merkle, MerkleLockup_Integration_Test {
    using MerkleBuilder for uint256[];

    function setUp() public virtual override {
        MerkleLockup_Integration_Test.setUp();
    }

    function test_RevertGiven_CampaignExpired() external {
        uint40 expiration = defaults.EXPIRATION();
        uint256 warpTime = expiration + 1 seconds;
        bytes32[] memory merkleProof;
        vm.warp({ newTimestamp: warpTime });
        vm.expectRevert(
            abi.encodeWithSelector(Errors.SablierV2MerkleLockup_CampaignExpired.selector, warpTime, expiration)
        );
        merkleLT.claim({ index: 1, recipient: users.recipient1, amount: 1, merkleProof: merkleProof });
    }

    modifier givenCampaignNotExpired() {
        _;
    }

    function test_RevertGiven_AlreadyClaimed() external givenCampaignNotExpired {
        claimLT();
        uint256 index1 = defaults.INDEX1();
        uint128 amount = defaults.CLAIM_AMOUNT();
        bytes32[] memory merkleProof = defaults.index1Proof();
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2MerkleLockup_StreamClaimed.selector, index1));
        merkleLT.claim(index1, users.recipient1, amount, merkleProof);
    }

    modifier givenNotClaimed() {
        _;
    }

    modifier givenNotIncludedInMerkleTree() {
        _;
    }

    function test_RevertWhen_InvalidIndex()
        external
        givenCampaignNotExpired
        givenNotClaimed
        givenNotIncludedInMerkleTree
    {
        uint256 invalidIndex = 1337;
        uint128 amount = defaults.CLAIM_AMOUNT();
        bytes32[] memory merkleProof = defaults.index1Proof();
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2MerkleLockup_InvalidProof.selector));
        merkleLT.claim(invalidIndex, users.recipient1, amount, merkleProof);
    }

    function test_RevertWhen_InvalidRecipient()
        external
        givenCampaignNotExpired
        givenNotClaimed
        givenNotIncludedInMerkleTree
    {
        uint256 index1 = defaults.INDEX1();
        address invalidRecipient = address(1337);
        uint128 amount = defaults.CLAIM_AMOUNT();
        bytes32[] memory merkleProof = defaults.index1Proof();
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2MerkleLockup_InvalidProof.selector));
        merkleLT.claim(index1, invalidRecipient, amount, merkleProof);
    }

    function test_RevertWhen_InvalidAmount()
        external
        givenCampaignNotExpired
        givenNotClaimed
        givenNotIncludedInMerkleTree
    {
        uint256 index1 = defaults.INDEX1();
        uint128 invalidAmount = 1337;
        bytes32[] memory merkleProof = defaults.index1Proof();
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2MerkleLockup_InvalidProof.selector));
        merkleLT.claim(index1, users.recipient1, invalidAmount, merkleProof);
    }

    function test_RevertWhen_InvalidMerkleProof()
        external
        givenCampaignNotExpired
        givenNotClaimed
        givenNotIncludedInMerkleTree
    {
        uint256 index1 = defaults.INDEX1();
        uint128 amount = defaults.CLAIM_AMOUNT();
        bytes32[] memory invalidMerkleProof = defaults.index2Proof();
        vm.expectRevert(abi.encodeWithSelector(Errors.SablierV2MerkleLockup_InvalidProof.selector));
        merkleLT.claim(index1, users.recipient1, amount, invalidMerkleProof);
    }

    modifier givenIncludedInMerkleTree() {
        _;
    }

    /// @dev Needed this variable in storage due to how the imported libraries work.
    uint256[] public leaves = new uint256[](4); // same number of recipients as in Defaults

    function test_Claim_CalculatedAmountsSumNotEqualClaimAmount()
        external
        givenCampaignNotExpired
        givenNotClaimed
        givenIncludedInMerkleTree
    {
        // Declare a claim amount that will cause a rounding error.
        uint128 claimAmount = defaults.CLAIM_AMOUNT() + 1;

        // Compute the test Merkle tree.
        leaves = defaults.getLeaves();
        uint256 leaf = MerkleBuilder.computeLeaf(defaults.INDEX1(), users.recipient1, claimAmount);
        leaves[0] = leaf;
        MerkleBuilder.sortLeaves(leaves);

        // Compute the test Merkle proof.
        uint256 pos = Arrays.findUpperBound(leaves, leaf);
        bytes32[] memory proof = getProof(leaves.toBytes32(), pos);

        /// Declare the constructor params.
        MerkleLockup.ConstructorParams memory baseParams = defaults.baseParams();
        baseParams.merkleRoot = getRoot(leaves.toBytes32());

        // Deploy a test MerkleLT contract.
        ISablierV2MerkleLT testMerkleLT = merkleLockupFactory.createMerkleLT(
            baseParams,
            lockupTranched,
            defaults.tranchesWithPercentages(),
            defaults.AGGREGATE_AMOUNT(),
            defaults.RECIPIENT_COUNT()
        );

        // Fund the MerkleLT contract.
        deal({ token: address(dai), to: address(testMerkleLT), give: defaults.AGGREGATE_AMOUNT() });

        uint256 expectedStreamId = lockupTranched.nextStreamId();
        vm.expectEmit({ emitter: address(testMerkleLT) });
        emit Claim(defaults.INDEX1(), users.recipient1, claimAmount, expectedStreamId);

        uint256 actualStreamId = testMerkleLT.claim(defaults.INDEX1(), users.recipient1, claimAmount, proof);
        LockupTranched.StreamLT memory actualStream = lockupTranched.getStream(actualStreamId);
        LockupTranched.StreamLT memory expectedStream = LockupTranched.StreamLT({
            amounts: Lockup.Amounts({ deposited: claimAmount, refunded: 0, withdrawn: 0 }),
            asset: dai,
            endTime: getBlockTimestamp() + defaults.TOTAL_DURATION(),
            isCancelable: defaults.CANCELABLE(),
            isDepleted: false,
            isStream: true,
            isTransferable: defaults.TRANSFERABLE(),
            recipient: users.recipient1,
            sender: users.admin,
            startTime: getBlockTimestamp(),
            tranches: defaults.tranches(claimAmount),
            wasCanceled: false
        });

        assertTrue(testMerkleLT.hasClaimed(defaults.INDEX1()), "not claimed");
        assertEq(actualStreamId, expectedStreamId, "invalid stream id");
        assertEq(actualStream, expectedStream);
    }

    modifier whenCalculatedAmountsSumEqualsClaimAmount() {
        _;
    }

    function test_Claim()
        external
        givenCampaignNotExpired
        givenNotClaimed
        givenIncludedInMerkleTree
        whenCalculatedAmountsSumEqualsClaimAmount
    {
        uint256 expectedStreamId = lockupTranched.nextStreamId();
        vm.expectEmit({ emitter: address(merkleLT) });
        emit Claim(defaults.INDEX1(), users.recipient1, defaults.CLAIM_AMOUNT(), expectedStreamId);

        uint256 actualStreamId = claimLT();
        LockupTranched.StreamLT memory actualStream = lockupTranched.getStream(actualStreamId);
        LockupTranched.StreamLT memory expectedStream = LockupTranched.StreamLT({
            amounts: Lockup.Amounts({ deposited: defaults.CLAIM_AMOUNT(), refunded: 0, withdrawn: 0 }),
            asset: dai,
            endTime: getBlockTimestamp() + defaults.TOTAL_DURATION(),
            isCancelable: defaults.CANCELABLE(),
            isDepleted: false,
            isStream: true,
            isTransferable: defaults.TRANSFERABLE(),
            recipient: users.recipient1,
            sender: users.admin,
            startTime: getBlockTimestamp(),
            tranches: defaults.tranches(),
            wasCanceled: false
        });

        assertTrue(merkleLT.hasClaimed(defaults.INDEX1()), "not claimed");
        assertEq(actualStreamId, expectedStreamId, "invalid stream id");
        assertEq(actualStream, expectedStream);
    }
}
