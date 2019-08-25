use std::fs::DirEntry;
use std::fs::File;
use std::fs;
use std::io;
use std::io::Write;
use std::os::unix::fs::FileExt;
use std::path::Path;

use stick::Port;
use stick::Btn;

pub struct GamePad {
    port: stick::Port,
}

pub struct GamePadLeds {
    basename: String,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct GamePadState {
    pub lx: i32,
    pub ly: i32,
    pub rx: i32,
    pub ry: i32,
    
    pub rt: i32,
    pub lt: i32,

    pub rt_b: bool,
    pub lt_b: bool,

    pub circle: bool,
    pub cross: bool,
    pub triangle: bool,
    pub square: bool,

    pub left_dpad: bool,
    pub right_dpad: bool,
    pub down_dpad: bool,
    pub up_dpad: bool,

    pub lc: bool,
    pub rc: bool,

    pub option: bool,
    pub share: bool,
}

impl GamePadState {
    pub fn new() -> Self {
        return GamePadState {
            ..Default::default()
        }
    }

    //needs reimplement using From trait
    pub fn consume_device(&mut self, device: &stick::Device) {
        let (lx, ly) = device.joy().unwrap();
        self.lx = (lx * 128.0) as i32;
        self.ly = (ly * 128.0) as i32;
        let (rx, ry) = device.cam().unwrap();
        self.rx = (rx * 128.0) as i32;
        self.ry = (ry * 128.0) as i32;
        let (lt, rt) = device.lrt().unwrap();
        self.rt = (rt * 128.0) as i32;
        self.lt = (lt * 128.0) as i32;

        self.up_dpad = device.btn(Btn::Up).unwrap();
        self.down_dpad = device.btn(Btn::Down).unwrap();
        self.left_dpad = device.btn(Btn::Left).unwrap();
        self.right_dpad = device.btn(Btn::Right).unwrap();

        self.circle = device.btn(Btn::A).unwrap();
        self.cross = device.btn(Btn::B).unwrap();
        self.triangle = device.btn(Btn::X).unwrap();
        self.square = device.btn(Btn::Y).unwrap();

        self.lc = device.btn(Btn::D).unwrap();
        self.rc = device.btn(Btn::C).unwrap();

        self.rt_b = device.btn(Btn::Z).unwrap();
        self.lt_b = device.btn(Btn::W).unwrap();

        self.share = device.btn(Btn::F).unwrap();
        self.option = device.btn(Btn::E).unwrap();
    }
}

impl GamePadLeds {
    pub fn new() -> Self {
        // Option instead of using "" as None FIXME
        // Who cares about crates.io/crates/glob, i don't want
        // to drag additional dependencies, that's just 10 liner >_<

        let mut path = "".to_string();
        for file in fs::read_dir("/sys/class/leds/").unwrap() {
            let file = file.unwrap();
            let name = file.file_name();
            if name.into_string().unwrap().starts_with("0003:054C:09CC.") {
                path = file.path().into_os_string().into_string().unwrap();
                break;
            }
        }
        if path == "".to_string() { panic!("Can't found joystick leds") };
        let where_to_cut = "/sys/class/leds/0003:054C:09CC.0008".len();
        let path = path[..where_to_cut].to_string();
        println!("{}", path);

        return GamePadLeds {
            basename: path,
        }
    }

    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        let mut rf = File::create(self.basename.to_owned()
                                     + &":red/brightness").unwrap();
        let mut gf = File::create(self.basename.to_owned()
                                     + &":green/brightness").unwrap();
        let mut bf = File::create(self.basename.to_owned()
                                     + &":blue/brightness").unwrap();
        rf.write_all_at(r.to_string().as_bytes(), 0).unwrap();
        gf.write_all_at(g.to_string().as_bytes(), 0).unwrap();
        bf.write_all_at(b.to_string().as_bytes(), 0).unwrap();
    }
}

 //RAINBOW!!!
//use std::time;
//use std::thread;
//pub fn main() {
    //let mut js = GamePad::new();

    //let mut red = 0.;
    //let mut green = 0.;
    //let mut blue = 0.;
    //let mut angle = 0.;
    //loop {
      //angle += 0.1;
      //angle %= 360.;
      //if (angle<60.) {red = 255.0; green = angle*4.25-0.01; blue = 0.;} else
      //if (angle<120.) {red = (120.0-angle)*4.25-0.01; green = 255.; blue = 0.;} else 
      //if (angle<180.) {red = 0.; green = 255.; blue = (angle-120.)*4.25-0.01;} else 
      //if (angle<240.) {red = 0.; green = (240.-angle)*4.25-0.01; blue = 255.;} else 
      //if (angle<300.) {red = (angle-240.)*4.25-0.01; green = 0.; blue = 255.;} else 
                     //{red = 255.; green = 0.; blue = (360.-angle)*4.25-0.01;} 

    //js.set_rgba(red as u8, green as u8, blue as u8);
    //thread::sleep(time::Duration::from_millis(1))
    //}
//}

//loop {
    //port.poll();
    
    //// Cycle through all currently plugged in devices.
    //if let Some(state) = port.get(id) {
        //println!("{}: {}", id, state);
    //}
//}
