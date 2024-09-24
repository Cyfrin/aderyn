use crate::context::workspace_context::WorkspaceContext;

use super::WorkspaceCallGraph;

// Define a JSON structure that represents a call graph
//
// {
//
//  ID -> Metadata
//
//  "contracts" : {
//      "0": {
//          "name": "C1",
//          "state_var_ids" : [
//              "12", "11", "10"
//          ],
//          "modifier_ids": [
//              "6", "7", "8"
//          ],
//          "func_ids": [
//              "1", "2", "3"
//          ],
//       },
//      "1": {
//          ...
//      }
//  },
//  "state_variable_ids": {
//      "12": {
//          name: "s_count",
//          visibility: "..."
//      },
//      ...
//  },
//  "functions": {
//      "1" : {
//          name: "_func1",
//          visibility: "internal" | "public" | "external" | "external"
//      },
//      ...
//  }
//  "modifiers": {
//      "1" : {
//          name: "_modif1",
//          visibility: "internal" | "public" | "external" | "external"
//      },
//      ...
//  },
//
//  // adjacency list representation of callgraph
//  // every node id in raw_callgraph represents a modifier or a function
//
//  "raw_callgraph" : {
//      "1": ["2", "3"],
//      "2": ["1", "5"],
//      ...
//  }
//
//
// }

impl WorkspaceCallGraph {
    fn serialize(context: &WorkspaceContext) {}
}
