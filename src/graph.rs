extern crate petgraph;

use std::fs::File;
use std::path::Path;

use serde_json;

#[derive(Debug, Copy, Clone)]
pub enum Edge{
    Empty,
    CrossLine,
}


#[derive(Debug, Copy, Clone)]
pub struct Neighbour{
    pub id: i32,
    pub angle: i32,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub kind: i32,
    pub name: String,
    pub neighbors: Vec<Neighbour>,
}

pub fn load_json(file_name: &str) -> (petgraph::Graph<Point, Edge, petgraph::Undirected, u32>, Vec<String>) {
    let json_file_path = Path::new(file_name);
    let json_file = File::open(json_file_path).unwrap();

    let deserialized_data: serde_json::Value =
        serde_json::from_reader(json_file).unwrap();


    // Points deserialization
    let point_names: Vec<String> = deserialized_data.get("pnames")
                                                                .unwrap()
                                                                .as_array()
                                                                .unwrap()
                                                                .iter()
                                                                .map(|x|{
                                                                    x.as_str().unwrap().to_string()
                                                                })
                                                                .collect();
    
    let point_values: &Vec<serde_json::Value> = deserialized_data.get("pvals")
                                                                 .unwrap()
                                                                 .as_array()
                                                                 .unwrap();
    let json_edges: &Vec<serde_json::Value> = deserialized_data.get("edges")
                                                                 .unwrap()
                                                                 .as_array()
                                                                 .unwrap();

    let mut graph = petgraph::graph::Graph::<Point, Edge, petgraph::Undirected, u16>
        ::with_capacity(point_names.len(), json_edges.len());

    let mut graph = petgraph::graph::Graph::new_undirected();

    for a in point_names.iter().zip(point_values.iter()) {
        let (name, data) = a;

        let neighbors_json = data.get("neighbors").unwrap().as_array().unwrap();
        let mut neighbors_ready = Vec::with_capacity(neighbors_json.len());

        for a in neighbors_json.iter() {
            neighbors_ready.push(
                Neighbour{
                    id: a.get("id").unwrap().as_i64().unwrap() as i32,
                    angle: a.get("ang").unwrap().as_i64().unwrap() as i32,
                         })
        }

        let point = Point{
            x: data.get("x").unwrap().as_i64().unwrap() as i32,
            y: data.get("y").unwrap().as_i64().unwrap() as i32,
            kind: data.get("kind").unwrap().as_i64().unwrap() as i32,
            name: name.to_string(),
            neighbors: neighbors_ready,};
        graph.add_node(point);
    }

    for edge in json_edges.iter() {
        graph.add_edge(
        petgraph::graph::NodeIndex::new(edge.get("0").unwrap().as_i64().unwrap() as usize),
        petgraph::graph::NodeIndex::new(edge.get("1").unwrap().as_i64().unwrap() as usize),
        Edge::CrossLine,
        );
    }

    return (graph, point_names)
}

fn find_path(start: i32, finish: i32, 
             graph: &petgraph::Graph::<Point, Edge, petgraph::Undirected, u16>) {
    let path = petgraph::algo::astar(
        &graph,
        petgraph::graph::NodeIndex::new(start as usize),
        |node| { node.index() == finish as usize},
        |_edge| { 1 },
        |_| { 0 },
                          ).unwrap();
    println!("{:?}", path);
}

fn main() {
    println!("Hello world");
}
