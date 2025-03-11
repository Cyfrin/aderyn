# Release checklist

This checklist is meant to be used as a guide for the `aderyn` release process.

Releases are always made in lockstep, meaning that all crates in the repository
are released with the same version number, regardless of whether they have
changed or not.

## Requirements

- [cargo-release](https://github.com/crate-ci/cargo-release): `cargo install cargo-release`

## Pre-requisites

- Code in the `dev` branch must be in good shape for release.
- If there are breaking changes, a newer version of VSCode extension is ready to be released in it's `main` branch.

## Steps

- [ ] Cut a release from `dev` branch.
    - [ ] Run `git checkout dev`
    - [ ] Run `cargo release patch --no-publish` to see a dry run of all the incremented versions.
    - [ ] Run `cargo release patch --no-publish --execute` and expect the following in CI:
        - [ ] Sarif report tests fail because of version mismatch.
        - [ ] After the building of global artifacts, library crates fail to publish.
        - [ ] Binary crate `aderyn` is successfully published.
    - [ ] Wait for the CI/CD process to be over. (Important before proceeding)
    - [ ] Run `cli/reportgen.sh` to regenerate sarif-report locally, then commit & push.
    - [ ] Verify now that all the tests pass.
- [ ] Create a checkpoint on `master`.
    - [ ] Run `git checkout master`
    - [ ] Run `git checkout -b master-merge-<version-to-be-released>`.
    - [ ] Merge `dev` into it and preserve all changes in `dev`. `git merge --squash -X theirs dev`.
- [ ] Switch back to `dev` branch with `git checkout dev`.

> NOTE: Replace `patch` with `minor` or `major` based on the needs.

## Breaking change

If the at least any one of the following is observed, it's a breaking change. If so, at minimum the minor version must be bumped.
This is to ensure VS Code extension doesn't face compatibility issues (as the users will be prompted to install new version of extension)

- [ ] Changes in CLI flags - `--lsp`, `--json`, `--stdout`
- [ ] Changes in the JSON output format.
- [ ] Arbitrary Changes to LSP server.
- [ ] Changes to asset names or URLs in Github Releases.
- [ ] Changes to `aderyn.toml` template.

Above is a non-exhaustive list There can be more.
