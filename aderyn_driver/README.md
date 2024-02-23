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
1. Framework detection.
   * Aderyn uses the compiled AST of smart contracts to analyze them.
   * Foundry and Hardhat projects are supported, and the ASTs are loaded depending on which one is detected.
2. Build the `WorkspaceContext`.
   * Upon loading the AST, context is built up within the `WorkspaceContext` that detectors can access.
3. Run Detectors on the `WorkspaceContext`.
   * Each detector is run on the context and issues are stored in each detector.
4. Report printed.
   * Each detector is passed to the report printer, which prints each found issue in the report.
