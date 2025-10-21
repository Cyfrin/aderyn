## Node IDs and compilation unit indexes for matching nodes.

### Regex input: {{ term }}

{% if nodes.len() > 0 %}
{% for (node_type, nodes) in nodes %}
### Matching {{ node_type }}s
{% for node_info in nodes %}
- Name: {{ node_info.name }} | NodeID: {{ node_info.node_id }} | CompilationUnitIndex: {{ node_info.compilation_unit_index }}
{% endfor %}
{% endfor %}
{% else %}
No match found for regex {{ term }}s !
{% endif %}

### Tip: Now, use the node summarizer tool to investigate these nodes
