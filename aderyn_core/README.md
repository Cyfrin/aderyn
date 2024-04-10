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

# Aderyn Core

aderyn_core is the backend infrastructure for Aderyn.

```
src/
├─ ast/             // Solidity AST Structs
├─ context/         // Solidity project context to be analyzed
├─ detect/          // Where Detector specifications and logic live
├─ framework/       // Foundry/Hardhat detection
├─ fscloc/          // Solidity file stats
├─ report/          // Report printers
├─ visitor/         // AST visitors
```
