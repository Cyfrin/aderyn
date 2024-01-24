# Adeyn - Nyth

[Aderyn](https://github.com/Cyfrin/aderyn) is a powerful Solidity static analyzer that takes a bird's eye view over your smart contracts.

**This is a `nyth` base repo.** `nyth` is the Aderyn bot development framework. It facilitates the creation of Solidity static analysis bots.

# Usage

Development flow:
1. [Generate a detector](#1-generate-a-detector)
2. Write the detector
3. Test the detector
4. Run your bot!

## 1. Generate a detector

From this root:

```
nyth generate bot/src/new_detector_name
```

This generates a new detector folder like this:

```

```

## Folder Structure

```
└── bot/
    ├── foundry_workspace/ <- Solidity workspace for testing
    │   ├── out/
    │   ├── src/
    │   └── ...
    ├── src/               <- Generated detectors go here
    │   └── ...
    ├── config_tests.rs    <- Test configuration
    └── runner.rs          <- `cargo run` config
```
