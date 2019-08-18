#![ allow( dead_code, unused ) ]
extern crate rlua;

use std::env;
use std::fs;
use std::thread;
use rlua::{Lua, Context, Debug, Result, HookTriggers};
mod graph;
mod line;

const DEFAULT_FILENAME: &str = "script.lua";
const ENV_FILENAME_VAR: &str = "LUA_FILENAME";

fn get_script() -> String {
    let script_name = match env::var(ENV_FILENAME_VAR) {
        Ok(val) => val,
        Err(_e) => DEFAULT_FILENAME.to_string(),
    };

    return fs::read_to_string(script_name)
        .expect("Error reading script file");
}

fn lua_hook(_c: Context, d: Debug) -> Result<()> {
    println!("Line №{}", d.curr_line());
    Ok(())
}

fn rust_function(_c: Context, (name, age): (String, u8)) -> Result<()> {
    println!("{} {}", name, age);
    Ok(())
}

fn mane() {
    let lua = Lua::new();
    let hook_triggers = HookTriggers {every_line: true, ..Default::default()};
    lua.set_hook(hook_triggers, lua_hook);
    
    let script = get_script();
    lua.context(move |lua_ctx|{
        let mut motor_pair = line::MotorPair::new();
        let mut sensor_pair = line::SensorPair::new();
        
        let mut cross_line = move |_c, (p, i, d, speed):(f32, f32, f32, i32)| {
            line::ride_line_cross(p, i, d, speed, 0, &mut motor_pair, &mut sensor_pair);
            Ok(())
        };

        let set_middle_grey= |_c, val: i32| {
            line::set_middle_grey(val);
            Ok(())
        };

        // Creating lua functions
        let lua_set_middle_grey= lua_ctx.create_function(set_middle_grey)
            .expect("Function creation failed");
        let lua_cross_line = lua_ctx.create_function_mut(cross_line)
            .expect("Function creation failed");

        // Assigning lua functions
        lua_ctx.globals().set("set_middle_grey", lua_set_middle_grey)
            .expect("Function to var in lua failed");
        lua_ctx.globals().set("ride_line_cross", lua_cross_line)
            .expect("Function to var in lua failed");

        // Executing lua script
        lua_ctx
            .load(&script)
            .exec()
            .expect("Script execution failed");
    });
}

fn main() {
    let kp: f32 = 1.0;
    let ki: f32 = 0.0;
    let kd: f32 = 0.0;
    let speed: i32 = 40;
    let se: i32 = 0;

    line::set_middle_grey(30);
    let mut motor_pair = line::MotorPair::new();
    let mut sensor_pair = line::SensorPair::new();


    let (mut graph, point_names) = graph::load_json("points.json");
    //println!("{:?}", point_names);

    let sp_i = point_names.iter().position(|r| {*r == "START"}).unwrap();
    let fp_i = point_names.iter().position(|r| {*r == "FINISH"}).unwrap();

    let start_point = petgraph::graph::NodeIndex::<u32>::new(sp_i as usize);
    let finish_point = petgraph::graph::NodeIndex::<u32>::new(fp_i as usize);
    //println!("S:{:?}, F:{:?}", start_point, finish_point);

    let (cost, path) = petgraph::algo::astar(
        &graph,
        start_point,
        |node| { node.index() == finish_point.index()},
        |_edge| { 1 },
        |_est_cost| { 0 },
                          ).unwrap();

    let mut current_point = graph.node_weight(start_point).unwrap();
    let mut current_ang: i32 = 180;
    //println!("{:#?}", path);

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

        for edge in cur_neighbors.iter() {
            if edge.angle == start_angle {
                if is_inside {
                    is_inside = false;
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

        println!("{}, {}", diff, is_first_start);
        if (diff > 0) && (is_first_start) {
            //turn right inside times
            line::turn_right_count(inside_edges + 1,
                                   &mut motor_pair, &mut sensor_pair);
            println!("Turn right {} + 1", inside_edges);

        } else if (diff > 0) && !(is_first_start) {
            //turn right outside times
            line::turn_right_count(outside_edges + 1,
                                   &mut motor_pair, &mut sensor_pair);
            println!("Turn right {} + 1", outside_edges);

        } else if (diff < 0) && (is_first_start) {
            //turn left outside times
            line::turn_left_count(outside_edges + 1,
                                  &mut motor_pair, &mut sensor_pair);
            println!("Turn left {} + 1", outside_edges);

        } else if (diff < 0) && !(is_first_start) {
            //turn left inside times
            line::turn_left_count(inside_edges + 1,
                                  &mut motor_pair, &mut sensor_pair);
            println!("Turn left {} + 1", inside_edges);

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
                        ((edge.angle < finish) && (edge.angle > 0)) { return true }
                }
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

       let is_left_line = is_something_inside(left_start, left_finish);
       let is_right_line = is_something_inside(right_start, right_finish);

       if is_left_line && is_right_line {
           line::ride_line_cross(kp, ki, kd, speed, se, 
                                 &mut motor_pair, &mut sensor_pair)
       } else if is_left_line {
           line::ride_line_left_stop(kp, ki, kd, speed, se, 
                                 &mut motor_pair, &mut sensor_pair)
       } else if is_right_line {
           line::ride_line_right_stop(kp, ki, kd, speed, se, 
                                 &mut motor_pair, &mut sensor_pair)
       } else {
           panic!("can't stop nowhere.... ")
       }

       println!("{}, {}", is_left_line, is_right_line);
       println!("{:?}", current_point.name);
       println!("{:?}", target_ang);
       
       println!("===================");
       current_point = n_val;
       current_ang = target_ang;
    }
}

fn maine() {
    line::set_middle_grey(50);
    let mut motor_pair = line::MotorPair::new();
    let mut sensor_pair = line::SensorPair::new();
    line::turn_right_on_line(&mut motor_pair, &mut sensor_pair);
}
