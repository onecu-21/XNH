use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use xnh_core::NodeId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterValue {
    Number(f64),
    Text(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    pub parameters: BTreeMap<String, ParameterValue>,
    pub metadata: BTreeMap<String, String>,
    pub dependencies: Vec<NodeId>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DesignGraph {
    pub nodes: Vec<Node>,
}

impl DesignGraph {
    pub const fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_and_serializes_a_node() {
        let node = Node {
            id: NodeId::new(),
            name: "example box".to_owned(),
            parameters: BTreeMap::new(),
            metadata: BTreeMap::new(),
            dependencies: Vec::new(),
        };
        let id = node.id;
        let mut graph = DesignGraph::new();
        graph.add_node(node);

        assert_eq!(
            graph.node(id).map(|node| node.name.as_str()),
            Some("example box")
        );
        let json = serde_json::to_value(graph).expect("graph should serialize");
        assert_eq!(json["nodes"][0]["name"], "example box");
        assert!(json["nodes"][0].get("geometry").is_none());
    }
}
