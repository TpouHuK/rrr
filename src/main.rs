#![ allow( dead_code, unused ) ]
extern crate rlua;

use std::env;
use std::fs;
use std::thread;
use std::sync::{Mutex, Condvar, Arc};
use std::time;
use rlua::{Lua, Context, Debug, Result, HookTriggers};

mod graph;
mod line;
mod joystick;

const DEFAULT_FILENAME: &str = "script.lua";
const ENV_FILENAME_VAR: &str = "LUA_FILENAME";
const DEGREES: i32 = 20;
const MACRO_SPEED: i32 = 20;

fn get_script() -> String {
    let script_name = match env::var(ENV_FILENAME_VAR) {
        Ok(val) => val,
        Err(_e) => DEFAULT_FILENAME.to_string(),
    };

    return fs::read_to_string(script_name)
        .expect("Error reading script file");
}

fn lua_hook(_c: Context, d: Debug) -> Result<()> {
    //println!("Line №{}", d.curr_line());
    Ok(())
}

use std::cmp;

fn maxf32(a: f32, b: f32) -> f32 {
    if a > b { return a } else {return b}
}
fn minf32(a: f32, b: f32) -> f32 {
    if a < b { return a } else {return b}
}

enum TypeOfMove {
    GotoPoint(Vec<graph::MoveAction>),
    Macro(Vec<i32>),
    UnMacro(Vec<i32>),
    LineDegrees((i32)),
    RightOutLine,
}

fn main() {
    println!("Launched");
    let is_goto_running = Arc::new((Mutex::new(false), Condvar::new()));
    let is_goto_running_c = is_goto_running.clone();
    let is_goto_running_c2 = is_goto_running.clone();
    let is_goto_running_c3 = is_goto_running.clone();
    let is_goto_running_c4 = is_goto_running.clone();
    let is_goto_running_c5 = is_goto_running.clone();
    let is_goto_running_c6 = is_goto_running.clone();
    let is_goto_running_c7 = is_goto_running.clone();

    let (send_ch, receive_ch) = std::sync::mpsc::channel();
    let send_ch2 = send_ch.clone();
    let send_ch3 = send_ch.clone();
    let send_ch4 = send_ch.clone();
    let send_ch5 = send_ch.clone();
    let send_ch6 = send_ch.clone();

    let mut kpid = Arc::new(Mutex::new((1.0, 0.0, 0.0)));
    let mut kpid_c = kpid.clone();

    let mut klspeed = Arc::new(Mutex::new(40 as i32));
    let mut klspeed_c = klspeed.clone();
    let mut klspeed_c2 = klspeed.clone();

    let mut krspeed = Arc::new(Mutex::new(40 as i32));
    let mut krspeed_c = krspeed.clone();
    let mut cs = line::ControlSensor::new();

    println!("Stuff inited");
    let goto_point_thread = thread::spawn(move|| {
        let mut robot = line::RobotMoveBase::new();
        let (mutex, condvar) = &*is_goto_running_c;

        loop {
            let moveact = match receive_ch.recv() {
               Ok(val) => val,
               Err(e) => return,
            };

            match moveact {
                TypeOfMove::GotoPoint(path) => {
            let mut pid;
            let mut lspeed;
            let mut rspeed;
            {pid = *kpid.lock().unwrap()};
            {lspeed = *klspeed.lock().unwrap()};
            {rspeed = *krspeed.lock().unwrap()};

            for elem in path {
                match elem {
                    graph::MoveAction::LineRide(lineride) => {
                        match lineride {
                            graph::LineRide::CrossStop => {
                                line::ride_line_cross(pid, lspeed, &mut robot);
                            },
                            graph::LineRide::LeftStop=> {
                                line::ride_line_left_stop(pid, lspeed, &mut robot);
                            },
                            graph::LineRide::RightStop => {
                                line::ride_line_right_stop(pid, lspeed, &mut robot);
                            },
                        }
                        robot.motor_pair.go_on_degrees(lspeed, 70);
                    },
                    graph::MoveAction::Rotate(count) => {
                        line::turn_count(&mut robot, count, rspeed);
                    }
                }
            //robot.motor_pair.set_steering(0, 0);
            //thread::sleep(time::Duration::from_secs(2));
            }
            robot.motor_pair.set_steering(0, 0);
            },
                TypeOfMove::Macro(steerings) => {
                    for steering in steerings.iter() {
                        robot.motor_pair.steer_on_degrees(*steering, MACRO_SPEED, DEGREES);
                    }
                    robot.motor_pair.set_steering(0, 0);
                },
                TypeOfMove::UnMacro(steerings) => {
                    for steering in steerings.iter() {
                        robot.motor_pair.steer_on_degrees(*steering, -MACRO_SPEED, DEGREES);
                    }
                    robot.motor_pair.set_steering(0, 0);
                },
                TypeOfMove::LineDegrees(degrees) => {
                    let mut pid;
                    let mut lspeed;
                    {pid = *kpid.lock().unwrap()};
                    {lspeed = *klspeed.lock().unwrap()};
                    line::ride_line_degrees(pid, lspeed, &mut robot, degrees);
                    robot.motor_pair.set_steering(0, 0);
                },
                TypeOfMove::RightOutLine => {
                    let mut pid;
                    let mut lspeed;
                    {pid = *kpid.lock().unwrap()};
                    {lspeed = *klspeed.lock().unwrap()};
                    line::ride_outer_line_left_stop(pid, lspeed, &mut robot);
                    robot.motor_pair.set_steering(0, 0);
                },
            }

            {let mut running = mutex.lock().unwrap();
                *running = false;
                condvar.notify_all();}
        }
    });


    println!("Near graph");
    //let loaded_graph = Arc::new(Mutex::new(graph::load_json("points.json")));
    let loaded_graph = Arc::new(graph::load_json("points.json"));
    let loaded_graph2 = loaded_graph.clone();
    println!("After graph");
    let lua = Lua::new();
    let mut lift_motor = line::Lift::new();
    let mut rotate_motor = line::Rotate::new();

    let hook_triggers = HookTriggers {every_line: true, ..Default::default()};

    lua.set_hook(hook_triggers, lua_hook);
    
    let script = get_script();
    println!("Befored context");
    lua.context(move |lua_ctx|{
        let set_lspeed = move |_c, ispeed: i32|{
            let mutex = &*klspeed_c;
            let mut speed = mutex.lock().unwrap();
            *speed = ispeed;
            Ok(())
        };

        let set_rspeed = move |_c, ispeed: i32|{
            let mutex = &*krspeed_c;
            let mut speed = mutex.lock().unwrap();
            *speed = ispeed;
            Ok(())
        };

        let set_pid = move |_c: Context, (kp, ki, kd):(f32, f32, f32)|{
            let mutex = &*kpid_c;
            let mut tuple = mutex.lock().unwrap();
            *tuple = (kp, ki, kd);
            Ok(())
        };

        let set_middle_grey = |_c, val: i32| {
            line::set_middle_grey(val);
            Ok(())
        };

        let set_black = |_c, val: i32| {
            line::set_black(val);
            Ok(())
        };

        let set_white = |_c, val: i32| {
            line::set_white(val);
            Ok(())
        };

        let wait_till_arrival = move|_c, _:()| {
            let (mutex, cvar) = &*is_goto_running;
            let mut running = mutex.lock().unwrap();
            while *running {
                running = cvar.wait(running).unwrap();
            }
            Ok(())
        };

        //let is_arrived |_c, _:()| {
            //is_goto_running_c3();
            //Ok()
        //}

        let goto_point = move |c: Context, finish_point: String| {
            let cur_ang: i32 = c.globals()
                .get::<_, i32>("CUR_ANG").unwrap() as i32;
            let cur_point: String = c.globals()
                .get::<_, String>("CUR_POINT").unwrap();
            let (path, end_angle) = graph::goto_point(
                &loaded_graph,
                cur_point,
                finish_point.to_owned(),
                cur_ang);
            
            let (mutex, condvar) = &*is_goto_running_c2;
            {
                let mut running = mutex.lock().unwrap();
                *running = true;
            }
            send_ch.send(TypeOfMove::GotoPoint(path)).unwrap();


            c.globals().set("CUR_ANG", end_angle);
            c.globals().set("CUR_POINT", finish_point);
            Ok(())
        };

        let rotate_to_point = move |c: Context, finish_point: String| {
            let cur_ang: i32 = c.globals()
                .get::<_, i32>("CUR_ANG").unwrap() as i32;
            let cur_point: String = c.globals()
                .get::<_, String>("CUR_POINT").unwrap();
            let (path, end_angle) = graph::goto_point(
                &loaded_graph2,
                cur_point,
                finish_point.to_owned(),
                cur_ang);
            let path = (&path[..1]).to_vec();
            if let graph::MoveAction::Rotate(_) = path[0] {
                {}
            } else {
                return Ok(())
            }
            
            let (mutex, condvar) = &*is_goto_running_c4;
            {
                let mut running = mutex.lock().unwrap();
                *running = true;
            }
            send_ch3.send(TypeOfMove::GotoPoint(path)).unwrap();


            c.globals().set("CUR_ANG", end_angle);
            //c.globals().set("CUR_POINT", finish_point);
            Ok(())
        };

        let run_macro = move |_c: Context, mac: String| {
            let mac = mac.split(",").map(|x| {x.parse::<i32>().unwrap()}).collect::<Vec<i32>>();
            let (mutex, condvar) = &*is_goto_running_c3;
            {
                let mut running = mutex.lock().unwrap();
                *running = true;
            }
            send_ch2.send(TypeOfMove::Macro(mac)).unwrap();
            Ok(())
        };

        let run_unmacro = move |_c: Context, mac: String| {
            let mac = mac.split(",").map(|x| {x.parse::<i32>().unwrap()}).collect::<Vec<i32>>();
            let (mutex, condvar) = &*is_goto_running_c5;
            {
                let mut running = mutex.lock().unwrap();
                *running = true;
            }
            send_ch4.send(TypeOfMove::UnMacro(mac)).unwrap();
            Ok(())
        };

        let get_cs_hsv = move |_c: Context, _:()|{
            let (r, g, b) = cs.get_rgb();
            let (h, s, v) = line::ControlSensor::rgb2hsv(r, g, b);
            return Ok((h, s, v))
        };

        let set_lift = move |_c: Context, sp: i32| {
            lift_motor.set_point(sp);
            Ok(())
        };
        
        let set_rotate= move |_c: Context, sp: i32| {
            rotate_motor.set_point(sp);
            Ok(())
        };

        let ride_line_degrees = move |_c: Context, degrees: i32| {
            let (mutex, condvar) = &*is_goto_running_c6;
            {
                let mut running = mutex.lock().unwrap();
                *running = true;
            }
            send_ch5.send(TypeOfMove::LineDegrees(degrees)).unwrap();
            Ok(())
        };

        let ride_outer_line_left_stop = move |_c: Context, _: ()| {
            let (mutex, condvar) = &*is_goto_running_c7;
            {
                let mut running = mutex.lock().unwrap();
                *running = true;
            }
            send_ch6.send(TypeOfMove::RightOutLine).unwrap();
            Ok(())
        };

        let lua_fun = lua_ctx.create_function_mut(set_middle_grey).expect("Lua function creationg failed");
        lua_ctx.globals().set("set_middle_grey", lua_fun).expect("Function to var in lua failed");
        let lua_fun = lua_ctx.create_function_mut(set_black).expect("Lua function creationg failed");
        lua_ctx.globals().set("set_black", lua_fun).expect("Function to var in lua failed");
        let lua_fun = lua_ctx.create_function_mut(set_white).expect("Lua function creationg failed");
        lua_ctx.globals().set("set_white", lua_fun).expect("Function to var in lua failed");

        let lua_fun = lua_ctx.create_function_mut(wait_till_arrival).expect("Lua function creationg failed");
        lua_ctx.globals().set("wait_till_arrival", lua_fun).expect("Function to var in lua failed");

        let lua_fun = lua_ctx.create_function_mut(goto_point).expect("Lua function creationg failed");
        lua_ctx.globals().set("goto_point", lua_fun).expect("Function to var in lua failed");
        let lua_fun = lua_ctx.create_function_mut(rotate_to_point).expect("Lua function creationg failed");
        lua_ctx.globals().set("rotate_to_point", lua_fun).expect("Function to var in lua failed");
        let lua_fun = lua_ctx.create_function_mut(run_macro).expect("Lua function creationg failed");
        lua_ctx.globals().set("macro", lua_fun).expect("Function to var in lua failed");

        let lua_fun = lua_ctx.create_function_mut(run_unmacro).expect("Lua function creationg failed");
        lua_ctx.globals().set("unmacro", lua_fun).expect("Function to var in lua failed");
        let lua_fun = lua_ctx.create_function_mut(ride_line_degrees).expect("Lua function creationg failed");
        lua_ctx.globals().set("ride_line_degrees", lua_fun).expect("Function to var in lua failed");
        let lua_fun = lua_ctx.create_function_mut(ride_outer_line_left_stop).expect("Lua function creationg failed");
        lua_ctx.globals().set("rolls", lua_fun).expect("Function to var in lua failed");

        let lua_fun = lua_ctx.create_function_mut(get_cs_hsv).expect("Lua function creationg failed");
        lua_ctx.globals().set("get_cs_hsv", lua_fun).expect("Function to var in lua failed");

        let lua_fun = lua_ctx.create_function_mut(set_pid).expect("Lua function creationg failed");
        lua_ctx.globals().set("set_pid", lua_fun).expect("Function to var in lua failed");

        let lua_fun = lua_ctx.create_function_mut(set_lspeed).expect("Lua function creationg failed");
        lua_ctx.globals().set("set_lspeed", lua_fun).expect("Function to var in lua failed");
        let lua_fun = lua_ctx.create_function_mut(set_rspeed).expect("Lua function creationg failed");
        lua_ctx.globals().set("set_rspeed", lua_fun).expect("Function to var in lua failed");

        let lua_fun = lua_ctx.create_function_mut(set_lift).expect("Lua function creationg failed");
        lua_ctx.globals().set("set_lift", lua_fun).expect("Function to var in lua failed");
        let lua_fun = lua_ctx.create_function_mut(set_rotate).expect("Lua function creationg failed");
        lua_ctx.globals().set("set_rotate", lua_fun).expect("Function to var in lua failed");

        let lua_fun = lua_ctx.create_function_mut(joystick_write).expect("Lua function creationg failed");
        lua_ctx.globals().set("joystick_write", lua_fun).expect("Function to var in lua failed");

        // Setuping global vars
        lua_ctx.globals().set("CUR_ANG", 0);
        lua_ctx.globals().set("CUR_POINT", "START");
        // Executing lua script
        lua_ctx
            .load(&script)
            .exec()
            .expect("Script execution failed");
    });
}

use stick::Port;
use stick;

fn joystick_write(_c: Context, _:()) -> Result<()>{
    let speed = 20;
    let degrees = 20;
    let mut port = Port::new();

    let state = Arc::new(Mutex::new(joystick::GamePadState::new()));
    let state_c = state.clone();

    loop { if let Some(_) = port.poll() { break; } }
    thread::spawn(move ||{
        loop {
            port.poll();
            {state_c.lock().unwrap().consume_device(port.get(0).unwrap())}
        }
    });

    let mut robot = line::RobotMoveBase::new();
    let mut cur_data = Vec::<i32>::new();
    loop {
        let mut val;
        { val = *state.lock().unwrap(); }
        if val.rt_b {
            let steering = val.lx*100/128;
            cur_data.push(steering);
            robot.motor_pair.steer_on_degrees(steering, speed, degrees);
            robot.motor_pair.set_steering(0, 0);
        } else if val.lt_b {
            let steering = cur_data.pop().expect("Nothing to undo");
            robot.motor_pair.steer_on_degrees(steering, -speed, degrees);
            robot.motor_pair.set_steering(0, 0);
        } else if val.cross {
            for steering in cur_data.iter().rev() {
                robot.motor_pair.steer_on_degrees(*steering, -speed, degrees);
            }
            robot.motor_pair.set_steering(0, 0);
        } else if val.circle {
            for steering in cur_data.iter() {
                robot.motor_pair.steer_on_degrees(*steering, speed, degrees);
            }
            robot.motor_pair.set_steering(0, 0);
        } else if val.triangle {
            cur_data.clear();
        } else if val.square {
            println!("{}", cur_data.iter().map(|x| {x.to_string()})
                     .collect::<Vec<String>>().join(","));
            thread::sleep(time::Duration::from_millis(400));
        }
        //println!("{:?}", val);
        //println!("printing");
        thread::sleep(time::Duration::from_millis(100));
    }
    Ok(())
}
