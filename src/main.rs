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

const DEFAULT_SPEED: i32 = 40;
const DEFAULT_KP: f32 = 1.0;
const DEFAULT_KI: f32 = 0.0;
const DEFAULT_KD: f32 = 0.0;

const LINE_DEGREES: i32 = 80;

const DEFAULT_FILENAME: &str = "script.lua";
const ENV_FILENAME_VAR: &str = "LUA_FILENAME";


const MACRO_DEGREES: i32 = 20;
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
    Degrees((i32)),
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
    let is_goto_running_c8 = is_goto_running.clone();

    let (send_ch, receive_ch) = std::sync::mpsc::channel();
    let send_ch2 = send_ch.clone();
    let send_ch3 = send_ch.clone();
    let send_ch4 = send_ch.clone();
    let send_ch5 = send_ch.clone();
    let send_ch6 = send_ch.clone();
    let send_ch7 = send_ch.clone();

    let mut kpidb = Arc::new(Mutex::new((DEFAULT_KP,
                                        DEFAULT_KI,
                                        DEFAULT_KD)));
    let mut kpidb_c = kpidb.clone();

    let mut kpid = Arc::new(Mutex::new((DEFAULT_KP,
                                        DEFAULT_KI,
                                        DEFAULT_KD)));
    let mut kpid_c = kpid.clone();

    let mut klspeed = Arc::new(Mutex::new(DEFAULT_SPEED));
    let mut klspeed_c = klspeed.clone();
    let mut klspeed_c2 = klspeed.clone();


    let mut kmspeed = Arc::new(Mutex::new(MACRO_SPEED));
    let mut kmspeed_c = kmspeed.clone();

    let mut krspeed = Arc::new(Mutex::new(DEFAULT_SPEED));
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
            let mut pidb;
            let mut pid;
            let mut lspeed;
            let mut rspeed;
            {pidb = *kpidb.lock().unwrap()};
            {pid = *kpid.lock().unwrap()};
            {lspeed = *klspeed.lock().unwrap()};
            {rspeed = *krspeed.lock().unwrap()};

            for elem in path {
                match elem {
                    graph::MoveAction::LineRide(lineride) => {
                        match lineride {
                            graph::LineRide::CrossStop => {
                                line::ride_line_cross(pidb, lspeed, &mut robot);
                            },
                            graph::LineRide::LeftStop=> {
                                line::ride_line_left_stop(pid, lspeed, &mut robot);
                            },
                            graph::LineRide::RightStop => {
                                line::ride_line_right_stop(pid, lspeed, &mut robot);
                            },
                        }
                        robot.motor_pair.go_on_degrees(lspeed, LINE_DEGREES);
                    },
                    graph::MoveAction::Rotate(count) => {
                        robot.motor_pair.set_steering(0, 0);
                        thread::sleep(time::Duration::from_millis(100));
                        line::turn_count(&mut robot, count, rspeed);
                    }
                }
            }
            robot.motor_pair.set_steering(0, 0);
            },
                TypeOfMove::Macro(steerings) => {
                    let mut mspeed;
                    {mspeed = *kmspeed.lock().unwrap()};
                    for steering in steerings.iter() {
                        robot.motor_pair.steer_on_degrees(*steering, mspeed, MACRO_DEGREES);
                    }
                    robot.motor_pair.set_steering(0, 0);
                },
                TypeOfMove::UnMacro(steerings) => {
                    let mut mspeed;
                    {mspeed = *kmspeed.lock().unwrap()};
                    for steering in steerings.iter() {
                        robot.motor_pair.steer_on_degrees(*steering, -mspeed, MACRO_DEGREES);
                    }
                    robot.motor_pair.set_steering(0, 0);
                },
                TypeOfMove::LineDegrees(degrees) => {
                    let mut pidb;
                    let mut lspeed;
                    {pidb = *kpidb.lock().unwrap()};
                    {lspeed = *klspeed.lock().unwrap()};
                    line::ride_line_degrees(pidb, lspeed, &mut robot, degrees);
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
                TypeOfMove::Degrees(degrees) => {
                    let mut mspeed;
                    {mspeed = *kmspeed.lock().unwrap()};
                    robot.motor_pair.go_on_degrees(mspeed, degrees);
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
        let lua_sleep = |_c, secs: u64|{
            thread::sleep(time::Duration::from_secs(secs));
            Ok(())
        };

        let set_mspeed = move |_c, ispeed: i32|{
            let mutex = &*kmspeed_c;
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

        let set_pidb = move |_c: Context, (kp, ki, kd):(f32, f32, f32)|{
            let mutex = &*kpidb_c;
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

        let ride_degrees = move |_c: Context, degrees: i32| {
            let (mutex, condvar) = &*is_goto_running_c8;
            {
                let mut running = mutex.lock().unwrap();
                *running = true;
            }
            send_ch7.send(TypeOfMove::Degrees(degrees)).unwrap();
            Ok(())
        };

        macro_rules! create_lua_func {
            ($lua_ctx:ident, $rust_func:expr, $lua_name:expr) => {
                $lua_ctx.globals().set($lua_name, 
                    $lua_ctx.create_function_mut($rust_func).expect("Lua function creation failed")
                ).expect("Function to var in lua failed");
            }
        }

        create_lua_func!(lua_ctx, set_middle_grey, "set_middle_grey");
        create_lua_func!(lua_ctx, set_black, "set_black");
        create_lua_func!(lua_ctx, set_white, "set_white");
        create_lua_func!(lua_ctx, wait_till_arrival, "wait_till_arrival");

        create_lua_func!(lua_ctx, goto_point, "r_goto_point");
        create_lua_func!(lua_ctx, rotate_to_point, "rotate_to_point");
        create_lua_func!(lua_ctx, run_macro, "r_macro");

        create_lua_func!(lua_ctx, run_unmacro, "r_unmacro");
        create_lua_func!(lua_ctx, ride_line_degrees, "ride_line_degrees");
        create_lua_func!(lua_ctx, ride_outer_line_left_stop, "rolls");
        create_lua_func!(lua_ctx, ride_degrees, "r_ride_degrees");

        create_lua_func!(lua_ctx, get_cs_hsv, "get_cs_hsv");

        create_lua_func!(lua_ctx, set_pid, "set_pid");
        create_lua_func!(lua_ctx, set_pidb, "set_pidb");

        create_lua_func!(lua_ctx, set_lspeed, "set_lspeed");
        create_lua_func!(lua_ctx, set_rspeed, "set_rspeed");
        create_lua_func!(lua_ctx, set_mspeed, "set_mspeed");

        create_lua_func!(lua_ctx, lua_sleep, "sleep");

        create_lua_func!(lua_ctx, set_lift, "set_lift");
        create_lua_func!(lua_ctx, set_rotate, "set_rotate");

        create_lua_func!(lua_ctx, joystick_write, "joystick_write");
        create_lua_func!(lua_ctx, joystick_line, "joystick_line");

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


fn joystick_line(_c: Context, _:()) -> Result<()>{
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
    let mut cur_mode = "s".to_string();
    let mut kspeed = 0;
    let mut kp = 0.0;
    let mut kd = 0.0;
    let mut running = true;
    let mut pid = line::PID::new(kp, 0.0, kd);

    fn error_fun(l: i32, r: i32) -> i32{
        (((l - r) as f32)) as i32
    }

    loop {
        let mut val;
        { val = *state.lock().unwrap(); }
        if val.up_dpad {
            if cur_mode == "s" {
                kspeed += 1;
            } else if cur_mode == "p" {
                kp += 0.1;
            } else if cur_mode == "d" {
                kd += 0.1;
            }
            pid.kp = kp;
            pid.kd = kd;
            thread::sleep(time::Duration::from_millis(100));
        } else if val.down_dpad {
            if cur_mode == "s" {
                kspeed -= 1;
            } else if cur_mode == "p" {
                kp -= 0.1;
            } else if cur_mode == "d" {
                kd -= 0.1;
            }
            pid.kp = kp;
            pid.kd = kd;
            thread::sleep(time::Duration::from_millis(100));
        } else if val.triangle {
            cur_mode = "s".to_string();
            thread::sleep(time::Duration::from_millis(100));
        } else if val.square {
            cur_mode = "p".to_string();
            thread::sleep(time::Duration::from_millis(100));
        } else if val.circle {
            cur_mode = "d".to_string();
            thread::sleep(time::Duration::from_millis(100));
        } else if val.cross {
            running = !running;
            thread::sleep(time::Duration::from_millis(100));
        } else if val.option {
            println!("{}, {}, {}", kp, kd, kspeed);
            thread::sleep(time::Duration::from_millis(400));
        }

        if running {
            let (ls, rs) = robot.sensor_pair.get_reflected_color();
            let error = error_fun(ls, rs);
            let diff = pid.step(error);
            robot.motor_pair.set_steering(diff, kspeed);
        } else {
            robot.motor_pair.set_steering(0,0);
        }

        // Global loop sleep
        thread::sleep(time::Duration::from_millis(10));
    }

    Ok(())
}
