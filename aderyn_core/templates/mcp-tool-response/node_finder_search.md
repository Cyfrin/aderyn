## Node IDs and compilation unit indexes of matching {{ node_type }}s.

### Search input: {{ term }}

{% if matching_nodes.len() > 0 %}
### Matching {{ node_type }}s

{% for node_info in matching_nodes %}
- Name: {{ node_info.name }} | NodeID: {{ node_info.node_id }} | CompilationUnitIndex: {{ node_info.compilation_unit_index }}
{% endfor %}
{% else %}
No match found for {{ node_type }}s !
{% endif %}

### Tip: Now, use the node summarizer tool to investigate these nodes
