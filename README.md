

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
<a href="https://docs.cyfrin.io">Docs</a>
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

Built using **Rust**, Aderyn integrates seamlessly into small and **enterprise-level development workflows**, offering lighting-fast command-line functionality and a framework to [build custom detectors](https://docs.cyfrin.io/aderyn-custom-detectors/what-is-a-detector) to adapt to your codebase.

You can read the [Cyfrin official documentation](https://docs.cyfrin.io) for an in-depth look at Aderyn's functionalities.

## Features
* Supports any development framework (Foundry/Hardhat/Truffle/etc)
* Modular [detectors](./aderyn_core/src/detect/)
* AST Traversal
* Markdown reports

## Installation

### Prerequisites
Before installing Aderyn, ensure you have the following:
* Rust: Aderyn is built in Rust. Before running, you must install Rust and Cargo (Rust's package manager). If you still need to install Rust, follow the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started).

**Suggested VSCode extensions:**
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax) - Rust language support for Visual Studio Code
[Rust Syntax](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax) - Improved Rust syntax highlighting

### Installing Aderyn
**Step 1: Install Aderyn using cargo**

Aderyn is currently installed using Cargo, Rust's package manager. Open your command line interface and run the following command:
```sh
cargo install aderyn
```
This command downloads and installs the Aderyn package.

**Step 2: Verify installation**

After the installation, you can verify that Aderyn is correctly installed by checking its version. In your command line, execute:
```sh
aderyn --version
```
This command should return the installed version of Aderyn, confirming that the installation was successful.

**Step 3: Update PATH (if necessary)**

If you cannot run the aderyn after installation, you may need to add Cargo's bin directory to your PATH. The exact instructions can vary based on your operating system. Typically, it involves adding ~/.cargo/bin to your PATH in your shell profile script (like .bashrc or .zshrc).

**Step 4: Future Updates**

To update Aderyn to the latest version, you can run the install command again:
```sh
cargo install aderyn
```
Cargo will replace the existing version with the latest one.

## Quick Start
Once Aderyn is installed on your system, you can run it against your Foundry-based codebase to find vulnerabilities in your code.

We will use the [aderyn-contracts-playground](https://github.com/Cyfrin/aderyn-contracts-playground) repository in this example. You can follow along by cloning it to your system:
```sh
git clone https://github.com/Cyfrin/aderyn-contracts-playground.git
```
Navigate inside the repository:
```sh
cd aderyn-contracts-playground
```
We usually use several smart contracts and tests to try new detectors. Build the contracts by running:
```sh
forge build
```
Building your project by running forge build --ast will save you time the first time you run Aderyn.
Once your smart contracts have been successfully compiled, run Aderyn using the following command:
```sh
aderyn [OPTIONS] path/to/your/project
```
Replace [OPTIONS] with specific command-line arguments as needed.

For an in-depth walkthrough on how to get started using Aderyn, check the [Cyfrin official docs](https://docs.cyfrin.io/aderyn-static-analyzer/quickstart)

### Arguments

Usage: `aderyn [OPTIONS] <ROOT>`

`<ROOT>`: The path to the root of the codebase to be analyzed. Defaults to the current directory.

Options:
  - `-s`, `--src`: Path to the source contracts. If not provided, or if aderyn can't find famous files to read (like `foundry.toml`, which it automatically searches for) the ROOT directory will be used.
    - In foundry projects, this is usually the `src/` folder unless stated otherwise in `foundry.toml`.
    - In Hardhat projects, this is usually the `contracts/` folder unless stated otherwise in the config.
  - `-i`, `--path-includes <PATH_INCLUDES>`: List of path strings to include, delimited by comma (no spaces). Any solidity file path not containing these strings will be ignored
  - `-x`, `--path-excludes <PATH_EXCLUDES>`: List of path strings to exclude, delimited by comma (no spaces). Any solidity file path containing these strings will be ignored
  - `-o`, `--output <OUTPUT>`: Desired file path for the final report (will overwrite the existing one) [default: report.md]
  - `-n`, `--no-snippets`: Do not include code snippets in the report (reduces report size in large repos)
  - `-h`, `--help`: Print help
  - `-V`, `--version`: Print version


You must provide the root directory of the repo you want to analyze. Alternatively, you can provide a single Solidity file path (this mode requires [Foundry](https://book.getfoundry.sh/) to be installed).

Examples:

```sh
aderyn /path/to/your/foundry/project/root/directory/
```
Find more examples on the official [Cyfrin Docs](https://docs.cyfrin.io)

## Building a custom Aderyn detector
Aderyn makes it easy to build Static Analysis detectors that can adapt to any Solidity codebase and protocol. This guide will teach you how to build, test, and run your custom Aderyn detectors.
To learn how to create your custom Aderyn detectors, [checkout the official docs](https://docs.cyfrin.io/aderyn-custom-detectors/detectors-quickstart)

## Docker

You can run Aderyn from a Docker container. 

Build the image:
```sh
  docker build -t aderyn .
```
`/path/to/project/root` should be the path to your Foundry or Hardhat project root directory and it will be mounted to `/share` in the container.

Run Aderyn:
```sh
  docker run -v /path/to/project/root/:/share aderyn
```
Run with flags:
```sh
  docker run -v /path/to/project/root/:/share aderyn -h
```

## Single Solidity File Mode

If it is a Solidity file path, then Aderyn will create a temporary Foundry project, copy the contract into it, compile the contract and then analyze the AST generated by that temporary project.

## Contributing & License

Help us build Aderyn 🦜 Please see our [contribution guidelines](./CONTRIBUTING.md).
Aderyn is an open-source software licensed under the [GPL-3.0 License](./LICENSE).

To build Aderyn locally:
1. [Install Rust](https://www.rust-lang.org/tools/install),
2. Clone this repo and `cd aderyn/`,
3. `make`,
4. Use [`cargo`](https://doc.rust-lang.org/cargo/getting-started/first-steps.html) commands to build, test and run locally.

## Credits

This project exists thanks to all the people who [contribute](/CONTRIBUTING.md).<br>

<a href="https://github.com/cyfrin/Aderyn/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=cyfrin/Aderyn" />
</a>

## Attribution
* AST Visitor code from [solc-ast-rs](https://github.com/hrkrshnn/solc-ast-rs).
* Original detectors based on [4naly3er](https://github.com/Picodes/4naly3er) detectors.
* Shoutout to the original king of static analysis [slither](https://github.com/crytic/slither).


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
