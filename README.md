<p align="center">
    <br />
    <a href="https://cyfrin.io/">
        <img src="https://github.com/Cyfrin/aderyn/blob/dev/.github/images/aderyn_logo.png" width="400" alt=""/></a>
    <br />
</p>
<p align="center"><strong>A powerful Solidity static analyzer that takes a bird's eye view over your smart contracts.
</strong></p>
<p align="center">
    <br />
    <a href="https://cyfrin.io/">
        <img src="https://github.com/Cyfrin/aderyn/blob/dev/.github/images/poweredbycyfrinblue.png" width="145" alt=""/></a>
    <br />
</p>

<p align="center">
<a href="https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli/readme">Docs</a>
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

**Aderyn is an open-source public good developer tool.** It is a Rust-based solidity smart contract static analyzer designed to help protocol engineers and security researchers find vulnerabilities in Solidity code bases.

You can read the [Cyfrin official documentation](https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli/readme) for an in-depth look at Aderyn's functionalities.

There is also an officially supported [VSCode extension](https://github.com/Cyfrin/vscode-aderyn/) for Aderyn. Download from the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=Cyfrin.aderyn&ssr=false#overview) and start identifying vulnerabilities in your Solidity 
code with ease.  

## Features

- Off the shelf support for Foundry projects.
- Off the shelf support for Hardhat projects. (Sometimes `remappings.txt` maybe required)
- Configuration file (`aderyn.toml`) needed to support custom frameworks.
- Markdown, JSON and Sarif reports

## Installation

> **NOTE** Windows users must have WSL installed

#### Cyfrinup - All in one cross platform installation manager for Cyfrin tools.

[One time setup](https://github.com/Cyfrin/up). - Run `cyfrinup`

Re-run `cyfrinup` to upgrade all Cyfrin tools to the latest version.

#### Curl
Once installed, run `aderyn-update` to upgrade.
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/cyfrin/aderyn/releases/latest/download/aderyn-installer.sh | bash
```


#### Homebrew
Once installed, run `brew upgrade cyfrin/tap/aderyn` to upgrade.
```sh
brew install cyfrin/tap/aderyn
```

#### npm
Once installed, re-run `npm install @cyfrin/aderyn -g` to upgrade.
```sh
npm install @cyfrin/aderyn -g
```

If you are installing with Curl or Homebrew or npm, ensure that the correct version of Aderyn in your path comes from either the Homebrew or npm global packages directory. If an older version exists at `~/.cyfrin/bin/aderyn`, remove it using `rm -f ~/.cyfrin/bin/aderyn`, as this is no longer the default installation location.

## Quick Start

Run `aderyn --version` to check if Aderyn is installed successfully.

[Quick Start](https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli/quickstart) example with video guide.

```
cd path/to/solidity/project/root
aderyn
```

This generates a [report.md](https://github.com/Cyfrin/aderyn/blob/dev/reports/report.md)

See examples using more CLI options [here](https://cyfrin.gitbook.io/cyfrin-docs/cli-options)

## VS Code extension

Officially supported [VSCode extension](https://github.com/Cyfrin/vscode-aderyn/) for Aderyn. 
Download from [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=Cyfrin.aderyn&ssr=false#overview)

## Github Action

Checkout [Aderyn CI Assistant](https://github.com/marketplace/actions/aderyn-ci-assistant) in the marketplace.
Performs Static analysis on Solidity codebases in CI to catch potential vulnerabilities before committing code. 

## Contributing & License

Help us build Aderyn 🦜 Please see our [contribution guidelines](./CONTRIBUTING.md) for in-depth developer environment setup and PR approval process.
Aderyn is an open-source software licensed under the [GPL-3.0 License](./LICENSE).

To learn how to create your custom Aderyn detectors, [checkout the official docs](https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli/detectors-quickstart)

Aderyn relies on a [custom backend](https://github.com/Cyfrin/solidity-ast-rs) to generate AST for Solidity codebases. 
It leverages [foundry-compilers](https://github.com/foundry-rs/compilers) 

## Credits

This project exists thanks to all the people who [contribute](/CONTRIBUTING.md).<br>

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
[license-url]: https://github.com/cyfrin/aderyn/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
