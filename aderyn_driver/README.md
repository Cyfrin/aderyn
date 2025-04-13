<p align="center">
    <br />
    <a href="https://cyfrin.io/">
        <img src="../.github/images/aderyn_logo.png" width="400" alt=""/></a>
    <br />
</p>
<p align="center"><strong>A powerful Solidity static analyzer that takes a bird's eye view over your smart contracts.
</strong></p>
<p align="center">
    <br />
    <a href="https://cyfrin.io/">
        <img src="../.github/images/poweredbycyfrinblue.png" width="145" alt=""/></a>
    <br />
</p>


<p align="center">
<a href="https://twitter.com/cyfrinaudits">Twitter</a>
<a href="https://cyfrin.io">Website</a>
<a href="https://discord.gg/cyfrin">Discord</a>
<p>

# Aderyn Driver

aderyn_driver drives the process of running aderyn over a codebase, utilizing [aderyn_core](../aderyn_core).

Order of operation:
1. Configuration
   * Aderyn detects the source folder containing the contracts as well as the remappings to enable scan.
2. Build the `WorkspaceContext`.
   * Uses `solidity-ast-rs` to get the ASTs of the solidity files and then createss a Workspace Context out of the same.
4. Calls `aderyn_core::detect_issues` and then serializes the report based on output file format
