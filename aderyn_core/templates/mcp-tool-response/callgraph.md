## Callgraph for {{ contract.name }} contract when traversed from {{ entrypoint_function.name }}

*Compilation Unit index:* {{ compilation_unit_index }}

*Contract:* {{ contract.name }} | Node ID: {{ contract.node_id }}

*Entrypoint function:* {{ entrypoint_function.name }} | Node ID: {{ entrypoint_function.node_id }}

Below is a adjacency list representation of the subgraph of {{ contract.name }}'s callgraph. It covers all function or modifiers nodes reachable when we start traversal from the single entrypoint function `{{ entrypoint_function.name }}`   with Node ID: {{ entrypoint_function.node_id }}

**Note:** This represents only the portion of {{ contract.name }}'s complete callgraph that is reachable from this specific entrypoint. Other public/external functions in the contract and their call chains are not included unless they are called by `{{ entrypoint_function.name }}`.

Every number in the adjacency list is a Node ID of a function or a modifier node.
For example, if there is an edge from `A->B` (i.e A calls B) and `A->C` (A calls C), it would be represented as *A -> B, C*

### Adjacency List Graph of NodeIDs

{% for (from, to_list) in &graph %}
- {{ from }} -> {% for (i, v) in to_list.iter().enumerate() %} {{ v }}{% if i != to_list.len() %},{% endif %} {% endfor %}
{% endfor %}

### Reverse Post Order of nodes in the above graph and their corresponding Node IDs

{% for node in post_order_nodes.iter().rev() %}
**{{ node.name }} (node.node_id)** calls:
{% for called_node in node.called_nodes %}
  - {{ called_node.name }} ({{ called_node.node_id }})
{% endfor %}
{% endfor %}

### Next steps:
It maybe helpful to run the node summarizer tool on function nodes or a modifier nodes of interest from the above output. You only have to pass in the compilation unit index and the node ID which is mentioned next to that node.
