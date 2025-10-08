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
In this compilation unit {{ compilation_unit_index }}, Node ID {{ node_id }} appears in the callgraphs of the following entrypoints:
{% for c in containing_callgraphs %}
- Deployable contract `{{ c.deployable_contract_name }}` (node ID: {{ c.deployable_contract_id }}) entrypoints:
{% for e in c.entrypoint_ids %}
    - Entrypoint Node ID {{ e }}
{% endfor %}

{% endfor %}

You can make use of Aderyn's callgraph tool to further holistically explore the bigger context in which this function is a part of.
{% endif %}
