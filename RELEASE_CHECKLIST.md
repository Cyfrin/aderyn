# Release checklist

This checklist is meant to be used as a guide for the `aderyn` release process.

Releases are always made in lockstep, meaning that all crates in the repository
are released with the same version number, regardless of whether they have
changed or not.

## Requirements

- [cargo-release](https://github.com/crate-ci/cargo-release): `cargo install cargo-release` (v0.25.22)
- [gh](https://cli.github.com/): `brew install gh` (v2.53.0)

Stay logged in with `gh auth login`

## Pre-requisites

- Code in the `dev` branch must be in good shape for release.
- If there are breaking changes, a newer version of VSCode extension is ready to be released in it's `main` branch.

## Steps to cut a release

1. Switch to dev branch and pull latest changes.
   
2. Run `cargo patch`. Now, wait until the command is fully done. In the terminal, you should see something on the lines of "waiting for release completion". This process will take ~30 minutes as it is constantly polling Github Actions API to determine when release is complete. After that it shoots of a post completion script.

> NOTE: If for some reason, you weren't able to keep the terminal open - you have to run `cargo blesspr` locally _after the whole release process is done (from `dev` branch)_. 
  
3. Finally, in the Releases page, click on `Generate Release Notes` for the latest release.
   

- **NOTE: Expect the following to happen in CI**:
  * Sarif report tests fail because of version mismatch.(But it's okay, an automatic script will shoot off to fix this after the whole release is complete)
  * Binary crate `aderyn` is successfully published.

> NOTE: Replace `patch` with `minor` if it is a breaking change. Read below to find out more.

To monitor the changes, visit the Actions tab of this repository and click on Releases workflow. Then search for the workflow event `aderyn-v0.x.x` You can ignore the remaining ones.

<img width="1075" height="604" alt="Screenshot 2026-01-11 at 1 00 23 PM" src="https://github.com/user-attachments/assets/21c0acd1-6cfa-4c8d-b5ee-14581b3ba6fe" />



## Breaking change

**If at least one of the following is observed, it's considered a breaking change. In that case, the minor version must be bumped at a minimum.** This ensures the VS Code extension remains compatible, as users will be prompted to install the updated version.

- [ ] Changes in CLI flags - `--lsp`, `--json`, `--stdout`
- [ ] Changes in the JSON output format.
- [ ] Arbitrary Changes to LSP server.
- [ ] Changes to asset names or URLs in Github Releases.
- [ ] Changes to `aderyn.toml` template.

Above is a non-exhaustive list There can be more.

## Implication of bumping up minor version

For non breaking changes, cut a patch release and everything will be fine.

**For a breaking change, when you cut a minor release of Aderyn, a new VS Code extension release is required**. 
This is because we assume the Typescript code needs to adapt it's way of communicating with Aderyn in order to accommodate breaking changes. Such as starting LSP, parsing JSON encoded vulnerabilities, etc.  

**New version of VS Code extension must be released _after_ releasing Aderyn.**
This is so that when people download VS Code extension, there is ALWAYS an Aderyn Version that is compatible with it.

Make sure to adjust the supported Aderyn versions in the VS Code extension here - https://github.com/Cyfrin/vscode-aderyn/blob/main/package.json#L29C2-L32C5


