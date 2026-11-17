## Retirement

> **Aderyn is retired and no longer maintained.** There are no supported
> installation or upgrade paths. The usage and integration documentation below
> is retained for historical reference. Existing installations receive no updates
> or support.

<p align="center">
    <br />
    <a href="https://cyfrin.io/">
        <img src="https://github.com/Cyfrin/aderyn/blob/dev/.github/images/aderyn_logo.png" width="400" alt=""/></a>
    <br />
</p>
<p align="center"><strong>Retired Solidity static analyzer. Source and documentation retained for reference.
</strong></p>
<p align="center">
    <br />
    <a href="https://cyfrin.io/">
        <img src="https://github.com/Cyfrin/aderyn/blob/dev/.github/images/poweredbycyfrinblue.png" width="145" alt=""/></a>
    <br />
</p>

<p align="center">
<a href="https://aderyn.cyfrin.io/overview">Reference docs</a>
<a href="https://discord.gg/cyfrin">Discord</a>
<a href="https://x.com/cyfrin">X/Twitter</a>
<p>

---

<div align="center">

[![Stargazers][stars-shield]][stars-url] [![Forks][forks-shield]][forks-url]
[![Contributors][contributors-shield]][contributors-url]
[![Release][release-shield]][release-url]
[![Issues][issues-shield]][issues-url]
[![GPL-3.0 License][license-shield]][license-url]

</div>

## What is Aderyn?

**Aderyn was an open-source public good developer tool.** It is a retired Rust-based Solidity smart contract static analyzer designed to help protocol engineers and security researchers find vulnerabilities in Solidity code bases.

The [reference documentation](https://aderyn.cyfrin.io/overview) remains available for an in-depth look at Aderyn's historical functionality.

The [VS Code extension](https://github.com/Cyfrin/vscode-aderyn/) is also retired
and has been delisted from the Visual Studio Marketplace.

## Historical features

- Runs without need for any configuration support for Foundry and Hardhat projects.
- Creates Markdown, JSON and Sarif reports.
- Previously powered the [VS Code extension](https://github.com/Cyfrin/vscode-aderyn/).
- Historical [detector reference](https://aderyn.cyfrin.io/configuration/supported-detectors)
- Extensible detector framework
- Downloaded more than 45K times. 

## Installation and removal

Aderyn installation and upgrades are no longer supported through Homebrew, npm,
Cyfrinup, or the shell installer. Official CLI binaries, updater binaries, package
archives, and installation manifests have been withdrawn from GitHub releases.
Retained shell installer URLs print a retirement notice and exit with an error.
Source archives and checksum records remain available for historical reference.

Existing installations are not automatically removed. Some package-registry
archives, cached copies, and existing lockfiles remain usable; their availability
does not imply maintenance or support.

#### Homebrew

The Aderyn Homebrew package is retired. Remove an existing installation with:

```sh
brew uninstall cyfrin/tap/aderyn
```

#### npm

The Aderyn npm package is retired. Remove an existing global installation with:

```sh
npm uninstall --global @cyfrin/aderyn
```

## Quick Start (historical)

For an existing historical installation, `aderyn --version` identifies the installed version.

[Historical quickstart](https://aderyn.cyfrin.io/quickstart) example with video guide.

```
cd path/to/solidity/project/root
aderyn
```

This generates a [report.md](https://github.com/Cyfrin/aderyn/blob/dev/reports/report.md)

See the historical [CLI options reference](https://aderyn.cyfrin.io/cli-options).

## VS Code extension

The [VS Code extension](https://github.com/Cyfrin/vscode-aderyn/) is retired and
has been delisted from the Visual Studio Marketplace. Existing users can uninstall
Aderyn from VS Code's Extensions view.

## Github Action

The retired [Aderyn CI Assistant](https://github.com/Cyfrin/aderyn-ci)
performed static analysis on Solidity codebases in CI. Its Marketplace listing has
been removed, and the integration is no longer maintained. Remove or replace
workflow steps referencing `Cyfrin/aderyn-ci`, including tag and commit-SHA pins;
delisting does not disable those references.

## Contributing & License

Historical developer setup and contribution information is retained in the
[contribution guidelines](./CONTRIBUTING.md).
Aderyn is an open-source software licensed under the [GPL-3.0 License](./LICENSE).

Historical detector-development instructions remain in the [custom detector reference](https://aderyn.cyfrin.io/internals/custom-detectors-guide).

Aderyn relies on a [custom backend](https://github.com/Cyfrin/solidity-ast-rs) to generate AST for Solidity codebases. 
It leverages [foundry-compilers](https://github.com/foundry-rs/compilers) 

## Credits

Thank you to everyone who contributed to Aderyn.<br>

<a href="https://github.com/cyfrin/Aderyn/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=cyfrin/Aderyn" />
</a>

## Attribution

- Initial inspiration for AST Visitor code from [solc-ast-rs](https://github.com/hrkrshnn/solc-ast-rs).
- Original detectors based on [4naly3er](https://github.com/Picodes/4naly3er) detectors.
- Shoutout to the original king of static analysis [slither](https://github.com/crytic/slither).


[contributors-shield]: https://img.shields.io/github/contributors/cyfrin/aderyn
[contributors-url]: https://github.com/cyfrin/aderyn/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/cyfrin/aderyn
[forks-url]: https://github.com/cyfrin/aderyn/network/members
[stars-shield]: https://img.shields.io/github/stars/cyfrin/aderyn
[stars-url]: https://github.com/cyfrin/aderyn/stargazers
[release-shield]: https://img.shields.io/github/v/release/Cyfrin/aderyn
[release-url]: https://github.com/Cyfrin/aderyn/releases
[issues-shield]: https://img.shields.io/github/issues/cyfrin/aderyn
[issues-url]: https://github.com/cyfrin/aderyn/issues
[license-shield]: https://img.shields.io/github/license/cyfrin/aderyn?logoColor=%23fff&color=blue
[license-url]: https://github.com/cyfrin/aderyn/blob/dev/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
