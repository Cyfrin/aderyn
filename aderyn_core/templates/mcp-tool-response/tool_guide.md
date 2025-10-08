## Tool Guide

### Preface:
This guide will first introduce essential terminology for the whole interface. Later, some general approaches with examples will be provided followed by tool call strategies that other LLMs have used to improve the quality of the answers that were provided to the user. It is assumed that you are already well versed with Solidity programming and other blockchain concepts to follow along. The hope is that after learning and following the guide, you will follow it diligently and if warranted, even come up with your own strategies to answer complex problems that the user may have.

-----

### Table of Contents:

#### 1. Glossary of terms:
  - **Solc**
  - **Compilation unit**
  - **Compilation unit index**
  - **Contract class**
  - **Included file**
  - **Node ID**
  - **Problem**
  - **Callgraph**

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

It is also not true that all the files in a given compilation unit are inter-dependent. There could be many independently operating sets of files within a compilation unit.

#### **Contract class**

In a Solidity codebase, every contract, abstract contract, or interface you write in source code is a contract class - a definition that describes state and behaviour but does not yet exist on-chain. Only non-abstract contract classes (i.e. concrete implementations) are deployable contract classes: they compile to bytecode that can be deployed and become a smart contract instance at an address. Abstract contracts and interfaces are still contract classes but serve purely as blueprints or type definitions; they cannot be deployed directly.

Note - When we say just contracts, it usually means deployable contract classes. For example the list contracts tool, as evident from its name, only lists deployable contract classes. Same way, the inspect contract surface tool only inspects deployable contract classes.

#### **Compilation unit index**

If there are N compilation units, the compilation unit index is a number to uniquely identify a compilation unit. Index counting starts from 1. For example, if there are 4 compilation units, possible values for compilation unit index are 1, 2, 3, and 4.

#### **Included file**

Included files are files that are in scope for issue reporting. The user decides this and also the framework used has a large say in this. Resolution of included files is already done and provided by Aderyn's project overview MCP tool. Typically that's the first tool to reach out for, before starting any analysis.

Note that files that are not included are typically dependencies of included files (like third-party libraries) which may need to be examined if warranted, but there is no requirement to report on any issues in these files.

#### **Node ID**

A node is an element in the Abstract Syntax Tree (AST) representation of a Solidity program. Each node can be uniquely identified within a given compilation unit by its Node ID - a unique identifier that serves as a reference to that specific AST element.

Node IDs are commonly required as arguments when calling MCP tools for detailed analysis. These identifiers are typically obtained from initial discovery tools (like node finder or list contracts) and then passed to other tools (like node summarizer or contract surface area inspector) to get detailed information about specific nodes.

*Key point*: Node IDs are scoped to their compilation unit - the same Node ID may refer to different nodes in different compilation units.

Also note that Node IDs are integers and can therefore be positive and negative

#### **Problem**

Problem refers to the user's request or query to the LLM. This could range from simple tasks like identifying specific code patterns or finding particular contract features, to complex analysis requirements such as tracing transaction flows or evaluating security criteria across multiple contracts.

#### **Callgraph**

In the context of Aderyn's MCP tools, a callgraph is defined for a deployable contract class (or) let's just say a contract, where in a nodes are function or modifiers, and edges are function calls or modifier calls. To be more precise, the callgraph only represents JUMP opcode relations. This is in contrast to outbound contract calls which corresponds to CALL / DELEGATECALL / STATICCALL opcodes which are NOT a part of the callgraph.

**Important:** The callgraph provider tool shows only a focused subgraph from one specific entrypoint function, not the complete contract callgraph.

**Example:**
If a contract has 3 public functions (entrypoints A, B, C), the complete callgraph would show all three starting points. But if you call the callgraph provider tool with entrypoint A, you only get:
- Node A and everything A can call
- You do NOT get nodes B, C or their call chains (unless A happens to call them)

This is how the callgraph was created in Aderyn as claimed in the blog post written by the author:

```blog
#### Preparing call graphs
The first step is to extract all deployable contracts in a project, that is, fully implemented, non-abstract contracts.

A separate call graph for each contract was generated. If a function node is reused across contract classes, it will show up in multiple graphs. (Example - function node defined in a parent contract class which has 2 or more children inheriting from it) The duplication is intentional and necessary, since the same code in the function node can behave differently depending on where it’s called from. This has to do with function overriding and method resolution order during contract linearization.

For each contract, start by identifying the entry points: public and external functions defined in the contract or inherited from parent contracts. These are inserted into a worklist.

We go through each function in the worklist step by step, following the chain of calls within that function as far as it goes before moving on to the next one. This helps us build the full picture of how all functions connect.

For every function in the worklist, we extract its inbound calls and resolve each one to its definition using Aderyn's router. We then draw an edge in the callgraph from  the calling function node to the target function or modifier node.

Each resolved target, whether a function or a modifier, is pushed back into the worklist for further exploration. This continues until all the function and modifier nodes reachable from the contract’s entrypoints have been visited.

Side note: A worklist is a continuously updated list that contains all the subtasks that the algorithm must complete. These can be added to or removed from the list during the execution of the algorithm itself. Typically the algorithm runs until the worklist becomes empty.
```

----

### 2. General approaches:

Based on the problem, determine a good approach to reach the desired result while keeping the user satisfied.

#### **Chain of thought**

This approach is preferred when the user's query requires a deep understanding of transaction flow and callgraphs to provide an accurate result. Typically, there can be multiple tool calls, and we get to the answer slowly.

Think deeply and make a step-by-step plan to identify the solution of the given problem. The key here is to understand all of Aderyn's MCP tools provided to you and learn to best leverage them to get accurate results. Gather all the necessary data first before making a judgment about the end result.

#### **Fast search**

This approach is for simple problems that focus on the static nature of written contract code. Prefer calling simpler tools like node finder tools or the node summarizer tools for this as there is no need to understand the whole context of surrounding code to determine the answer to these problems. An example would be - are there events that don't have any field indexed? OR What contracts have a dependency on OpenZeppelin?

#### **Hybrid search**

This approach combines both of the above approaches. Start off with chain of thought and then occasionally during the process use the fast search approach to quickly gather information about certain nodes to make decisions.

Lastly, please be careful in making plans. You don't have to stick to the above search models for all problems. Maybe for some problems, you get the answer sooner or you'll have to make a decision that varies slightly. If not sure, please follow the general approaches above, you should get to a decent state.

Note: If a user has already specified a plan of steps specified in the problem and if that plan is more robust than the one you come up with after reading this guide, and so you think the results are better off, feel free to follow the user's search method. You can always supplement the techniques mentioned below.

Note: Please note that if there are unsatisfactory results with one of the search models, give a try with the other strategy as a backup because chances are for some older versions of Solidity, the tools related to entrypoint and callgraph tracing in chain of thought approach may not work as well. Then you may have to resort to using node finders.

----

### 3. Tool call strategies:

**Scenario 1**

- *Given: Issue criteria that require understanding of the full surface area and interaction of a smart contract*
- *Goal: Find code that matches said criteria (if any)*

**Steps:**
1. First, use the project overview tool to see all the compilation units.
2. Analyses must operate on each compilation unit.
3. Start by calling the list contracts tool to enlist the contracts in a compilation unit.
4. Use the contract surface area inspection tool on the contracts of interest (stick to contracts in included files only). This reveals methods, state variables, etc. that the contract inherits from its parent contracts, third-party libraries, as well as the various library contracts that it makes internal calls to, and more. It also exposes entrypoint functions in the contract.
5. Use the callgraph provider tool to gauge the different functions and modifiers within the contract that could be called as a result of executing a given entrypoint function in a contract. Lookup the callgraph using the entrypoint function. While it is true that a callgraph is defined for a contract, the callgraph provider tool works slightly differently in the sense that it only exposes the portion of the callgraph of the contract that is reachable from the given entrypoint function.
6. Now, use the node summarizer tools to look inside these functions and modifiers. These tools will also output code snippets, so carefully use all the information gathered and evaluate it against the issue criteria. Report matches if found in included files.

#### Notes:

- The node summarizer tools provide a summary of just that node. There is no intelligence in them to perform resolutions of any kind that were not already present in the AST. All that to say, if there is an internal function call, the callgraph provider would help to resolve these to function definitions and modifier definitions. Later the node summarizer tools would help in looking inside.

- It is also noteworthy that the callgraph does not contain edges to external calls made. If there is a requirement to analyze that, then make a guess of the external contract that is being called to and re-use some of the methods described above on those contracts to gather full overview. Although, that's a very advanced use case, you might have to sometimes go through that.

- At every step see if you can eliminate the number of things to check based on the intuition developed over the years of auditing smart contracts. But don't force yourself if unsure.

**Scenario 2**

- *Given: Issue criteria that are focused on simpler static checks like basic code patterns*
- *Goal: Find code that matches said criteria (if any)*

**Steps:**
1. First, use the project overview tool to see all the compilation units.
2. Analyses must operate on each compilation unit.
3. Use the node finder tools to enlist the nodes of a certain type that are relevant to the problem at hand.
4. Now, use the node summarizer tools to look inside these nodes. These tools will also output code snippets, so carefully use all the information gathered and evaluate it against the issue criteria. Report matches if found in included files.


----
