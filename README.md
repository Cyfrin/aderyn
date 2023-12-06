

<p align="center">
    <br />
    <a href="https://updraft.cyfrin.io/">
        <img src=".github/images/aderyn_logo.png" width="400" alt=""/></a>
    <br />
</p>
<p align="center"><strong>A powerful Solidity static analyzer that takes a bird's eye view over your smart contracts.
</strong></p>
<p align="center">Powered by Cyfrin.</p>



<p align="center">
<a href="https://twitter.com/cyfrinaudits">Twitter</a>
<a href="https://cyfrin.io">Website</a>
<a href="https://discord.gg/cyfrin">Discord</a>
<p>

---

<div align="center">

[![Stargazers][stars-shield]][stars-url] [![Forks][forks-shield]][forks-url]
[![Contributors][contributors-shield]][contributors-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

</div>





## What is Aderyn?

Aderyn is a Solidity Static Analyzer. It takes a bird's eye view over your smart contracts, traversing the Abstract Syntax Trees (AST) to pinpoint suspected vulnerabilities. Aderyn prints out these potential issues in an easy-to-consume markdown format.

## When/Why to Use it?

Use Aderyn when developing or auditing Solidity smart contracts to quickly identify areas where the code may not be following best practices or has potential vulnerabilities.

## Features

* [Hardhat](https://hardhat.org/) and [Foundry](https://book.getfoundry.sh/) support
* Modular [detectors](./src/detect/)
* AST Traversal
* Markdown reports

# Usage

To get started using Aderyn make sure to have Rust installed on your device. For more information, refer to the [official Rust documentation](https://www.rust-lang.org/tools/install).

## Mac, Linux, Unix

You can install Rust and Cargo by running the following command on your terminal:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 ```

## Windows

You can install Rust and Cargo by downloading and running [`rustup-init.exe`](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe).

## Installation

In the command line, run:
```sh
cargo install aderyn
```

## Quick Start

The project you're running Aderyn on should be either a **Foundry** or compiled **Hardhat** project.

```sh
aderyn /path/to/your/foundry/project/root/directory/
```

That's it! Aderyn identifies whether the project root is a Foundry or Hardhat repo, then uses the compiled AST files to hunt for vulnerabilities. 

`report.md` will be output **in the directory in which you ran the command.**

### Arguments

You must provide the root directory of the repo you want to analyze. 

Examples:

```sh
aderyn /path/to/your/foundry/project/root/directory/
```

To run Aderyn in the folder you're currently on, run:


```sh
aderyn .
```
## Supported Development Frameworks

Aderyn automatically detects the development framework so long as it's Foundry or Hardhat. 

### Foundry

If Foundry is detected in the project root, Aderyn will first run `forge build` to ensure that the contract compiles correctly and the latest artifacts are available.

### Hardhat

If Hardhat is detected, Aderyn does not auto-compile. Make sure to run `hardhat compile` BEFORE running Aderyn. 



# Roadmap

**Medium-term goals - Auditor Aid:**
* [x] Support Multiple Abstract Syntax Trees representing multiple Solidity files
* [x] Support Foundry/Hardhat/Truffle/Solc output formats for ingesting AST
  * [x] Foundry
  * [x] Hardhat
* [ ] Complexity score (with Percentage YUL code & nsloc)
* [ ] More complex static analysis detectors
* [ ] auto-fixes
* [ ] installer that doesn't require Rust (aderynup)

**Long-term goals - Product**

* [ ] Provide automated gas optimizations
* [ ] Custom subscribable detectors
* [ ] Control/data flow analyses
* [ ] Symbolic execution
* [ ] Invariant handler generation
* [ ] Vyper support

## Contributing & License

Help us build Aderyn 🦜 Please see our [contribution guidelines](./CONTRIBUTING.md).
Aderyn is an open source software licensed under the [MIT License](./LICENSE).

To build Aderyn locally, [install Rust](https://www.rust-lang.org/tools/install), clone this repo, and use [`cargo`](https://doc.rust-lang.org/cargo/getting-started/first-steps.html) commands to build, test and run locally

## Credits

This project exists thanks to all the people who [contribute](/contributing.md).<br>

<a href="https://github.com/cyfrin/Aderyn/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=cyfrin/Aderyn" />
</a>

## Attribution
* AST Visitor code from [solc-ast-rs](https://github.com/hrkrshnn/solc-ast-rs).
* Original detectors based on [4naly3er](https://github.com/Picodes/4naly3er) detectors.


[contributors-shield]: https://img.shields.io/github/contributors/cyfrin/aderyn
[contributors-url]: https://github.com/cyfrin/aderyn/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/cyfrin/aderyn
[forks-url]: https://github.com/cyfrin/aderyn/network/members
[stars-shield]: https://img.shields.io/github/stars/cyfrin/aderyn
[stars-url]: https://github.com/cyfrin/aderyn/stargazers
[issues-shield]: https://img.shields.io/github/issues/cyfrin/aderyn
[issues-url]: https://github.com/cyfrin/aderyn/issues
[license-shield]: https://img.shields.io/github/license/cyfrin/aderyn?logoColor=%23fff&color=blue
[license-url]: https://github.com/cyfrin/aderyn/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555

## Changelog

* 0.0.8 - Parallel processing foundry output files to slow down time