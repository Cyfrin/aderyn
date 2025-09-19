## Node summarizer

### Compilation Unit: {{ compilation_unit_index }}
### Node ID: {{ node_id }}
### Filepath: {{ filepath }}

{% if let Some(contract) = containing_contract %}
### Containing Contract Class: {{ contract.name }} | Node Id: {{ contract.node_id }}
{% endif %}

{% if let Some(function) = containing_function %}
### Containing Function: {{ function.name }} | Node Id: {{ function.node_id }}
{% endif %}

{% if let Some(modifier) = containing_modifier %}
### Containing Modifier: {{ modifier.name }} | Node Id: {{ modifier.node_id }}
{% endif %}

### Code snippet of the node:
```solidity
{{ code }}
```
