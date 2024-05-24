# Sablier V2 Periphery [![Github Actions][gha-badge]][gha] [![Coverage][codecov-badge]][codecov] [![Foundry][foundry-badge]][foundry] [![Discord][discord-badge]][discord]

[gha]: https://github.com/sablier-labs/v2-periphery/actions
[gha-badge]: https://github.com/sablier-labs/v2-periphery/actions/workflows/ci.yml/badge.svg
[codecov]: https://codecov.io/gh/sablier-labs/v2-periphery
[codecov-badge]: https://codecov.io/gh/sablier-labs/v2-periphery/branch/main/graph/badge.svg
[discord]: https://discord.gg/bSwRCwWRsT
[discord-badge]: https://dcbadge.vercel.app/api/server/bSwRCwWRsT?style=flat
[foundry]: https://getfoundry.sh/
[foundry-badge]: https://img.shields.io/badge/Built%20with-Foundry-FFDB1C.svg

This repository contains the peripheral smart contracts of the Sablier V2 Protocol. For lower-level logic, see the
[sablier-labs/v2-core](https://github.com/sablier-labs/v2-core) repository.

In-depth documentation is available at [docs.sablier.com](https://docs.sablier.com).

## Install

### Node.js

This is the recommended approach.

Install Sablier V2 Periphery using your favorite package manager, e.g., with Bun:

```shell
bun add @sablier/v2-periphery
```

Then, if you are using Foundry, add these to your `remappings.txt` file:

```text
@sablier/v2-core/=node_modules/@sablier/v2-core/
@sablier/v2-periphery/=node_modules/@sablier/v2-periphery/
@openzeppelin/contracts/=node_modules/@openzeppelin/contracts/
```

### Git Submodules

This installation method is not recommended, but it is available for those who prefer it.

First, install the submodule using Forge:

```sh
forge install sablier-labs/v2-periphery
```

Second, you need to install the project's dependencies:

```sh
forge install --no-commit sablier-labs/v2-core@release OpenZeppelin/openzeppelin-contracts@v4.9.2
```

Finally, add these to your `remappings.txt` file:

```text
@sablier/v2-core/=lib/v2-core/
@sablier/v2-periphery/=lib/v2-periphery/
@openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/
```

## Security

The codebase has undergone rigorous audits by leading security experts from Cantina, as well as independent auditors.
For a comprehensive list of all audits conducted, please click [here](https://github.com/sablier-labs/audits).

For any security-related concerns, please refer to the [SECURITY](./SECURITY.md) policy. This repository is subject to a
bug bounty program per the terms outlined in the aforementioned policy.

## Contributing

Feel free to dive in! [Open](https://github.com/sablier-labs/v2-periphery/issues/new) an issue,
[start](https://github.com/sablier-labs/v2-periphery/discussions/new) a discussion or submit a PR. For any informal
concerns or feedback, please join our [Discord server](https://discord.gg/bSwRCwWRsT).

For guidance on how to create PRs, see the [CONTRIBUTING](./CONTRIBUTING.md) guide.

## License

Sablier V2 Periphery is licensed under [GPL v3 or later](./LICENSE.md), except for most of the files in `test/`, which
remain unlicensed (as indicated in their SPDX headers).
