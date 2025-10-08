## Contract Surface Inspector for {{ name }}

*Contract name:* {{ name }}

*Contract ID:* {{ node_id }}

*Compilation Unit index:* {{ compilation_unit_index }}

*Filepath:* {{ filepath }}

### C3 Linearized Inheritance

After performing C3 linearization on the main contract {{ name }}, the following chain is the end result. It goes from the most base parent class at the beginning to the most derived class at the end of the list chain.

The last contract class in the chain is {{ name }} because it is the most derived contract class in its hierarchy.

List of names of contract class in the inheritance chain of {{ name }} along with their filepath.

**Total contracts in inheritance chain:** {{ reversed_chain.len() }}

{% for c in reversed_chain %}
{{ loop.index }}. {{ c.name }} | Node ID: {{ c.node_id }} | Filepath: {{ c.filepath }}
{% endfor %}

### State variables

State variables of a contract include all variables defined in the contract itself plus those inherited from parent classes.

{% if total_state_variables == 0 %}
*No state variables found*
{% else %}
The following code snippets show all state variables defined in each contract class within the inheritance chain.

{% for c in reversed_chain %}
{{ loop.index }}. {{ c.name }}

{% if c.state_variables.len() == 0 %}
No state variables found defined in {{ c.name }}
{% else %}
```solidity
{% for s in c.state_variables %}
{{ s }}
{% endfor %}
```
{% endif %}

{% endfor %}

{% endif %}

### Entrypoint functions

In Solidity, entrypoint functions are functions that can be called from outside the contract or serve as initial execution points. The main types of entrypoint functions are external functions, public functions, fallback functions, and receive functions.

The following is a list of entrypoint functions for {{ name }} with their corresponding Node IDs and the contract class in the inheritance chain where each function is defined.

If a function has been overridden, it won't be listed in here. This list contains all the functions after filtering out the ones that are actually called.

#### External Functions

{% if entrypoints.external_functions.len() == 0 %}
*No external functions found*
{% else %}
{% for func in entrypoints.external_functions %}
{{ loop.index }}. **{{ func.name }}** | Function's Node Id: {{ func.node_id }} | Containing contract class: {{ func.containing_contract.name }} | Containing contract's Node Id: {{ func.containing_contract.node_id }}
{% endfor %}
{% endif %}

#### Public Functions

{% if entrypoints.public_functions.len() == 0 %}
*No public functions found*
{% else %}
{% for func in entrypoints.public_functions %}
{{ loop.index }}. **{{ func.name }}** | Function's Node Id: {{ func.node_id }} | Containing contract class: {{ func.containing_contract.name }} | Containing contract's Node Id: {{ func.containing_contract.node_id }}
{% endfor %}
{% endif %}

#### Fallback Function

{% if let Some(fallback_function) = entrypoints.fallback_function %}
Node Id: {{ fallback_function.node_id }} | Containing contract class: {{ fallback_function.containing_contract.name }} | Containing contract's Node Id: {{ fallback_function.containing_contract.node_id }}
{% else %}
*No fallback function found.*
{% endif %}

#### Receive Function

{% if let Some(receive_function) = entrypoints.receive_function %}
Node Id: {{ receive_function.node_id }} | Containing contract class: {{ receive_function.containing_contract.name }} | Containing contract's Node Id: {{ receive_function.containing_contract.node_id }}
{% else %}
*No receive function found.*
{% endif %}

### Suggestion for next steps:

Try to explore callgraphs starting from a given entrypoint function. To do that, use the callgraph provider tool and pass the entrypoint function's Node ID, and the Node ID of the original deployable contract (same one used to call this contract surface tool). NOT just the containing contract's Node ID.

A good analysis explores the project on a per callgraph basis for entrypoints of interest. This provides a holistic picture and thefore less false positives.
