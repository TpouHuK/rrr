extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::tacho_motor::{MediumMotor, LargeMotor, TachoMotor};
use ev3dev_lang_rust::tacho_motor;
use ev3dev_lang_rust::core::{MotorPort, SensorPort};
use ev3dev_lang_rust::color_sensor::ColorSensor;
use ev3dev_lang_rust::color_sensor;
use ev3dev_lang_rust::core::Sensor;

use std::thread;
use std::time;
use std::sync::mpsc;

pub struct RobotMoveBase {
    pub motor_pair: MotorPair,
    pub sensor_pair: SensorPair,
}

pub struct MotorPair {
    pub lmotor: MediumMotor,
    pub rmotor: MediumMotor,
    send_ch: mpsc::SyncSender<(i32, i32, bool, bool)>,
    //TODO add message type instead of bare tuple
}

pub struct SensorPair {
    lsensor: ColorSensor,
    rsensor: ColorSensor,
}

pub struct PID {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    last_error: f32,
    int: f32,
}

impl MotorPair {
    pub fn new() -> Self {
        // Old init
         let mut lmotor = match MediumMotor::new(MotorPort::OutB) {
            Some(motor) => motor,
            None => panic!("Left motor not found"), 
        };

        let mut rmotor = match MediumMotor::new(MotorPort::OutC) {
            Some(motor) => motor,
            None => panic!("Right motor not found"), 
        }; 

        //New init
        let (tx, rx) = mpsc::sync_channel::<(i32, i32, bool, bool)>(1);

    thread::spawn(move || {
        #[inline]
        fn limit(a: i32) -> i32 {
            return if a > 100 { 100 } else if a < -100 { -100 } else { a }
        }

        let mut lmotor = match MediumMotor::new(MotorPort::OutB) {
            Some(motor) => motor,
            None => panic!("Left motor not found"), 
        };

        let mut rmotor = match MediumMotor::new(MotorPort::OutC) {
            Some(motor) => motor,
            None => panic!("Right motor not found"), 
        };
        lmotor.set_stop_action(tacho_motor::STOP_ACTION_HOLD.to_string()).unwrap();
        rmotor.set_stop_action(tacho_motor::STOP_ACTION_HOLD.to_string()).unwrap();
        
        let mut ls: i32 = 0;
        let mut rs: i32 = 0;

        let mut cls: i32 = 0;
        let mut crs: i32 = 0;

        let mut is_adjusting = false;
        let mut val = (0, 0, false);
        let mut pid = PID::new(0.03, 0.0, 0.01);
        loop {
            // TODO remove code duplication
            if is_adjusting {
                match rx.try_recv() {
                    Ok(val) => {
                        ls = val.0;
                        rs = val.1;
                        is_adjusting = val.2; 
                        let is_reset_degrees = val.3;
                        if is_reset_degrees {
                            cls = lmotor.get_position().unwrap() as i32;
                            crs = rmotor.get_position().unwrap() as i32;
                        }
                    },
                    Err(e) => match e {
                        mpsc::TryRecvError::Disconnected => { break; },
                        mpsc::TryRecvError::Empty => {},
                    }, 
                }
            } else {
                match rx.recv() {
                    Ok(val) => {
                        ls = val.0;
                        rs = val.1;
                        is_adjusting = val.2;
                        let is_reset_degrees = val.3;
                        if is_reset_degrees {
                            cls = lmotor.get_position().unwrap() as i32;
                            crs = rmotor.get_position().unwrap() as i32;
                        }
                    },
                    Err(_) => { break; }
                }
            }

            let lc = -(cls - lmotor.get_position().unwrap() as i32);
            let rc = (crs - rmotor.get_position().unwrap() as i32);

            let diff = pid.step(lc*rs - rc*ls);

            let lse = limit(ls + rs.signum()*diff)*15;
            let rse = limit(rs - ls.signum()*diff)*15;

            lmotor.set_speed_sp((-lse) as isize).unwrap();
            rmotor.set_speed_sp(rse as isize).unwrap();

            if lse == 0 { lmotor.stop().unwrap();}
            else { lmotor.run_forever().unwrap();}
            if rse == 0 { rmotor.stop().unwrap();}
            else { rmotor.run_forever().unwrap();}
             
            thread::sleep(time::Duration::from_millis(50))
        }
    }); // end of thread

        return MotorPair {
            lmotor: lmotor,
            rmotor: rmotor,
            send_ch: tx,
        }
    }

    fn set_speed(&mut self, lm: i32, rm: i32) {
        fn limit(a: i32) -> i32 {
            return if a > 100 { 100 } else if a < -100 { -100 } else { a }
        }

        let lm = limit(lm);
        let rm = limit(rm);

        // 1560
        self.lmotor.set_speed_sp((-(lm*15)) as isize).unwrap();
        self.rmotor.set_speed_sp((rm*15) as isize).unwrap();


        if lm != 0 { self.lmotor.run_forever().unwrap(); }
        else { self.lmotor.stop().unwrap(); }

        if rm != 0 { self.rmotor.run_forever().unwrap(); } 
        else { self.rmotor.stop().unwrap(); }
    }

    //TODO fix code repetition
    pub fn set_steering(&mut self, steering: i32, speed: i32) {
        let mut lmot;
        let mut rmot;
        if steering > 0 {
            lmot = speed;
            rmot = speed - steering * speed / 50 ;
        } else {
            lmot = speed + steering * speed / 50 ;
            rmot = speed;
        }
        
        self.set_speed(lmot, rmot);
        //self.send_ch.send((lmot, rmot, false, true));
    }
    
    pub fn set_pid_steering(&mut self, steering: i32, speed: i32) {
        let mut lmot;
        let mut rmot;
        if steering > 0 {
            lmot = speed;
            rmot = speed - steering * speed / 50 ;
        } else {
            lmot = speed + steering * speed / 50 ;
            rmot = speed;
        }
        if speed == 0 {
            self.send_ch.send((lmot, rmot, false, true));
        } else {
            self.send_ch.send((lmot, rmot, true, true));
        }
    }

    pub fn set_pid_steering_no_reset(&mut self, steering: i32, speed: i32) {
        let mut lmot;
        let mut rmot;
        if steering > 0 {
            lmot = speed;
            rmot = speed - steering * speed / 50 ;
        } else {
            lmot = speed + steering * speed / 50 ;
            rmot = speed;
        }
        self.send_ch.send((lmot, rmot, true, false));
    }

    pub fn go_on_degrees(&mut self, speed: i32, degrees: i32) {
        let cl = self.lmotor.get_position().unwrap() as i32;
        let cr = self.rmotor.get_position().unwrap() as i32;

        self.set_pid_steering(0, speed);

        while 
        (((self.lmotor.get_position().unwrap() as i32) - cl).abs() < degrees)
        || (((self.rmotor.get_position().unwrap() as i32) - cr).abs() < degrees) {
            thread::sleep(time::Duration::from_millis(10));
        }
    }

    pub fn steer_on_degrees(&mut self, steering: i32, 
                            speed: i32, degrees: i32) {
        let cl = self.lmotor.get_position().unwrap() as i32;
        let cr = self.rmotor.get_position().unwrap() as i32;

        self.set_pid_steering(steering, speed);

        while 
        (((self.lmotor.get_position().unwrap() as i32) - cl).abs() < degrees)
        && (((self.rmotor.get_position().unwrap() as i32) - cr).abs() < degrees) {
            //thread::sleep(time::Duration::from_millis(10));
        }
    }
}

impl PID {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        return PID {
            kp: kp,
            ki: ki,
            kd: kd,
            last_error: 0.0,
            int: 0.0,
        }
    }

    pub fn step(&mut self, err: i32) -> i32 {
        let err = err as f32;

        self.int = self.int + err;
        if self.int > 10_000.0 { self.int = 10_000.0 } 
        else if self.int < -10_000.0 { self.int = -10_000.0 }

        let p = err * self.kp;
        let i = self.int * self.ki;
        let d = (err - self.last_error) * self.kd;

        let result = p + i + d;
        self.last_error = err;

        return result as i32;
    }

    pub fn clean(&mut self) {
        self.last_error = 0.0;
        self.int = 0.0;
    }
}

impl SensorPair {
    pub fn new() -> Self {
        let lsensor = match ColorSensor::new(SensorPort::In2) {
            Some(sensor) => sensor,
            None => panic!("Left sensor not found"), 
        };

        let rsensor = match ColorSensor::new(SensorPort::In3) {
            Some(sensor) => sensor,
            None => panic!("Right sensor not found"), 
        };

        return SensorPair {
            lsensor: lsensor,
            rsensor: rsensor,
        }
    }

    pub fn get_reflected_color(&mut self) -> (i32, i32) {
        (self.lsensor.get_value0().unwrap() as i32,
        self.rsensor.get_value0().unwrap() as i32)
    }
}

impl RobotMoveBase {
    pub fn new() -> Self {
        return RobotMoveBase {
            motor_pair: MotorPair::new(),
            sensor_pair: SensorPair::new(),
        }
    }
}

pub fn ride_line(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: &mut RobotMoveBase,) {
    // error_fun: &dyn Fn(i32, i32) -> i32,) {
    // stop_cond: &dyn Fn(i32, i32) -> bool) {

    #[inline]
    fn limit(a: i32) -> i32 {
        return if a > 100 { 100 } else if a < 0 { 0 } else { a }
    }

    let (p, i, d) = pid_k;
    let mut pid = PID::new(p, i, d);

    let (ls, rs) = robot.sensor_pair.get_reflected_color();
    // let error = error_fun(ls, rs);
    let error = (ls - rs) as i32;
    let diff = pid.step(error);

    let mut counter = 0;
    let start = time::Instant::now();
    loop {
        let cycle_start = time::Instant::now();
        counter += 1;
        let (ls, rs) = robot.sensor_pair.get_reflected_color();
        // let error = error_fun(ls, rs);
        let error = (ls - rs) as i32;
        let diff = pid.step(error);

        // MaGiCk!1
        let mycof = 30;
        let end_speed = speed - limit(error*error)*(speed*80/100)/100/mycof;
        robot.motor_pair.set_steering(diff, end_speed);
        if start.elapsed().as_secs() > 10 { break; }
        // if stop_cond(ls, rs) { break }
        // while cycle_start.elapsed() < time::Duration::from_millis(1) {}
    }
    eprintln!("{}", counter);
    eprintln!("{}", counter/10);
}

fn main() {
    let mut robot = RobotMoveBase::new();
    ride_line((0.5, 0.0, 20.0), 30, &mut robot);
}
