// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity >=0.8.22 <0.9.0;

import { Arrays } from "@openzeppelin/contracts/utils/Arrays.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { ud2x18 } from "@prb/math/src/UD2x18.sol";
import { UD60x18, ud } from "@prb/math/src/UD60x18.sol";
import { Broker, LockupDynamic, LockupLinear, LockupTranched } from "@sablier/v2-core/src/types/DataTypes.sol";

import { BatchLockup, MerkleLockup, MerkleLT } from "src/types/DataTypes.sol";

import { ArrayBuilder } from "./ArrayBuilder.sol";
import { BatchLockupBuilder } from "./BatchLockupBuilder.sol";
import { Merkle } from "./Murky.sol";
import { MerkleBuilder } from "./MerkleBuilder.sol";
import { Users } from "./Types.sol";

/// @notice Contract with default values for testing.
contract Defaults is Merkle {
    using MerkleBuilder for uint256[];

    /*//////////////////////////////////////////////////////////////////////////
                                      GENERICS
    //////////////////////////////////////////////////////////////////////////*/

    uint64 public constant BATCH_SIZE = 10;
    UD60x18 public constant BROKER_FEE = UD60x18.wrap(0);
    uint40 public constant CLIFF_DURATION = 2500 seconds;
    uint40 public immutable CLIFF_TIME;
    uint40 public immutable END_TIME;
    uint256 public constant ETHER_AMOUNT = 10_000 ether;
    uint128 public constant PER_STREAM_AMOUNT = 10_000e18;
    uint128 public constant REFUND_AMOUNT = 7500e18; // deposit - cliff amount
    uint40 public immutable START_TIME;
    uint40 public constant TOTAL_DURATION = 10_000 seconds;
    uint128 public constant TOTAL_TRANSFER_AMOUNT = PER_STREAM_AMOUNT * uint128(BATCH_SIZE);
    uint128 public constant WITHDRAW_AMOUNT = 2500e18;

    /*//////////////////////////////////////////////////////////////////////////
                                  MERKLE-LOCKUP
    //////////////////////////////////////////////////////////////////////////*/

    uint256 public constant AGGREGATE_AMOUNT = CLAIM_AMOUNT * RECIPIENT_COUNT;
    bool public constant CANCELABLE = false;
    uint128 public constant CLAIM_AMOUNT = 10_000e18;
    uint40 public immutable EXPIRATION;
    uint40 public immutable FIRST_CLAIM_TIME;
    uint256 public constant INDEX1 = 1;
    uint256 public constant INDEX2 = 2;
    uint256 public constant INDEX3 = 3;
    uint256 public constant INDEX4 = 4;
    string public constant IPFS_CID = "QmbWqxBEKC3P8tqsKc98xmWNzrzDtRLMiMPL8wBuTGsMnR";
    uint256[] public LEAVES = new uint256[](RECIPIENT_COUNT);
    uint256 public constant RECIPIENT_COUNT = 4;
    bytes32 public immutable MERKLE_ROOT;
    string public constant NAME = "Airdrop Campaign";
    bytes32 public constant NAME_BYTES32 = bytes32(abi.encodePacked("Airdrop Campaign"));
    bool public constant TRANSFERABLE = false;

    /*//////////////////////////////////////////////////////////////////////////
                                     VARIABLES
    //////////////////////////////////////////////////////////////////////////*/

    IERC20 private asset;
    Users private users;

    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTRUCTOR
    //////////////////////////////////////////////////////////////////////////*/

    constructor(Users memory users_, IERC20 asset_) {
        users = users_;
        asset = asset_;

        // Initialize the immutables.
        START_TIME = uint40(block.timestamp) + 100 seconds;
        CLIFF_TIME = START_TIME + CLIFF_DURATION;
        END_TIME = START_TIME + TOTAL_DURATION;
        EXPIRATION = uint40(block.timestamp) + 12 weeks;
        FIRST_CLAIM_TIME = uint40(block.timestamp);

        // Initialize the Merkle tree.
        LEAVES[0] = MerkleBuilder.computeLeaf(INDEX1, users.recipient1, CLAIM_AMOUNT);
        LEAVES[1] = MerkleBuilder.computeLeaf(INDEX2, users.recipient2, CLAIM_AMOUNT);
        LEAVES[2] = MerkleBuilder.computeLeaf(INDEX3, users.recipient3, CLAIM_AMOUNT);
        LEAVES[3] = MerkleBuilder.computeLeaf(INDEX4, users.recipient4, CLAIM_AMOUNT);
        MerkleBuilder.sortLeaves(LEAVES);
        MERKLE_ROOT = getRoot(LEAVES.toBytes32());
    }

    function getLeaves() public view returns (uint256[] memory) {
        return LEAVES;
    }

    /*//////////////////////////////////////////////////////////////////////////
                                  MERKLE-LOCKUP
    //////////////////////////////////////////////////////////////////////////*/

    function index1Proof() public view returns (bytes32[] memory) {
        uint256 leaf = MerkleBuilder.computeLeaf(INDEX1, users.recipient1, CLAIM_AMOUNT);
        uint256 pos = Arrays.findUpperBound(LEAVES, leaf);
        return getProof(LEAVES.toBytes32(), pos);
    }

    function index2Proof() public view returns (bytes32[] memory) {
        uint256 leaf = MerkleBuilder.computeLeaf(INDEX2, users.recipient2, CLAIM_AMOUNT);
        uint256 pos = Arrays.findUpperBound(LEAVES, leaf);
        return getProof(LEAVES.toBytes32(), pos);
    }

    function index3Proof() public view returns (bytes32[] memory) {
        uint256 leaf = MerkleBuilder.computeLeaf(INDEX3, users.recipient3, CLAIM_AMOUNT);
        uint256 pos = Arrays.findUpperBound(LEAVES, leaf);
        return getProof(LEAVES.toBytes32(), pos);
    }

    function index4Proof() public view returns (bytes32[] memory) {
        uint256 leaf = MerkleBuilder.computeLeaf(INDEX4, users.recipient4, CLAIM_AMOUNT);
        uint256 pos = Arrays.findUpperBound(LEAVES, leaf);
        return getProof(LEAVES.toBytes32(), pos);
    }

    function baseParams() public view returns (MerkleLockup.ConstructorParams memory) {
        return baseParams(users.admin, asset, EXPIRATION, MERKLE_ROOT);
    }

    function baseParams(
        address admin,
        IERC20 asset_,
        uint40 expiration,
        bytes32 merkleRoot
    )
        public
        pure
        returns (MerkleLockup.ConstructorParams memory)
    {
        return MerkleLockup.ConstructorParams({
            asset: asset_,
            cancelable: CANCELABLE,
            expiration: expiration,
            initialAdmin: admin,
            ipfsCID: IPFS_CID,
            merkleRoot: merkleRoot,
            name: NAME,
            transferable: TRANSFERABLE
        });
    }

    function tranchesWithPercentages()
        public
        pure
        returns (MerkleLT.TrancheWithPercentage[] memory tranchesWithPercentages_)
    {
        tranchesWithPercentages_ = new MerkleLT.TrancheWithPercentage[](2);
        tranchesWithPercentages_[0] =
            MerkleLT.TrancheWithPercentage({ unlockPercentage: ud2x18(0.25e18), duration: 2500 seconds });
        tranchesWithPercentages_[1] =
            MerkleLT.TrancheWithPercentage({ unlockPercentage: ud2x18(0.75e18), duration: 7500 seconds });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                 SABLIER-V2-LOCKUP
    //////////////////////////////////////////////////////////////////////////*/

    function assets() public view returns (IERC20[] memory assets_) {
        assets_ = new IERC20[](1);
        assets_[0] = asset;
    }

    function broker() public view returns (Broker memory) {
        return Broker({ account: users.broker, fee: BROKER_FEE });
    }

    function incrementalStreamIds() public pure returns (uint256[] memory streamIds) {
        return ArrayBuilder.fillStreamIds({ firstStreamId: 1, batchSize: BATCH_SIZE });
    }

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-LOCKUP-DYNAMIC
    //////////////////////////////////////////////////////////////////////////*/

    function createWithDurationsLD() public view returns (LockupDynamic.CreateWithDurations memory) {
        return createWithDurationsLD(asset);
    }

    function createWithDurationsLD(IERC20 asset_) public view returns (LockupDynamic.CreateWithDurations memory) {
        return LockupDynamic.CreateWithDurations({
            sender: users.alice,
            recipient: users.recipient0,
            totalAmount: PER_STREAM_AMOUNT,
            asset: asset_,
            cancelable: true,
            transferable: true,
            segments: segmentsWithDurations(),
            broker: broker()
        });
    }

    function createWithTimestampsLD() public view returns (LockupDynamic.CreateWithTimestamps memory) {
        return createWithTimestampsLD(asset);
    }

    function createWithTimestampsLD(IERC20 asset_) public view returns (LockupDynamic.CreateWithTimestamps memory) {
        return LockupDynamic.CreateWithTimestamps({
            sender: users.alice,
            recipient: users.recipient0,
            totalAmount: PER_STREAM_AMOUNT,
            asset: asset_,
            cancelable: true,
            transferable: true,
            startTime: START_TIME,
            segments: segments(),
            broker: broker()
        });
    }

    /// @dev Returns a batch of {LockupDynamic.Segment} parameters.
    function segments() private view returns (LockupDynamic.Segment[] memory segments_) {
        segments_ = new LockupDynamic.Segment[](2);
        segments_[0] = LockupDynamic.Segment({
            amount: 2500e18,
            exponent: ud2x18(3.14e18),
            timestamp: START_TIME + CLIFF_DURATION
        });
        segments_[1] = LockupDynamic.Segment({
            amount: 7500e18,
            exponent: ud2x18(3.14e18),
            timestamp: START_TIME + TOTAL_DURATION
        });
    }

    /// @dev Returns a batch of {LockupDynamic.SegmentWithDuration} parameters.
    function segmentsWithDurations() public pure returns (LockupDynamic.SegmentWithDuration[] memory) {
        return segmentsWithDurations({ amount0: 2500e18, amount1: 7500e18 });
    }

    /// @dev Returns a batch of {LockupDynamic.SegmentWithDuration} parameters.
    function segmentsWithDurations(
        uint128 amount0,
        uint128 amount1
    )
        public
        pure
        returns (LockupDynamic.SegmentWithDuration[] memory segments_)
    {
        segments_ = new LockupDynamic.SegmentWithDuration[](2);
        segments_[0] =
            LockupDynamic.SegmentWithDuration({ amount: amount0, exponent: ud2x18(3.14e18), duration: 2500 seconds });
        segments_[1] =
            LockupDynamic.SegmentWithDuration({ amount: amount1, exponent: ud2x18(3.14e18), duration: 7500 seconds });
    }

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-LOCKUP-LINEAR
    //////////////////////////////////////////////////////////////////////////*/

    function createWithDurationsLL() public view returns (LockupLinear.CreateWithDurations memory) {
        return createWithDurationsLL(asset);
    }

    function createWithDurationsLL(IERC20 asset_) public view returns (LockupLinear.CreateWithDurations memory) {
        return LockupLinear.CreateWithDurations({
            sender: users.alice,
            recipient: users.recipient0,
            totalAmount: PER_STREAM_AMOUNT,
            asset: asset_,
            cancelable: true,
            transferable: true,
            durations: durations(),
            broker: broker()
        });
    }

    function createWithTimestampsLL() public view returns (LockupLinear.CreateWithTimestamps memory) {
        return createWithTimestampsLL(asset);
    }

    function createWithTimestampsLL(IERC20 asset_) public view returns (LockupLinear.CreateWithTimestamps memory) {
        return LockupLinear.CreateWithTimestamps({
            sender: users.alice,
            recipient: users.recipient0,
            totalAmount: PER_STREAM_AMOUNT,
            asset: asset_,
            cancelable: true,
            transferable: true,
            timestamps: linearTimestamps(),
            broker: broker()
        });
    }

    function durations() public pure returns (LockupLinear.Durations memory) {
        return LockupLinear.Durations({ cliff: CLIFF_DURATION, total: TOTAL_DURATION });
    }

    function linearTimestamps() private view returns (LockupLinear.Timestamps memory) {
        return LockupLinear.Timestamps({ start: START_TIME, cliff: CLIFF_TIME, end: END_TIME });
    }

    /*//////////////////////////////////////////////////////////////////////////
                             SABLIER-V2-LOCKUP-TRANCHED
    //////////////////////////////////////////////////////////////////////////*/

    function createWithDurationsLT() public view returns (LockupTranched.CreateWithDurations memory) {
        return createWithDurationsLT(asset);
    }

    function createWithDurationsLT(IERC20 asset_) public view returns (LockupTranched.CreateWithDurations memory) {
        return LockupTranched.CreateWithDurations({
            sender: users.alice,
            recipient: users.recipient0,
            totalAmount: PER_STREAM_AMOUNT,
            asset: asset_,
            cancelable: true,
            transferable: true,
            tranches: tranchesWithDurations(),
            broker: broker()
        });
    }

    function createWithTimestampsLT() public view returns (LockupTranched.CreateWithTimestamps memory) {
        return createWithTimestampsLT(asset);
    }

    function createWithTimestampsLT(IERC20 asset_) public view returns (LockupTranched.CreateWithTimestamps memory) {
        return LockupTranched.CreateWithTimestamps({
            sender: users.alice,
            recipient: users.recipient0,
            totalAmount: PER_STREAM_AMOUNT,
            asset: asset_,
            cancelable: true,
            transferable: true,
            startTime: START_TIME,
            tranches: tranches(),
            broker: broker()
        });
    }

    function tranches() public view returns (LockupTranched.Tranche[] memory tranches_) {
        tranches_ = new LockupTranched.Tranche[](2);
        tranches_[0] = LockupTranched.Tranche({ amount: 2500e18, timestamp: uint40(block.timestamp) + CLIFF_DURATION });
        tranches_[1] = LockupTranched.Tranche({ amount: 7500e18, timestamp: uint40(block.timestamp) + TOTAL_DURATION });
    }

    /// @dev Mirros the logic from {SablierV2MerkleLT._calculateTranches}.
    function tranches(uint128 totalAmount) public view returns (LockupTranched.Tranche[] memory tranches_) {
        tranches_ = tranches();

        uint128 amount0 = ud(totalAmount).mul(tranchesWithPercentages()[0].unlockPercentage.intoUD60x18()).intoUint128();
        uint128 amount1 = ud(totalAmount).mul(tranchesWithPercentages()[1].unlockPercentage.intoUD60x18()).intoUint128();

        tranches_[0].amount = amount0;
        tranches_[1].amount = amount1;

        uint128 amountsSum = amount0 + amount1;

        if (amountsSum != totalAmount) {
            tranches_[1].amount += totalAmount - amountsSum;
        }
    }

    /// @dev Returns a batch of {LockupTranched.TrancheWithDuration} parameters.
    function tranchesWithDurations() public pure returns (LockupTranched.TrancheWithDuration[] memory) {
        return tranchesWithDurations({ amount0: 2500e18, amount1: 7500e18 });
    }

    /// @dev Returns a batch of {LockupTranched.TrancheWithDuration} parameters.
    function tranchesWithDurations(
        uint128 amount0,
        uint128 amount1
    )
        public
        pure
        returns (LockupTranched.TrancheWithDuration[] memory segments_)
    {
        segments_ = new LockupTranched.TrancheWithDuration[](2);
        segments_[0] = LockupTranched.TrancheWithDuration({ amount: amount0, duration: 2500 seconds });
        segments_[1] = LockupTranched.TrancheWithDuration({ amount: amount1, duration: 7500 seconds });
    }

    /*//////////////////////////////////////////////////////////////////////////
                                    BATCH-LOCKUP
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Returns a default-size batch of {BatchLockup.CreateWithDurationsLD} parameters.
    function batchCreateWithDurationsLD() public view returns (BatchLockup.CreateWithDurationsLD[] memory batch) {
        batch = BatchLockupBuilder.fillBatch(createWithDurationsLD(), BATCH_SIZE);
    }

    /// @dev Returns a default-size batch of {BatchLockup.CreateWithDurationsLL} parameters.
    function batchCreateWithDurationsLL() public view returns (BatchLockup.CreateWithDurationsLL[] memory batch) {
        batch = BatchLockupBuilder.fillBatch(createWithDurationsLL(), BATCH_SIZE);
    }

    /// @dev Returns a default-size batch of {BatchLockup.CreateWithDurationsLT} parameters.
    function batchCreateWithDurationsLT() public view returns (BatchLockup.CreateWithDurationsLT[] memory batch) {
        batch = BatchLockupBuilder.fillBatch(createWithDurationsLT(), BATCH_SIZE);
    }

    /// @dev Returns a default-size batch of {BatchLockup.CreateWithTimestampsLD} parameters.
    function batchCreateWithTimestampsLD() public view returns (BatchLockup.CreateWithTimestampsLD[] memory batch) {
        batch = batchCreateWithTimestampsLD(BATCH_SIZE);
    }

    /// @dev Returns a batch of {BatchLockup.CreateWithTimestampsLD} parameters.
    function batchCreateWithTimestampsLD(uint256 batchSize)
        public
        view
        returns (BatchLockup.CreateWithTimestampsLD[] memory batch)
    {
        batch = BatchLockupBuilder.fillBatch(createWithTimestampsLD(), batchSize);
    }

    /// @dev Returns a default-size batch of {BatchLockup.CreateWithTimestampsLL} parameters.
    function batchCreateWithTimestampsLL() public view returns (BatchLockup.CreateWithTimestampsLL[] memory batch) {
        batch = batchCreateWithTimestampsLL(BATCH_SIZE);
    }

    /// @dev Returns a batch of {BatchLockup.CreateWithTimestampsLL} parameters.
    function batchCreateWithTimestampsLL(uint256 batchSize)
        public
        view
        returns (BatchLockup.CreateWithTimestampsLL[] memory batch)
    {
        batch = BatchLockupBuilder.fillBatch(createWithTimestampsLL(), batchSize);
    }

    /// @dev Returns a default-size batch of {BatchLockup.CreateWithTimestampsLT} parameters.
    function batchCreateWithTimestampsLT() public view returns (BatchLockup.CreateWithTimestampsLT[] memory batch) {
        batch = batchCreateWithTimestampsLT(BATCH_SIZE);
    }

    /// @dev Returns a batch of {BatchLockup.CreateWithTimestampsLL} parameters.
    function batchCreateWithTimestampsLT(uint256 batchSize)
        public
        view
        returns (BatchLockup.CreateWithTimestampsLT[] memory batch)
    {
        batch = BatchLockupBuilder.fillBatch(createWithTimestampsLT(), batchSize);
    }
}
