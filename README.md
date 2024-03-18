

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

Aderyn is a Rust-based static analyzer specifically designed for Web3 smart contract security and development. It takes a bird's eye view over your smart contracts, traversing the Abstract Syntax Trees (AST) to pinpoint suspected vulnerabilities. Aderyn prints out these potential issues in an easy-to-consume markdown format.

## Features

* [Hardhat](https://hardhat.org/) and [Foundry](https://book.getfoundry.sh/) support
* Modular [detectors](./aderyn_core/src/detect/)
* AST Traversal
* Markdown reports
* Bot development framework ([Nyth](./nyth/))

# Usage

To get started using Aderyn make sure to have Rust installed on your device. For more information, refer to the [official Rust documentation](https://www.rust-lang.org/tools/install).

## Mac, Linux, Unix

You can install Rust and Cargo by running the following command on your terminal:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 ```

## Windows

You can install Rust and Cargo by downloading and running [`rustup-init.exe`](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe).

## Docker

You can run Aderyn from Docker container. 

Build the image:
```sh
  docker build -t aderyn .
```
`/path/to/project/root` should be the path to your Foundry or Hardhat project root directory and it will be mounted to `/share` in the container.

Run Aderyn in Docker:
```sh
  docker run -v /path/to/project/root/:/share aderyn
```
Run Aderyn with flags in Docker:
```sh
  docker run -v /path/to/project/root/:/share aderyn -h
```

## Installation

In the command line, run:
```sh
cargo install aderyn
```

## Quick Start

The root path you're running Aderyn on should be either a **Foundry** or compiled **Hardhat** project.

```sh
aderyn /path/to/your/foundry/project/root/directory/
```

That's it! Aderyn identifies whether the project root is a Foundry or Hardhat repo, then uses the compiled AST files to hunt for vulnerabilities. 

`report.md` will be output **in the directory in which you ran the command.**


### Arguments

Usage: `aderyn [OPTIONS] <ROOT>`

`<ROOT>`: The path to the root of the codebase to be analyzed. Defaults to the current directory.

Options:
  - `-o`, `--output <OUTPUT>`: Desired file path for the final report (will overwrite existing one) [default: report.md]
  - `-s`, `--scope <SCOPE>`: List of path strings to include, delimited by comma (no spaces). Any solidity file path not containing these strings will be ignored
  - `-e`, `--exclude <EXCLUDE>`: List of path strings to exclude, delimited by comma (no spaces). Any solidity file path containing these strings will be ignored
  - `-n`, `--no-snippets`: Do not include code snippets in the report (reduces report size in large repos)
  - `-h`, `--help`: Print help
  - `-V`, `--version`: Print version


You must provide the root directory of the repo you want to analyze. Alternatively, you can provide a single Solidity filepath (this mode requires [Foundry](https://book.getfoundry.sh/) to be installed).

Examples:

```sh
aderyn /path/to/your/foundry/project/root/directory/
```

Run Aderyn in the folder you're currently in:

```sh
aderyn
```

Output to a different markdown file:

```sh
aderyn -o output.md ./path/to/repo/
```

Refine the scope to a subdirectory called `/uniswap/`:

```sh
aderyn --scope uniswap ./path/to/repo/
```

Exclude a contract called `Counter.sol`:

```sh
aderyn --exclude Counter.sol ./path/to/repo/
```

Run on a single Solidity file (requires [Foundry](https://book.getfoundry.sh/) to be installed on your machine):

```sh
aderyn src/MyContract.sol
```

## Supported Development Frameworks

If the `<ROOT>` is a directory, Aderyn automatically detects the development framework so long as it's Foundry or Hardhat. 

### Foundry

If Foundry is detected in the project root, Aderyn will first run `forge build` to ensure that the contract compiles correctly and the latest artifacts are available.

### Hardhat

If Hardhat is detected, Aderyn does not auto-compile. Make sure to run `hardhat compile` BEFORE running Aderyn. 

## Single Solidity File Mode

If it is a Solidity file path, then Aderyn will create a temporary Foundry project, copy the contract into it, compile the contract and then analyze the AST generated by that temporary project.

## Contributing & License

Help us build Aderyn 🦜 Please see our [contribution guidelines](./CONTRIBUTING.md).
Aderyn is an open source software licensed under the [MIT License](./LICENSE).

To build Aderyn locally, [install Rust](https://www.rust-lang.org/tools/install), clone this repo, and use [`cargo`](https://doc.rust-lang.org/cargo/getting-started/first-steps.html) commands to build, test and run locally

## Credits

This project exists thanks to all the people who [contribute](/CONTRIBUTING.md).<br>

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
