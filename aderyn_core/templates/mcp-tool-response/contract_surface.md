## Contract Surface Inspector for {{ name }}

*Contract name:* {{ name }}

*Compilation Unit index:* {{ compilation_unit_index }}

*Filepath:* {{ filepath }} {% if included %} INCLUDED {% endif %}

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

```solidity
{% for s in c.state_variables %}
{{ s }}
{% endfor %}
```
{% endfor %}

{% endif %}

### Entrypoint functions

In Solidity, entrypoint functions are functions that can be called from outside the contract or serve as initial execution points. The main types of entrypoint functions are external functions, public functions, fallback functions, and receive functions.

The following is a list of entrypoint functions for {{ name }} with their corresponding Node IDs and the contract class in the inheritance chain where each function is defined.

If a function has been overridden, it won't be listed in here. This list contains all the functions after filtering out the ones that are actually called.

#### External Functions

{% if external_functions.len() == 0 %}
*No external functions found*
{% else %}
{% for func in external_functions %}
{{ loop.index }}. **{{ func.name }}** | Node Id: {{ func.node_id }} | Contract class: {{ func.containing_contract }}
{% endfor %}
{% endif %}

#### Public Functions

{% if public_functions.len() == 0 %}
*No public functions found*
{% else %}
{% for func in public_functions %}
{{ loop.index }}. **{{ func.name }}** | Node Id: {{ func.node_id }} | Contract class: {{ func.containing_contract }}
{% endfor %}
{% endif %}

#### Fallback Function

{% if let Some(fallback_function) = fallback_function %}
Node Id: {{ fallback_function.node_id }} | Contract class: {{ fallback_function.containing_contract }}
{% else %}
*No fallback function found.*
{% endif %}

#### Receive Function

{% if let Some(receive_function) = receive_function %}
Node Id: {{ receive_function.node_id }} | Contract class: {{ receive_function.containing_contract }}
{% else %}
*No receive function found.*
{% endif %}
