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


#[derive(Debug, Copy, Clone)]
pub enum LineRide {
    CrossStop,
    LeftStop,
    RightStop,
}

#[derive(Debug, Copy, Clone)]
pub enum MoveAction {
    LineRide(LineRide),
    Rotate(i32),
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

    let mut graph = petgraph::graph::Graph::<Point, Edge, petgraph::Undirected, u32>
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
             graph: &petgraph::Graph::<Point, Edge, petgraph::Undirected, u32>) {
    let path = petgraph::algo::astar(
        &graph,
        petgraph::graph::NodeIndex::new(start as usize),
        |node| { node.index() == finish as usize},
        |_edge| { 1 },
        |_| { 0 },
                          ).unwrap();
    println!("{:?}", path);
}


pub fn goto_point(
    graph: &(petgraph::Graph::<Point, Edge, petgraph::Undirected, u32>,
            Vec<String>),
    start_name: String,
    finish_name: String,
    current_ang: i32) -> (Vec<MoveAction>, i32) {
    let (graph, point_names) = graph;
    //println!("{:?}", point_names);

    let sp_i = point_names.iter().position(|r| {*r == start_name}).unwrap();
    let fp_i = point_names.iter().position(|r| {*r == finish_name}).unwrap();

    let start_point = petgraph::graph::NodeIndex::<u32>::new(sp_i as usize);
    let finish_point = petgraph::graph::NodeIndex::<u32>::new(fp_i as usize);
    println!("S:{:?}, F:{:?}", start_point, finish_point);

    let (cost, path) = petgraph::algo::astar(
        &graph,
        start_point,
        |node| { node.index() == finish_point.index()},
        |_edge| { 1 },
        |_est_cost| { 0 },
                          ).unwrap();

    let mut current_point = graph.node_weight(start_point).unwrap();
    let mut current_ang: i32 = current_ang;
    println!("{:#?}", path);
    let mut action_array: Vec<MoveAction> = Vec::with_capacity(path.len());

    // Skip start point
    for next_point in path.iter().skip(1) {
        let n_id = next_point.index() as i32;
        let n_val = graph.node_weight(*next_point).unwrap();
        let cur_neighbors = &current_point.neighbors;
        let next_neighbors = &n_val.neighbors;

        // Rotation calculation
        let target_ang = cur_neighbors.iter().find(|&r|{ r.id == next_point.index() as i32 }).unwrap().angle;

        let diff = target_ang - current_ang;
        let diff = if diff > 180 { diff - 360 }
            else if diff < -180 { diff + 360 }
            else { diff };


        let start_angle = current_ang;
        let finish_angle = target_ang;

        let mut inside_edges = 0;
        let mut outside_edges = 0;

        let mut is_inside = false;
        let mut is_first_start = true;
        let mut is_passed_start = false;

        println!("st_ang: {}", start_angle);
        for edge in cur_neighbors.iter() {
            println!("edge_ang: {}", edge.angle);
            if edge.angle == start_angle {
                if is_inside {
                    is_inside = false;
                    println!("FIRST START FAAALSE");
                    is_first_start = false;
                } else {
                    is_inside = true;
                    is_first_start = true;
                }
                is_passed_start = true;
                
                continue;
            }

            if (edge.angle > start_angle) && !is_passed_start{
                if is_inside {
                    is_inside = false;
                    is_first_start = false;
                } else {
                    is_inside = true;
                    is_first_start = true;
                }
                is_passed_start = true;
            }

            if (edge.angle == finish_angle) {
                if is_inside {
                    is_inside = false;
                } else {
                    is_inside = true;
                }
                
                continue;
            }

            if is_inside {
                inside_edges += 1;
            } else {
                outside_edges += 1;
            }
        }

        if !is_passed_start {
            is_first_start = false;
        }

        println!("{}, {}", diff, is_first_start);
        println!("I:{}, O:{}", inside_edges, outside_edges);
        if (diff > 0) && (is_first_start) {
            //turn right inside times
            action_array.push(MoveAction::Rotate(inside_edges + 1));
            println!("Turn right inside times");
        } else if (diff > 0) && !(is_first_start) {
            //turn right outside times
            action_array.push(MoveAction::Rotate(outside_edges + 1));
            println!("Turn right outside times");
        } else if (diff < 0) && (is_first_start) {
            //turn left outside times
            action_array.push(MoveAction::Rotate(-(outside_edges + 1)));
            println!("Turn left outside times");
        } else if (diff < 0) && !(is_first_start) {
            //turn left inside times
            action_array.push(MoveAction::Rotate(-(inside_edges + 1)));
            println!("Turn left inside times");
        }


       let is_something_inside = |start: i32, finish: i32| -> bool {
            let is_inside = false;
            if start < finish {
                for edge in next_neighbors {
                    if (edge.angle > start) && (edge.angle < finish){ return true }
                }
            } else {
                for edge in next_neighbors {
                    if ((edge.angle > start) && (edge.angle < 360)) ||
                        ((edge.angle < finish) && (edge.angle >= 0)) { return true }
                }
                println!("shit");
            }
            return false
        };


       fn normalize_ang(ang: i32) -> i32 {
           if ang > 360 {
               return ang % 360;
           }

           if ang < 0 {
                return 360 + ang;
           }
           if ang < -360 {
               return ((ang % 360) + 360) % 360;
           }
           return ang;
       }


       let left_start = normalize_ang(target_ang - 110);
       let left_finish = normalize_ang(target_ang - 70);

       let right_start = normalize_ang(target_ang + 70);
       let right_finish = normalize_ang(target_ang + 110);
       println!("{}, {}: aaa", right_start, right_finish);

       println!("right_start");
       let is_left_line = is_something_inside(left_start, left_finish);
       let is_right_line = is_something_inside(right_start, right_finish);
       println!("right_finish");

       println!("{}, {}", is_left_line, is_right_line);
       println!("{:?}", current_point.name);
       println!("{:?}", target_ang);
       println!("===================");

       if is_left_line && is_right_line {
           action_array.push(MoveAction::LineRide(LineRide::CrossStop));
       } else if is_left_line {
           action_array.push(MoveAction::LineRide(LineRide::LeftStop));
       } else if is_right_line {
           action_array.push(MoveAction::LineRide(LineRide::RightStop));
       } else {
           //panic!("can't stop nowhere.... ")
       }

       
       current_point = n_val;
       current_ang = target_ang;
    }

    return (action_array, current_ang);
}
