extern crate ev3dev_lang_rust;

use std::io::Result;
use std::sync::atomic;

use ev3dev_lang_rust::tacho_motor::{MediumMotor, LargeMotor, TachoMotor};
use ev3dev_lang_rust::core::{MotorPort, SensorPort};
use ev3dev_lang_rust::color_sensor::ColorSensor;

static MIDDLE_GREY: atomic::AtomicI32 = atomic::AtomicI32::new(0);

pub struct RobotMoveBase {
    motor_pair: MotorPair,
    sensor_pair: SensorPair,
}

pub struct MotorPair {
    lmotor: MediumMotor,
    rmotor: MediumMotor,
}

pub struct SensorPair {
    lsensor: ColorSensor,
    rsensor: ColorSensor,
}

pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    last_error: f32,
    int: f32,
}

impl MotorPair {
    pub fn new() -> Self {
        let lmotor = match MediumMotor::new(MotorPort::OutB) {
            Some(motor) -> motor,
            None -> panic!("Left motor not found"), 
        };

        let rmotor = match MediumMotor::new(MotorPort::OutB) {
            Some(motor) -> motor,
            None -> panic!("Right motor not found"), 
        };

        return MotorPair {
            lmotor: lmotor,
            rmotor: rmotor,
        }
    }

    fn set_speed(&mut self, lm: i32, rm: i32) {
        fn limit(a: i32) {
            if a > 100 { 100 } else if a < -100 { -100 } else { a }
        }

        let lm = limit(lm);
        let rm = limit(lm);

        self.lmotor.set_duty_cycle_sp(lm as isize);
        self.lmotor.set_duty_cycle_sp((-rm) as isize);

        if lm != 0 { self.lmotor.run_forever(); }
        else { self.lmotor.stop(); }
        if rm != 0 { self.rmotor.run_forever(); } 
        else { self.rmotor.stop(); }
    }

    fn set_steering(&mut self, steering: i32, speed: i32) {
        if steering > 0 {
            let lmot = speed;
            let rmot = speed - steering * speed / 50 ;
        } else {
            let lmot = speed - steering * speed / 50 ;
            let rmot = speed;
        }
        
        self.set_speed(lmot, rmot);
    }
}

impl SensorPair {
    pub fn new() -> Self {
        let lsensor= match ColorSensor::new(SensorPort::In2) {
            Some(sensor) -> sensor,
            None -> panic!("Left sensor not found"), 
        };

        let rmotor = match ColorSensor::new(SensorPort::In3) {
            Some(sensor) -> sensor,
            None -> panic!("Right sensor not found"), 
        };

        return MotorPair {
            lsensor: lsensor,
            rsensor: rsensor,
        }
    }

    fn get_reflected_color() -> (i32, i32) {
        unimplemented!();
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

    pub step(&mut self, err: i32) -> i32 {
        let err = err as f32;

        self.int = self.int + err;
        if self.int > 10_000 { self.int = 10_000 } 
        else if self.int < -10_000 { self.int = -10_000 }

        let p = err * self.kp;
        let i = self.int * self.ki;
        let d = (err - self.last_error) * self.kd;

        let result = p + i + d;
        self.last_error = err;

        return result;
    }
}

pub fn ride_line(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: Robot,
    error_fun: &Fn(i32, i32) -> i32,
    stop_cond: &Fn(i32, i32) -> bool) {

    let (p, i, d) = pid_k
    let pid = PID::new(p, i, d);

    loop {
        let ls, rs = robot.sensor_pair.get_reflected_color();
        let error = error_fun(ls, rs);
        diff = pid.step(error);
        robot.motor_pair.set_steering(diff, speed);

        if stop_cond(ls, rs) { break }
    }
}

#[inline]
pub fn middle_grey() -> i32 {
    MIDDLE_GREY.load(atomic::Ordering::Relaxed);
}

#[inline]
pub fn set_middle_grey(val: i32) {
    MIDDLE_GREY.store(val, atomic::Ordering::Relaxed);
}

pub fn ride_line_cross(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: Robot) {
    #[inline]
    fn stop_cross(l: i32, r: i32) -> bool{
        (l+r)/2 < middle_grey();
    }
    #[inline]
    fn both_err(l: i32, r: i32) -> i32{
        l - r;
    }

    ride_line(pid_k, speed, robot, both_err, stop_cross)
}

pub fn ride_line_left_stop(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: Robot) {
    #[inline]
    fn stop_left(l: i32, r: i32) -> bool{
        l < middle_grey();
    }
    #[inline]
    fn ride_right(l: i32, r: i32) -> i32{
        middle_grey() - r;
    }
    ride_line(pid_k, speed, robot, ride_right, stop_cross);
}

pub fn ride_line_right_stop(
    pid_k: (f32, f32, f32),
    speed: i32,
    robot: Robot) {
    #[inline]
    fn stop_right(l: i32, r: i32) -> bool{
        r < middle_grey();
    }
    #[inline]
    fn ride_left(l: i32, r: i32) -> i32{
        l - middle_grey();
    }
    ride_line(pid_k, speed, robot, ride_left, stop_cross);
}
