extern crate ev3dev_lang_rust;

use std::io::Result;
use std::sync::atomic;

use ev3dev_lang_rust::tacho_motor::{MediumMotor, LargeMotor, TachoMotor};
use ev3dev_lang_rust::tacho_motor;
use ev3dev_lang_rust::core::{MotorPort, SensorPort};
use ev3dev_lang_rust::color_sensor::ColorSensor;
use ev3dev_lang_rust::color_sensor;
use ev3dev_lang_rust::core::Sensor;

use std::thread;
use std::time;
use std::sync::mpsc;

static WHITE: atomic::AtomicI32 = atomic::AtomicI32::new(0);
static MIDDLE_GREY: atomic::AtomicI32 = atomic::AtomicI32::new(0);
static BLACK: atomic::AtomicI32 = atomic::AtomicI32::new(0);

pub struct RobotMoveBase {
    pub motor_pair: MotorPair,
    pub sensor_pair: SensorPair,
}

pub struct MotorPair {
    lmotor: MediumMotor,
    rmotor: MediumMotor,
    send_ch: mpsc::Sender<(i32, i32)>,
}

pub struct SensorPair {
    lsensor: ColorSensor,
    rsensor: ColorSensor,
}

pub struct ControlSensor {
    sensor: ColorSensor,
}

pub struct PID {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    last_error: f32,
    int: f32,
}

pub struct Lift{
    motor: MediumMotor,
}

pub struct Rotate {
    motor: MediumMotor,
    pos: i32,
}

impl Lift {
    pub fn new() -> Self {
        let mut motor = match MediumMotor::new(MotorPort::OutD) {
            Some(motor) => motor,
            None => panic!("Lift motor not found"), 
        };
        motor.set_stop_action(tacho_motor::STOP_ACTION_HOLD.to_string()).unwrap();
        motor.set_position(0).unwrap();
        return Lift{
            motor: motor,
        }
    }

    pub fn set_point(&mut self, point:i32) {
        self.motor.set_position_sp(point as isize).unwrap();
        self.motor.set_speed_sp(750).unwrap();
        self.motor.run_to_abs_pos(None).unwrap();
        while self.motor.is_running().unwrap() {
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

impl Rotate {
    pub fn new() -> Self {
        let mut motor = match MediumMotor::new(MotorPort::OutA) {
            Some(motor) => motor,
            None => panic!("Lift motor not found"), 
        };
        motor.set_stop_action(tacho_motor::STOP_ACTION_HOLD.to_string()).unwrap();
        motor.set_position(0).unwrap();
        return Rotate {
            motor: motor,
            pos: 0,
        }
    }

    pub fn set_point(&mut self, point:i32) {
        let mut cur_pos = (self.pos % 360);
        if cur_pos < 0 { cur_pos = cur_pos + 360 }
        let diff = point - cur_pos;
        let mut target;

        if diff > 180 {
            target = diff - 360;
        } else if diff < -180 {
            target = diff + 360;
        } else {
            target = diff;
        }

        self.motor.set_position_sp((self.pos + target) as isize).unwrap();
        self.pos += target;
        self.motor.set_speed_sp(250).unwrap();
        self.motor.run_to_abs_pos(None).unwrap();
        while self.motor.is_running().unwrap() {
            thread::sleep(time::Duration::from_millis(10));
        }
    }

    pub fn reset(&mut self) {
        self.pos = 0;
        self.motor.set_position(0).unwrap();
    }
}

impl ControlSensor {
    pub fn new() -> Self {
        let mut sensor = match ColorSensor::new(SensorPort::In4) {
            Some(sensor) => sensor,
            None => panic!("ControlSensor not found"),
        };
        sensor.set_mode_rgb_raw().unwrap();

        return ControlSensor {
            sensor: sensor,
        }
    }
    pub fn get_rgb(&mut self) -> (u8, u8, u8) {
        return (self.sensor.get_value0().unwrap() as u8,
                self.sensor.get_value1().unwrap() as u8,
                self.sensor.get_value2().unwrap() as u8,
                );
    }

    pub fn rgb2hsv(r: u8, g: u8, b: u8) -> (i32, i32, i32){
        let r: f32 = r as f32 / 255.0;
        let g: f32 = g as f32 / 255.0;
        let b: f32 = b as f32 / 255.0;

        let max = maxf32(maxf32(r, g), b);
        let min = minf32(minf32(r, g), b);
        let df = max - min;
        let mut h;

        if max == min {
            h = 0.0;
        } else if max == r {
            h = ((60.0 * (g-b)/df) + 360.0) % 360.0;
        } else if max == g {
            h = ((60.0 * (b-r)/df) + 120.0) % 360.0;
        } else if max == b {
            h = ((60.0 * (r-g)/df) + 240.0) % 360.0;
        } else {
            unreachable!();
        }

        let mut s: f32;
        if max == 0.0 {
            s = 0.0;
        } else {
            s = 1.0 - min/max;
        }

        let v = max;
        return ((h) as i32, (s*255.0) as i32, (v*255.0) as i32);
    }
}

impl MotorPair {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let mut lmotor = match MediumMotor::new(MotorPort::OutB) {
            Some(motor) => motor,
            None => panic!("Left motor not found"), 
        };

        let mut rmotor = match MediumMotor::new(MotorPort::OutC) {
            Some(motor) => motor,
            None => panic!("Right motor not found"), 
        };

    //thread::spawn(move || {
        //let mut lmotor = match MediumMotor::new(MotorPort::OutB) {
            //Some(motor) => motor,
            //None => panic!("Left motor not found"), 
        //};

        //let mut rmotor = match MediumMotor::new(MotorPort::OutC) {
            //Some(motor) => motor,
            //None => panic!("Right motor not found"), 
        //};
        lmotor.set_stop_action(tacho_motor::STOP_ACTION_HOLD.to_string()).unwrap();
        rmotor.set_stop_action(tacho_motor::STOP_ACTION_HOLD.to_string()).unwrap();
        
        //let mut ls: i32 = 0;
        //let mut rs: i32 = 0;

        //let mut cls: i32 = 0;
        //let mut crs: i32 = 0;

        //loop {
            //match rx.try_recv() {
                //Ok(val) => {
                    //ls = val.0;
                    //rs = val.1;
                    //cls = lmotor.get_position().unwrap() as i32;
                    //crs = rmotor.get_position().unwrap() as i32;
                //},
                //Err(e) => match e {}, 
            //}

            //let lc = (cls - lmotor.get_position().unwrap() as i32);
            //let rc = (crs - rmotor.get_position().unwrap() as i32);

            //let diff = lc - rc;
            //// + if L is more
            //// - if R is more
            //if diff > 0 {
                //lmotor.set_speed_sp((-(lc - diff)) as isize).unwrap();
                //rmotor.set_speed_sp(rc as isize).unwrap();
            //} else {
                //lmotor.set_speed_sp((-(lc)) as isize).unwrap();
                //rmotor.set_speed_sp((rc - diff) as isize).unwrap();
            //}
             
            //thread::sleep(time::Duration::from_millis(10))
        //}
    //});
        return MotorPair {
            lmotor: lmotor,
            rmotor: rmotor,
            send_ch: tx,
        }
    }

    pub fn set_speed(&mut self, lm: i32, rm: i32) {
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
    }

    pub fn go_on_degrees(&mut self, speed: i32, degrees: i32) {
        let cl = self.lmotor.get_position().unwrap() as i32;
        let cr = self.rmotor.get_position().unwrap() as i32;

        self.set_steering(0, speed);

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

        self.set_steering(steering, speed);

        while 
        (((self.lmotor.get_position().unwrap() as i32) - cl).abs() < degrees)
        && (((self.rmotor.get_position().unwrap() as i32) - cr).abs() < degrees) {
            //thread::sleep(time::Duration::from_millis(10));
        }
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

#[inline]
fn maxf32(a: f32, b: f32) -> f32 {
    if a > b { return a } else {return b}
}

#[inline]
fn minf32(a: f32, b: f32) -> f32 {
    if a < b { return a } else {return b}
}

#[inline]
pub fn middle_grey() -> i32 {
    MIDDLE_GREY.load(atomic::Ordering::Relaxed)
}

#[inline]
pub fn set_middle_grey(val: i32) {
    MIDDLE_GREY.store(val, atomic::Ordering::Relaxed);
}

#[inline]
pub fn white() -> i32 {
    WHITE.load(atomic::Ordering::Relaxed)
}

#[inline]
pub fn set_white(val: i32) {
    WHITE.store(val, atomic::Ordering::Relaxed);
}


#[inline]
pub fn black() -> i32 {
    BLACK.load(atomic::Ordering::Relaxed)
}

#[inline]
pub fn set_black(val: i32) {
    BLACK.store(val, atomic::Ordering::Relaxed);
}



//TODO fixme
pub fn ride_line_degrees(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: &mut RobotMoveBase,
    degrees: i32,
    ) {

    let sl = robot.motor_pair.lmotor.get_position().unwrap();
    let sr = robot.motor_pair.rmotor.get_position().unwrap();
    let degrees = degrees as isize;

    fn error_fun(l: i32, r: i32) -> i32{
        l - r
    }

    let (p, i, d) = pid_k;
    let mut pid = PID::new(p, i, d);

    let (ls, rs) = robot.sensor_pair.get_reflected_color();
    let error = error_fun(ls, rs);
    let diff = pid.step(error);

    loop {
        let (ls, rs) = robot.sensor_pair.get_reflected_color();
        let error = error_fun(ls, rs);
        let diff = pid.step(error);
        robot.motor_pair.set_steering(diff, speed);
        if ((robot.motor_pair.lmotor.get_position().unwrap()-sl).abs()+
            (robot.motor_pair.rmotor.get_position().unwrap()-sr).abs())/2 > degrees {
            break
        }
    }
}

pub fn ride_line(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: &mut RobotMoveBase,
    error_fun: &dyn Fn(i32, i32) -> i32,
    stop_cond: &dyn Fn(i32, i32) -> bool) {

    let (p, i, d) = pid_k;
    let mut pid = PID::new(p, i, d);

    let (ls, rs) = robot.sensor_pair.get_reflected_color();
    let error = error_fun(ls, rs);
    let diff = pid.step(error);

    loop {
        let (ls, rs) = robot.sensor_pair.get_reflected_color();
        let error = error_fun(ls, rs);
        let diff = pid.step(error);
        robot.motor_pair.set_steering(diff, speed);

        if stop_cond(ls, rs) { break }
    }
}

pub fn ride_line_cross(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: &mut RobotMoveBase) {
    #[inline]
    fn stop_cross(l: i32, r: i32) -> bool{
        (l+r)/2 < black()
    }
    fn stop_cross_white(l: i32, r: i32) -> bool{
        (l+r)/2 > white()
    }
    #[inline]
    fn both_err(l: i32, r: i32) -> i32{
        (((l - r) as f32)) as i32
    }

    //ride_line(pid_k, speed, robot, &both_err, &stop_cross_white);
    ride_line(pid_k, speed, robot, &both_err, &stop_cross);
    //ride_line(pid_k, speed, robot, &both_err, &stop_cross_white);
}

pub fn ride_outer_line_left_stop(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: &mut RobotMoveBase) {
    #[inline]
    fn stop_left(l: i32, r: i32) -> bool{
        l < black()
    }
    fn stop_left_white(l: i32, r: i32) -> bool{
        l > white()
    }

    let is_init = false;
    let mut last_l = 0;

    #[inline]
    fn ride_right(l: i32, r: i32) -> i32{
        r - middle_grey()
    }
    //ride_line(pid_k, speed, robot, &ride_right, &stop_left_white);
    ride_line(pid_k, speed, robot, &ride_right, &stop_left);
    //ride_line(pid_k, speed, robot, &ride_right, &stop_left_white);
}


pub fn ride_line_left_stop(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: &mut RobotMoveBase) {
    #[inline]
    fn stop_left(l: i32, r: i32) -> bool{
        l < black()
    }
    fn stop_left_white(l: i32, r: i32) -> bool{
        l > white()
    }

    let is_init = false;
    let mut last_l = 0;

    #[inline]
    fn ride_right(l: i32, r: i32) -> i32{
        middle_grey() - r
    }
    ride_line(pid_k, speed, robot, &ride_right, &stop_left_white);
    ride_line(pid_k, speed, robot, &ride_right, &stop_left);
    //ride_line(pid_k, speed, robot, &ride_right, &stop_left_white);
}

pub fn ride_line_right_stop(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: &mut RobotMoveBase) {
    #[inline]
    fn stop_right(l: i32, r: i32) -> bool{
        r < black()
    }
    fn stop_right_white(l: i32, r: i32) -> bool{
        r > white()
    }
    #[inline]
    fn ride_left(l: i32, r: i32) -> i32{
        l - middle_grey()
    }
    ride_line(pid_k, speed, robot, &ride_left, &stop_right_white);
    ride_line(pid_k, speed, robot, &ride_left, &stop_right);
    //ride_line(pid_k, speed, robot, &ride_left, &stop_right_white);
}

pub fn turn_count(robot: &mut RobotMoveBase, count: i32, speed: i32) {
    println!("count turn: {}", count);
    if count == 0 { return }
    if count > 0 {
        //turning right
        robot.motor_pair.set_steering(100, speed);
        for _ in 0..count {
            loop {
                //white
                let (ls, rs) = robot.sensor_pair.get_reflected_color();
                if rs > white() { break }
            }
            loop {
                //black
                let (ls, rs) = robot.sensor_pair.get_reflected_color();
                if rs < black() { break }
            }
            loop {
                //white
                let (ls, rs) = robot.sensor_pair.get_reflected_color();
                if rs > white() { break }
            }
        }
        robot.motor_pair.set_steering(0, 0);
    } else {
        //turning left
        robot.motor_pair.set_steering(-100, speed);
        for _ in 0..(-count) {
            loop {
                //white
                let (ls, rs) = robot.sensor_pair.get_reflected_color();
                if ls > white() { break }
            }
            loop {
                //black
                let (ls, rs) = robot.sensor_pair.get_reflected_color();
                if ls < black() { break }
            }
            loop {
                //white
                let (ls, rs) = robot.sensor_pair.get_reflected_color();
                if ls > white() { break }
            }
        }
        robot.motor_pair.set_steering(0, 0);
    }
}
