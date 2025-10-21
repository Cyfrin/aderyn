## Node IDs and compilation unit indexes for matching nodes.

### Regex input: {{ term }}

{% if nodes.len() > 0 %}
{% for (node_type, inodes) in nodes %}
{% if inodes.len() > 0 %}
### Found in implementation code of following {{ node_type }}s
{% for node_info in inodes %}
- Name: {{ node_info.name }} | NodeID: {{ node_info.node_id }} | CompilationUnitIndex: {{ node_info.compilation_unit_index }}
{% endfor %}
{% endif %}
{% endfor %}
### Tip: Now, use the node summarizer tool to investigate these nodes
{% else %}
### No match found for regex {{ term }} !
{% endif %}
