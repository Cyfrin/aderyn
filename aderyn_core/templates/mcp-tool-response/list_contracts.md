## List of Contracts in Compilation Unit {{ compilation_unit_index }}

The following is the list of names of deployable contracts and their corresponding Node IDs and filepaths.

{% for c in contracts_info %}
- {{ c.name }} | {{ c.filepath }} | Node ID: {{ c.node_id }}
{% endfor %}

*Suggestion for next steps:* Use these Node IDs with the contract surface area inspection tool to analyze specific contracts.

**Note:** All contracts listed above belong to compilation unit {{ compilation_unit_index }}. If you see duplicate contract names in this list, they are actually different contracts that happen to share the same name but exist in different files, making each one unique within the compilation unit.
