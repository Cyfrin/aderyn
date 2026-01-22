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

**Prerequisites**

- [Rust](https://www.rust-lang.org/tools/install)
- [Just command runner](https://just.systems/man/en/)

**Getting started**

```bash
git clone https://github.com/Cyfrin/aderyn.git
cd aderyn
just setup
```

**Development workflow**

1. Work on the issue and write unit tests
   - Add Solidity test files to `tests/contract-playground/` as needed
   - Run specific tests with `cargo test <test-name>`

2. Generate reports to verify your changes
   ```bash
   cargo prep -n playground   # Generate report for contract-playground
   cargo prep                 # Show all available test projects
   ```

3. Polish your PR before submitting
   ```bash
   cargo blesspr              # Run all checks to satisfy CI
   ```

4. Open a pull request to the `dev` branch. A maintainer (@alexroan or @TilakMaddy) will review it.

**Code quality**

Before submitting, run these commands to fix formatting and lint issues:
```bash
cargo fixfmt
cargo fixclippy
```

**Suggested VSCode extensions**

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - Rust language support
- [Rust Syntax](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax) - Improved syntax highlighting

#### Advanced setup (optional)

For a better development experience, install [Bacon](https://dystroy.org/bacon/) and run `bacon` at the project root. Keyboard shortcuts:

| Key | Action |
|-----|--------|
| `t` | Run tests |
| `r` | Generate report for contract-playground |
| `a` | Generate all reports in parallel |
| `Alt-b` / `⌥-b` | Bless the PR (run all CI checks) |

#### Adding a new test project

Test projects live in `tests/` and are registered in `reportgen.toml`. To add a new one:

1. Create a directory under `tests/` with your Solidity files

2. Add a configuration entry to `reportgen.toml`:
   ```toml
   [[reports]]
   name = "my-project"              # Used as: cargo prep -n my-project
   description = "My test project"
   root = "tests/my-project"
   ```

3. Generate the baseline report:
   ```bash
   cargo prep -n my-project
   ```
   This creates `reports/my-project-report.md`.

See `reportgen.toml` for all available configuration options.

**Project types**

- **Foundry projects**: Include a `foundry.toml`. Use `args` to specify source directories.
- **Standalone Solidity files**: Just add `.sol` files. Optionally include an `aderyn.toml` for configuration.
- **Hardhat projects**: Works out of the box if compilation artifacts exist.

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
