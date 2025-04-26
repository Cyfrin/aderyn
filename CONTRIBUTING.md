# Welcome to the Cyfrin Aderyn Contributing Guide

This guide aims to provide an overview of the contribution workflow to help make the contribution process effective for everyone involved.

## About the Project

Aderyn is a Rust-based solidity smart contract static analyzer designed to help protocol engineers and security researchers find vulnerabilities in Solidity code bases.

Thanks to its detectors, running Cyfrin Aderyn on your Solidity codebase will highlight all the issues currently supported, drastically reducing the potential for unknown vulnerabilities in your Solidity code.

Aderyn also makes it easier for smart contract engineers and protocols to create custom detectors to find specific or unsupported code vulnerabilities.

Read the [README](README.md) and consult the docs for an in-depth project overview.

### Vision

Cyfrin Aderyn aims to give engineers and smart contract security researchers reliable and open-source Solidity static analysis tools to find and suggest solutions to smart contract vulnerabilities while adapting quickly and efficiently to different codebases.

### Project Status

The project is under active development.

You can contribute to this repo in many ways:

- Solve open issues
- Report bugs or feature requests
- Improve the documentation

Contributions are made via Issues and Pull Requests (PRs). A few general guidelines for contributions:

- Search for existing Issues and PRs before creating your own.
- Contributions should only fix/add the functionality in the issue OR address style issues, not both.
- If you're running into an error, please give context. Explain what you're trying to do and how to reproduce the error.

## Getting started

### Overview 

Indexed in [DeepWiki](https://deepwiki.com/Cyfrin/aderyn)

### Pull Requests

#### Developer environment setup

1. [Install Rust](https://www.rust-lang.org/tools/install),
2. Clone this repo and `cd aderyn/`
3. Run `make` . Make sure to have tools listed at the top of the Makefile installed in your computer.
4. Work on the issue, write unit tests. Use `cargo test <test-name>` to test. Feel free to add solidity files to `tests/contract-playground`.
5. Run `cargo prep --cpg` to generate the report for the same.
6. Once happy with the work, run `cargo blesspr` to "polish" your PR so CI can be happy.
7. Create a pull request to `dev` branch here. The maintainers will be notified. Either @alexroan or @TilakMaddy will reach out to you.

Suggested VSCode extensions
* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax) - Rust language support for Visual Studio Code
* [Rust Syntax](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax) - Improved Rust syntax highlighting

#### Advanced Setup
1. Install [Bacon](https://dystroy.org/bacon/)
2. Run `bacon` at the root
3. Press
  * `t` for tests
  * `r` for generating a report on contract-playground. Shorthand for `cargo prep --cfg`
  * `a` for generating all reports. Shorthand for `cargo prep --all --parallel`
  * `Alt-b` or `⌥-b` (Option-B on Mac) for "blessing" the PR. Shorthand for `cargo blesspr`

#### Tips
Feel free to reach out to `cargo fixfmt` and `cargo fixclippy` to apply quick fixes on code quality.

#### Pull Request Process

We follow the ["fork-and-pull" Git workflow](https://github.com/susam/gitpr)

1. Fork the repo
2. Clone the project
3. Create a new branch with a descriptive name
4. Commit your changes to the new branch
5. Push changes to your fork
6. Open a PR in our repository and tag one of the maintainers to review your PR

Here are some tips for a high-quality pull request:

- Create a title for the PR that accurately defines the work done.
- Structure the description neatly to make it easy for the readers to consume. For example, you can include bullet points and screenshots instead of having one large paragraph.
- Add the link to the issue if applicable.
- Have a good commit message that summarises the work done.

Once you submit your PR:

- We may ask questions, request additional information, or ask for changes to be made before a PR can be merged. These are to clarify the PR for everyone involved and create a frictionless interaction process.
- As you update your PR and apply changes, mark each conversation resolved.

Once the PR is approved, we'll "squash-and-merge" to keep the git commit history clean.

### Issues

Issues should be used to report problems, request a new feature, or discuss potential changes before a PR is created.

#### Solve an issue

Please review our [existing issues](https://github.com/cyfrin/aderyn/issues) to find one that interests you.

If a contributor is working on the issue, they will be assigned to the individual. If you find an issue to work on, you can assign it to yourself and open a PR with a fix.

#### Report Bugs

If a related issue doesn't exist, you can open a new issue.

Some tips to follow when you are creating an issue:

- Provide as much context as possible. Over-communicate to give the most details to the reader.
- Include the steps to reproduce the issue or the reason for adding the feature.
- Screenshots, videos, etc., are highly appreciated.

