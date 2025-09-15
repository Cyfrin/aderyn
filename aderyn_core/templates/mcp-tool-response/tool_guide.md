## Tool Guide

### Preface:
This guide will first introduce essential terminology for the whole interface. Later, some general approaches with examples will be provided followed by tool call strategies that other LLMs have used to improve the quality of the answers that were provided to the user. It is assumed that you are already well versed with Solidity programming and other blockchain concepts to follow along. The hope is that after learning and following the guide, you will learn to come up with your own strategies and answer even more complex questions that the user may have for you.

-----

### Table of Contents:

#### 1. Glossary of terms:
  - **Solc**
  - **Compilation unit**
  - **Compilation unit index**
  - **Included file**

#### 2. General approaches:
  - **Chain of thought**
  - **Fast search**
  - **Hybrid search**

#### 3. Tool call strategies:
  - **Scenario 1**
  - **Scenario 2**
------

### 1. Glossary of terms

#### **Solc**

Solc is the official Solidity compiler. Each release is a new version. The pragma header at the beginning of the Solidity program dictates whether or not a given version of Solc can compile it.

#### **Compilation unit**

A compilation unit is a set of files that can all be compiled with the same version of a Solc compiler. A project can have multiple of these compilation units. This is largely determined by the version header ex - `pragma version ^0.8`. This version string is present at the beginning of every Solidity file. The important thing to note is that a file can be part of 1 or more compilation units. This is usually the library contracts because they are meant to support multiple versions of regular smart contracts.

To understand this simply, let's say hypothetically there exists the following set of files:
- A.sol : pragma version 0.8.5
- B.sol : pragma version 0.8.10
- Lib.sol : pragma version ^0.8.0

Say both A.sol and B.sol import Lib.sol.

In the above example, we can say there are 2 compilation units: The first one contains A.sol and Lib.sol (Solc version 0.8.5), the second one contains B.sol and Lib.sol (Solc version 0.8.10). Note that Lib.sol is a floating pragma, therefore it can compile with both Solc versions.

It is also not true that all the files in a given compilation unit are inter-dependent. There could be many independently operating sets of files within a compilation unit. The file import graph tool would provide more insight on this when inspected.

#### **Compilation unit index**

If there are N compilation units, the compilation unit index is a number to uniquely identify a compilation unit. Index counting starts from 1. For example, if there are 4 compilation units, possible values for compilation unit index are 1, 2, 3, and 4.

#### **Included file**
Included files are files that are in scope for issue reporting. The user decides this and also the framework used has a large say in this. Resolution of included files is already done and provided by Aderyn's project overview MCP tool. Typically that's the first tool to reach out for, before starting any analysis.

Note that files that are not included are typically dependencies of included files (like third-party libraries) which may need to be examined if warranted, but there is no requirement to report on any issues in these files.

----

### 2. General approaches:

Based on the problem, determine a good approach to reach the desired result while keeping the user satisfied.

#### **Chain of thought**

This approach is preferred when the user's query requires a deep understanding of transaction flow and callgraphs to provide an accurate result. Typically, there can be multiple tool calls, and we get to the answer slowly.

Think deeply and make a step-by-step plan to identify the solution of the given problem. The key here is to understand all of Aderyn's MCP tools provided to you and learn to best leverage them to get accurate results. Gather all the necessary data first before making a judgment about the end result.

#### **Fast search**

This approach is for simple problems that focus on the static nature of written contract code. Prefer calling simpler tools like node finder tools, node summarizer tools, or file import graph provider tool for this as there is no need to understand the whole context of surrounding code to determine the answer to these problems. An example would be - are there events that don't have any field indexed? OR What contracts have a dependency on OpenZeppelin?

#### **Hybrid search**

This approach combines both of the above approaches. Start off with chain of thought and then occasionally during the process use the fast search approach to quickly gather information about certain nodes to make decisions.

----

### 3. Tool call strategies:

**Scenario 1**

- *Given: Issue criteria that require understanding of the full surface area and interaction of a smart contract*
- *Goal: Find code that matches said criteria (if any)*

**Steps:**
1. First, use the project overview tool to see all the compilation units.
2. Analyses must operate on each compilation unit.
3. Start by calling the list contracts tool to enlist the contracts in a compilation unit.
4. Use the contract surface area inspection tool on the contracts of interest (stick to included ones only). This reveals methods, state variables, etc. that the contract inherits from its parent contracts, third-party libraries, as well as the various library contracts that it makes internal calls to, and more. It also exposes entrypoint functions in the contract.
5. Use the callgraph provider tool to gauge the different internal functions and modifiers that could be called as a result of executing a given entrypoint function in a contract.
6. Now, use the node summarizer tools to look inside these functions and modifiers. These tools will also output code snippets, so carefully use all the information gathered and evaluate it against the issue criteria. Report matches if found in included files.

#### Notes:

- The node summarizer tools provide a summary of just that node. There is no intelligence in them to perform resolutions of any kind that were not already present in the AST. All that to say, if there is an internal function call, the callgraph provider would help to resolve these to function definitions and modifier definitions. Later the node summarizer tools would help in looking inside.

- It is also noteworthy that the callgraph does not contain edges to external calls made. If there is a requirement to analyze that, then make a guess of the external contract that is being called to and re-use some of the methods described above on those contracts to gather full overview. Although, that's a very advanced use case, you might have to sometimes go through that.

- The file import graph provider is another tool that helps understand what files are connected to each other. So that means, if in a given compilation unit, there are 5 files and 3 of which are interdependent, and the remaining 2 are standalone the file import graph would expose this.

**Scenario 2**

- *Given: Issue criteria that are focused on simpler static checks like basic code patterns*
- *Goal: Find code that matches said criteria (if any)*

**Steps:**
1. First, use the project overview tool to see all the compilation units.
2. Analyses must operate on each compilation unit.
3. Use the node finder tools to enlist the nodes of a certain type that are relevant to the problem at hand.
4. Now, use the node summarizer tools to look inside these nodes. These tools will also output code snippets, so carefully use all the information gathered and evaluate it against the issue criteria. Report matches if found in included files.


----
