<p align="center">
<img src="https://files.sablier.com/logo.svg" width="500" alt="project-name">
</p>

# Contest Details

### Prize Pool

- Total Pool - $53,440
- H/M - $45,000
- Low - $3,100
- Community Judging Pool - $5,340

- Starts: May 10, 2024 Noon UTC
- Ends: May 31, 2024 Noon UTC

### Stats

- Total nSLOC: 2655
- $/nSloc: $20

## About

Sablier is a permissionless token distribution protocol for ERC-20 assets. It can be used for vesting, payroll, airdrops, and more.

The sender of a payment stream first deposits a specific amount of ERC-20 tokens in a contract. Then, the contract progressively allocates the funds to the stream recipient, also known as the Sablier NFT owner, who can access them as they become available over time. The payment rate is influenced by various factors such as the start time, the end time, the total amount of tokens deposited and the type of stream.

There are two repositories:

### **v2-core**

Core contracts facilitate the creation of three types of streams:

1. **Lockup Linear**: This stream linearly unlocks assets over time.

2. **Lockup Tranched**: Assets in this stream are unlocked in scheduled tranches such as monthly. This stream will be launching in v2.2.

3. **Lockup Dynamic**: Our most versatile stream, allowing users to create various unlocking curves including exponential, unlock cliff, and timelock.

For additional details on how these streams work, please refer to our [documentation](https://docs.sablier.com/concepts/protocol/streaming). Note that the documentation is for v2.1, while the contracts in this contest apply to v2.2.

### **v2-periphery**

Periphery contracts interact with core contracts and consist of two primary contracts:

1. **Batch contract** enabled the creation of multiple streams within a single transaction.
2. **Merkle Lockup contracts** support the distribution of vesting airdrops. A Merkle lockup contract is created using a factory contract and contains a Merkle tree root with details on airdrop recipients and amounts. Once deployed, recipients can claim their airdrops via the Merkle lockup contract. These are then distributed as Sablier streams, called as [Airstreams](https://docs.sablier.com/concepts/protocol/airstreams). Moreover, there is a grace period during which the airdrop creator can clawback unclaimed funds.

### Useful links:

[Previous audits](https://github.com/sablier-labs/audits)

[Sablier's Website](https://sablier.com/)

[Documentation](https://docs.sablier.com/)

[GitHub](https://github.com/sablier-labs)

[Warpcast](https://warpcast.com/sablier)

[X](https://twitter.com/Sablier)

## Actors

There are three roles assumed by actors in the Sablier protocol:

### Recipient

Users who are the recipients of the streams. These users own the Sablier NFT which grants them the right to withdraw assets from the stream.

### Sender

Users who create streams and are responsible for funding them. Senders are also authorized to cancel and renounce streams. These users can also trigger withdrawals on behalf of the recipients but only to the recipient's address.

### Unknown caller

These are callers who are neither Sender nor Recipient but are allowed to trigger withdrawals on behalf of the recipients. This is because the withdraw function is publicly callable. Note that an unknown caller can withdraw assets only to the recipient's address.

## Scope (contracts)

### v2-core

```tree
src
├── abstracts
│   ├── Adminable.sol - A minimalist implementation to handle admin access
│   ├── NoDelegateCall.sol - A minimalist implementation to prevent delegate calls
│   └── SablierV2Lockup.sol - Handles common logic between all Sablier V2 Lockup contracts
├── libraries
│   ├── Errors.sol - Library containing all custom errors used in the Sablier protocol
│   ├── Helpers.sol - Helpers to calculate and validate input data required to create streams
│   ├── NFTSVG.sol - Library to generate NFT SVG
│   └── SVGElements.sol - Library to generate specific components of NFT SVG
├── types
│   └── DataTypes.sol - Implementation for a set of custom data types used in V2 core
├── SablierV2LockupDynamic.sol - Creates and manages Lockup streams with a dynamic distribution function
├── SablierV2LockupLinear.sol - Creates and manages Lockup streams with a linear distribution function
├── SablierV2LockupTranched.sol - Creates and manages Lockup streams with a tranched distribution function
└── SablierV2NFTDescriptor.sol - Generates the URI describing the Sablier V2 stream NFTs
```

### v2-periphery

```tree
src
├── abstracts
│   └── SablierV2MerkleLockup.sol - Handles common logic between all Airstream campaigns
├── libraries
│   └── Errors.sol - Library containing all custom errors used in the Sablier protocol
├── types
│   └── DataTypes.sol - Implementation for a set of custom data types used in V2 periphery
├── SablierV2BatchLockup.sol - Helpers to batch create Sablier V2 Lockup streams
├── SablierV2MerkleLL.sol - Allows users to claim Airdrops using Merkle proofs. These airdrops are powered by Lockup Linear streams
├── SablierV2MerkleLT.sol - Allows users to claim Airdrops using Merkle proofs. These airdrops are powered by Lockup Tranched streams
└── SablierV2MerkleLockupFactory.sol - Factory for deploying Airdrop campaigns using CREATE
```

## Compatibilities

Sablier protocol is compatible with the following:

1. Any network which is EVM compatible
2. Any ERC20 token

Its not compatible with:

1. Any network which is not EVM compatible
2. Any token standard other than ERC20
3. Rebased ERC20 tokens can be used but yield will be lost
4. Ether (ETH)

## Setup

Clone the contest repository:

```bash
git clone https://github.com/Cyfrin/2024-05-Sablier.git
cd 2024-05-Sablier
code .
```

Then either go to `v2-core` or `v2-periphery`.

```bash
$ cd v2-core
```

Once you are inside the project directory, run the following commands to install Node.js dependencies and build the contracts:

```bash
$ bun install --frozen-lockfile
$ bun run build
```

To see a list of available scripts:

```bash
$ bun run
```

To run tests:

```bash
$ bun run test --no-match-test testFork
```

To run fork tests against the Ethereum mainnet, follow the below instructions:

1. copy the `.env.example` file to create a `.env` file
2. Set `RPC_URL_MAINNET` to the URL of your mainnet RPC endpoint.

Then use the below command to run fork tests:

```bash
$ bun run test --match-test testFork
```

## Assumptions

The protocol makes the following assumptions:

1. The total supply of any ERC-20 token remains below `type(uint128).max`.

2. The `transfer` and `transferFrom` methods of any ERC-20 token strictly reduce the sender's balance by the transfer amount and increase the recipient's balance by the same amount. In other words, tokens that charge fees on transfers are not supported.

3. An address's ERC-20 balance can only change as a result of a `transfer` call by the sender or a `transferFrom` call by an approved address. This excludes rebase tokens and interest-bearing tokens.

4. The token contract does not allow callbacks (e.g. ERC-777 is not supported).

### v2-core

1. The immutable variables `MAX_SEGMENT_COUNT` and `MAX_TRANCHE_COUNT` have values that cannot lead to an overflow of the block gas limit.

### v2-periphery

1. `MerkleLockup` uses a Merkle tree to distribute airstream funds. The leaves of the Merkle tree are not checked to be valid at the contract level. This Merkle tree is generated by the Sablier interface which we expect users to trust. If a user submits Merkle Root directly with an IPFS hash (which we also allow through the UI), our backend performs a validation check to ensure that the Merkle tree is correctly generated.

2. For `MerkleLockup,`, a **grace period** is defined as the initial period during which `clawback` can be used. It ends 7 days after the first airstream claim has been made. Thus, airstream creators are assumed to be trusted during the grace period.

3. In the case of `MerkleLockup`, if an Airstream campaign is created by a team, all members in that team are assumed to have trust among them i.e. they would not rug each other.

4. In `SablierV2MerkleLT`, the unlock percentages and durations for tranches are uniform across all airdrop claimers.

## Known Issues

### v2-core

1. In `SablierV2Lockup`, when `onLockupStreamCanceled()`, `onLockupStreamWithdrawn()` and `onLockupStreamRenounced()` hooks are called, there could be gas bomb attacks because we do not limit the gas transferred to the external contract. This is intended to support complex transactions implemented in those hooks.

### v2-periphery

1. In `SablierV2MerkleLockupFactory`, users can submit root hashes with duplicate indexes or create Merkle trees with incorrect depths, potentially leading to unclaimable assets or unexpected behavior. These issues are exacerbated by the fact that the Merkle tree's validity is not checked at the contract level. As per assumption (7), it is expected that the Merkle tree will be correctly generated. In case of a malicious Merkle tree, `clawback` can be called to withdraw funds from the deployed `MerkleLockup` contracts until the grace period ends.

2. In `SablierV2MerkleLockupFactory`, malicious `lockupLinear` and `lockupTranched` contract addresses can be passed during `createMerkleLL()` and `createMerkleLT()` functions respectively.

3. In `SablierV2MerkleLockupFactory`, `aggregateAmount` and `recipientCount` values are exclusively emitted in the `CreateMerkleLT` and `CreateMerkleLL` events. These values are not validated within the create functions as they are not utilized elsewhere in the contract. However, it's important to note that this impacts integrators who listen to these events and rely on these values. We advise caution, as they may be inaccurate.

4. If the `admin` of the deployed `SablierV2MerkleLT` and `SablierV2MerkleLL` contracts is modified, the `onLockupStreamWithdrawn()` hook callback, if it is implemented, will continue to be made to the original `admin` for the airstreams that were already claimed at the time of the change.

**[Additional known issues as detected by LightChaser](https://github.com/Cyfrin/2024-05-Sablier/issues/1)**
