#[cfg(test)]
pub mod control_flow_tests {

    use std::{collections::HashMap, fmt::Debug, path::Path, process::Command};

    use crate::context::{flow::Cfg, workspace::WorkspaceContext};
    use petgraph::{dot::Dot, prelude::Graph};

    struct CustomString {
        string: String,
    }

    impl Debug for CustomString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "{}", self.string)
        }
    }

    impl From<String> for CustomString {
        fn from(value: String) -> Self {
            Self { string: value }
        }
    }

    pub fn output_graph(context: &WorkspaceContext, cfg: &Cfg, output: &str) {
        let dot_file_path = format!("../tests/contract-playground/dot/{}.dot", output);
        let svg_file_path = format!("../tests/contract-playground/dot/{}.svg", output);

        let mut graph: Graph<CustomString, CustomString> = Graph::new();
        let mut node_indices = HashMap::new();

        for (node, value) in cfg.nodes.clone() {
            let node_index = graph.add_node(value.nd.display(context).into());
            node_indices.insert(node, node_index);
        }

        for (from, to_list) in &cfg.adj_list {
            let f = node_indices.get(from).unwrap();
            for to in to_list {
                let t = node_indices.get(to).unwrap();
                graph.add_edge(*f, *t, format!(" {}-->{}", from.peek(), to.peek()).into());
            }
        }

        let dot = Dot::new(&graph);
        _ = std::fs::write(Path::new(&dot_file_path), format!("{:?}", dot));

        let mut cmd = Command::new("dot");
        cmd.args(["-Tsvg", &dot_file_path]);
        if let Ok(raw_output) = cmd.output() {
            if let Ok(output_svg) = String::from_utf8(raw_output.stdout) {
                _ = std::fs::write(Path::new(&svg_file_path), output_svg);
            }
        }
    }
}
