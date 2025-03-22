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
    - [ ] Run `git checkout dev && git pull`
    - [ ] Run `cargo release --no-publish --exclude xtask --no-tag` to see a dry run of all the incremented versions.
    - [ ] Run `cargo release --no-publish --exclude xtask --execute` and expect the following in CI:
        - [ ] Sarif report tests fail because of version mismatch.
        - [ ] After the building of global artifacts, library crates fail to publish.
        - [ ] Binary crate `aderyn` is successfully published.
    - [ ] Wait for the CI/CD process to be over. (Important before proceeding)
    - [ ] Run `cli/reportgen.sh` to regenerate sarif-report locally, then commit & push.
    - [ ] Verify now that all the tests pass.
- [ ] Create a checkpoint on `master`.
    - [ ] Run `git checkout master && git pull`
    - [ ] Run `git checkout -b master-merge-<version-to-be-released>`.
    - [ ] Merge `dev` into it and preserve all changes in `dev`. `git merge --squash -X theirs dev`.
    - [ ] Merge it back to master by creating a PR (Verify CI tests)
- [ ] Switch back to `dev` branch with `git checkout dev`.

> NOTE: Replace `patch` with `minor` or `major` based on the needs.

## Breaking change

If at least one of the following is observed, it's considered a breaking change. In that case, the minor version must be bumped at a minimum. This ensures the VS Code extension remains compatible, as users will be prompted to install the updated version.

- [ ] Changes in CLI flags - `--lsp`, `--json`, `--stdout`
- [ ] Changes in the JSON output format.
- [ ] Arbitrary Changes to LSP server.
- [ ] Changes to asset names or URLs in Github Releases.
- [ ] Changes to `aderyn.toml` template.

Above is a non-exhaustive list There can be more.
