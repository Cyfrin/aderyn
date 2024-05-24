# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/).

[1.1.2]: https://github.com/sablier-labs/v2-core/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/sablier-labs/v2-core/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/sablier-labs/v2-core/compare/v1.0.2...v1.1.0
[1.0.2]: https://github.com/sablier-labs/v2-core/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/sablier-labs/v2-core/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/sablier-labs/v2-core/releases/tag/v1.0.0

## [1.1.2] - 2023-12-19

### Changed

- Use Solidity v0.8.23 ([#758](https://github.com/sablier-labs/v2-core/pull/758))

## [1.1.1] - 2023-12-16

### Changed

- Bump package version for NPM release
  ([88db61](https://github.com/sablier-labs/v2-core/tree/88db61bcf193ef9494b31c883ed2c9ad997a1271))

## [1.1.0] - 2023-12-15

### Changed

- **Breaking**: Remove ability to cancel for recipients ([#710](https://github.com/sablier-labs/v2-core/pull/710))
- Move `isWarm` and `isCold` to `SablierV2Lockup` ([#664](https://github.com/sablier-labs/v2-core/pull/664))
- Replace the streamed amount with the deposit amount in the NFT descriptor
  ([#692](https://github.com/sablier-labs/v2-core/pull/692))
- Simplify `renounce` and `withdraw` implementations ([#683](https://github.com/sablier-labs/v2-core/pull/683),
  [#705](https://github.com/sablier-labs/v2-core/pull/705))
- Update import paths to use Node.js dependencies ([#734](https://github.com/sablier-labs/v2-core/pull/734))
- Use Solidity v0.8.21 ([#688](https://github.com/sablier-labs/v2-core/pull/688))

### Added

- Add `ERC-4906` metadata update in `transferFrom` ([#686](https://github.com/sablier-labs/v2-core/pull/686))
- Add `transferable` boolean flag ([#668](https://github.com/sablier-labs/v2-core/pull/668))

### Removed

- Remove `@openzeppelin/contracts` from Node.js peer dependencies
  ([#694](https://github.com/sablier-labs/v2-core/pull/694))

## [1.0.2] - 2023-08-14

### Changed

- Update `@prb/math` import paths to contain `src/` ([#648](https://github.com/sablier-labs/v2-core/pull/648))

## [1.0.1] - 2023-07-13

### Changed

- Optimize use of variables in `tokenURI` ([#617](https://github.com/sablier-labs/v2-core/pull/617))

### Fixed

- Fix data URI scheme in `tokenURI` ([#617](https://github.com/sablier-labs/v2-core/pull/617))

## [1.0.0] - 2023-07-07

### Added

- Initial release
