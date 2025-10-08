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

### Metadata
{% if containing_callgraphs.len() == 0 %}
No metadata present.
{% else %}
Node ID {{ node_id }} is part of callgraphs starting from:
{% for c in containing_callgraphs %}
- Deployable contract ID {{ c.deployable_contract_id }} with entrypoint function node ID {{ c.entrypoint_function_id }} in compilation unit index {{ c.compilation_unit_index }} .
{% endfor %}
{% endif %}
