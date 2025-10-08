## List of Node IDs and compilation unit indexes for all {{ node_type }}

{% if nodes.len() > 0 %}
{% for n in nodes %}
- Name: {{ n.name }} | NodeID: {{ n.node_id }} | CompilationUnitIndex: {{ n.compilation_unit_index }}
{% endfor %}
{% else %}
No {{ node_type }} nodes found !
{% endif %}

### Tip: Now, use the node summarizer tool to investigate these nodes
