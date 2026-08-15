use std::collections::BTreeMap;

use serde::Serialize;
use xnh_core::{Millimeters, NodeId};
use xnh_geometry::{BoxGeometry, Geometry};
use xnh_graph::{DesignGraph, Node};

#[derive(Serialize)]
struct ExampleDesign {
    design_graph: DesignGraph,
    geometry: Geometry,
}

fn main() -> Result<(), serde_json::Error> {
    let geometry = Geometry::Box(BoxGeometry {
        width: Millimeters::new(120.0),
        depth: Millimeters::new(80.0),
        height: Millimeters::new(20.0),
    });

    let metadata = BTreeMap::from([("description".to_owned(), "120 x 80 x 20 mm box".to_owned())]);

    let node = Node {
        id: NodeId::new(),
        name: "example_box".to_owned(),
        parameters: BTreeMap::new(),
        metadata,
        dependencies: Vec::new(),
    };

    let mut graph = DesignGraph::new();
    graph.add_node(node);

    let example = ExampleDesign {
        design_graph: graph,
        geometry,
    };

    println!("{}", serde_json::to_string_pretty(&example)?);
    Ok(())
}
