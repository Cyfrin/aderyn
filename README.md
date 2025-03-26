<p align="center">
    <br />
    <a href="https://cyfrin.io/">
        <img src=".github/images/aderyn_logo.png" width="400" alt=""/></a>
    <br />
</p>
<p align="center"><strong>A powerful Solidity static analyzer that takes a bird's eye view over your smart contracts.
</strong></p>
<p align="center">
    <br />
    <a href="https://cyfrin.io/">
        <img src=".github/images/poweredbycyfrinblue.png" width="145" alt=""/></a>
    <br />
</p>

<p align="center">
<a href="https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli/readme">Docs</a>
<a href="https://discord.gg/cyfrin">Get support</a>
<a href="https://cyfrin.io">Website</a>
<a href="https://twitter.com/cyfrinaudits">Twitter</a>
<p>

---

<div align="center">

[![Stargazers][stars-shield]][stars-url] [![Forks][forks-shield]][forks-url]
[![Contributors][contributors-shield]][contributors-url]
[![Issues][issues-shield]][issues-url]
[![GPL-3.0 License][license-shield]][license-url]

</div>

## What is Aderyn?

**Aderyn is an open-source public good developer tool.** It is a Rust-based solidity smart contract static analyzer designed to help protocol engineers and security researchers find vulnerabilities in Solidity code bases.

Thanks to its collection of static vulnerability detectors, running Cyfrin Aderyn on your Solidity codebase will **highlight potential vulnerabilities**, drastically reducing the potential for unknown issues in your Solidity code and giving you the time to focus on more complex problems.

Built using **Rust**, Aderyn integrates seamlessly into small and **enterprise-level development workflows**, offering lighting-fast command-line functionality and a framework to [build custom detectors](https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli/detectors-quickstart) to adapt to your codebase.

You can read the [Cyfrin official documentation](https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli/readme) for an in-depth look at Aderyn's functionalities.

There is also an officially supported [VSCode extension](https://github.com/Cyfrin/vscode-aderyn/) for Aderyn. Download from the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=Cyfrin.aderyn&ssr=false#overview) and start identifying vulnerabilities in your Solidity code with ease.  

## Features

- Supports any development framework (Foundry/Hardhat/Truffle/etc)
- Modular [detectors](./aderyn_core/src/detect/)
- AST Traversal
- Markdown reports

## Installation

> **NOTE** Windows users must have WSL installed

### Using Cyfrinup

**Cyfrinup** simplifies the installation and management of Cyfrin tools.

Follow the instructions to install [here](https://github.com/Cyfrin/up).

Run `aderyn --version` to check the installation.

##### Upgrade older versions by (re)running: `cyfrinup`

---

### Using Homebrew

```sh
brew install cyfrin/tap/aderyn
```

##### Upgrade older versions by running: `brew upgrade cyfrin/tap/aderyn`

---

### Using npm

```sh
npm install @cyfrin/aderyn -g
```

##### Upgrade older versions by (re)running: `npm install @cyfrin/aderyn -g`

---

If you are installing with Homebrew or npm, ensure that the correct version of Aderyn in your path comes from either the Homebrew or npm global packages directory. If an older version exists at `~/.cyfrin/bin/aderyn`, remove it using `rm -f ~/.cyfrin/bin/aderyn`, as this is no longer the default installation location.

## Quick Start

[Quick Start](https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli/quickstart) example with video guide.

```
cd path/to/foundry/project/root
aderyn
```

For browsing more options, see
```
aderyn --help
```

More details and examples in [docs](https://cyfrin.gitbook.io/cyfrin-docs/cli-options)

## VS Code extension

Officially supported [VSCode extension](https://github.com/Cyfrin/vscode-aderyn/) for Aderyn. 
Download from [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=Cyfrin.aderyn&ssr=false#overview)

## Docker

You can run Aderyn from a Docker container.

Build the image:
```sh
  docker build -t aderyn .
```

`/path/to/project/root` should be the path to your Foundry or Hardhat project root directory and it will be mounted to `/share` in the container.

Run Aderyn from docker:
```sh
  docker run -v /path/to/project/root/:/share aderyn
```

## Contributing & License

Help us build Aderyn 🦜 Please see our [contribution guidelines](./CONTRIBUTING.md) for PR approval process and in-depth developer environment setup.
Aderyn is an open-source software licensed under the [GPL-3.0 License](./LICENSE).

To build Aderyn locally:

1. [Install Rust](https://www.rust-lang.org/tools/install),
2. Clone this repo and `cd aderyn/`,
3. `make`,
4. Use [`cargo`](https://doc.rust-lang.org/cargo/getting-started/first-steps.html) commands to build, test and run locally.

**Suggested VSCode extensions:**
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax) - Rust language support for Visual Studio Code
[Rust Syntax](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax) - Improved Rust syntax highlighting

## Building a custom Aderyn detector

Aderyn makes it easy to build Static Analysis detectors that can adapt to any Solidity codebase and protocol. This guide will teach you how to build, test, and run your custom Aderyn detectors.
To learn how to create your custom Aderyn detectors, [checkout the official docs](https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli/detectors-quickstart)


## Credits

This project exists thanks to all the people who [contribute](/CONTRIBUTING.md).<br>

<a href="https://github.com/cyfrin/Aderyn/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=cyfrin/Aderyn" />
</a>

## Attribution

- AST Visitor code from [solc-ast-rs](https://github.com/hrkrshnn/solc-ast-rs).
- Foundry Compilers for backend AST generation [foundry-compilers](https://github.com/foundry-rs/compilers)
- Original detectors based on [4naly3er](https://github.com/Picodes/4naly3er) detectors.
- Shoutout to the original king of static analysis [slither](https://github.com/crytic/slither).

[contributors-shield]: https://img.shields.io/github/contributors/cyfrin/aderyn
[contributors-url]: https://github.com/cyfrin/aderyn/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/cyfrin/aderyn
[forks-url]: https://github.com/cyfrin/aderyn/network/members
[stars-shield]: https://img.shields.io/github/stars/cyfrin/aderyn
[stars-url]: https://github.com/cyfrin/aderyn/stargazers
[issues-shield]: https://img.shields.io/github/issues/cyfrin/aderyn
[issues-url]: https://github.com/cyfrin/aderyn/issues
[license-shield]: https://img.shields.io/github/license/cyfrin/aderyn?logoColor=%23fff&color=blue
[license-url]: https://github.com/cyfrin/aderyn/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
