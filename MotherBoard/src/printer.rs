use std::{cmp::min, str, time::Duration, thread, fs::File, io::Read};
use tokio::time;

use self::gcode::Command;
mod gcode;

pub const BUFFERED_INSTRUCTIONS: usize = 4;
const PORT: &str = if cfg!(unix) { "/dev/serial0" } else if cfg!(windows) { "COM10" } else { "Unimplemented" };

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Initializing,
    Idle,
    Printing(PrintStatus),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PrintStatus {
    pub sent: usize,
    pub completed: usize,
    pub progress: u8,
    pub time_left: u16,
    pub silent_progress: u8,
    pub silent_time_left: u16,
}
impl Printer {

    pub async fn initialize(&mut self) {
        println!("Setting up printer...");
        self.listen();
        time::sleep(time::Duration::from_millis(100)).await;

        if cfg!(not(unix)) {
            println!("Virtual printer for debugging!");
            return;
        }
        
        self.home().await;
        println!("Printer ready !")
    }

    pub async fn home(&mut self) {
        match self.state {
            State::Initializing => return,
            State::Idle => {
                println!("Initialising controller...");
                self.commands.clear();
                _ = self.serial.write_all(&[2]);
                _ = self.serial.write_all(&[b'H', 0b111]);
                self.state = State::Initializing;
                while self.state == State::Initializing {
                    time::sleep(time::Duration::from_secs(2)).await;
                }
                println!("Controller initialised");
            }
            _ => {
                println!("Printer is busy. Stop the current print before homing")
            }
        }
    }

    pub const fn get_state_ref(&self) -> &State { &self.state }


    pub fn load_file(&mut self, path: &str){
        println!("Loading file \"{path}\"...");
        let mut file = File::open(path).unwrap();
        let mut code = String::new();
        _ = file.read_to_string(&mut code);
        self.load_gcode(&code);
    }
    
    pub fn load_gcode(&mut self, code: &str) { // Not supported: M862.3, M862.1
        println!("Parsing gcode...");
        self.commands.clear();
        let mut local: Vec<Box<dyn gcode::Command>> = Vec::new();
        
        for line in code.split("\n") {
            let line = line.strip_suffix('\r').unwrap_or(line);
            let command: Result<Box<dyn gcode::Command>,_> = gcode::parse(line, &self.settings);
            if let Ok(command) = command {
                let scope = command.scope();
                if scope & gcode::scope::CONFIG != 0 {command.edit_config(&mut self.settings);}
                if scope & gcode::scope::PI != 0 { local.push(command); }
                else if scope & gcode::scope::ARDUINO != 0 && command.data_arduino().len() != 0 {
                    self.commands.push((command, local));
                    local = Vec::new();
                }
            } else {
                println!("Unhandled GCommand \"{}\"", line);
            }
        }
        if local.len() != 0 {
            let command: Box<dyn gcode::Command> = gcode::parse("G4", &self.settings).unwrap();
            self.commands.push((command, local));
        }
        println!("Sucess!");
    }

    pub async fn print(&mut self){
        if self.state != State::Idle || self.commands.len() == 0 {
            println!("Cannot start a print");
            return
        }

        self.state = State::Printing(PrintStatus { sent: 0, completed: 0, progress: 0, time_left: 0, silent_progress: 0, silent_time_left: 0 });
        
        println!("Printing...");
        for _ in 0..min(BUFFERED_INSTRUCTIONS+1, self.commands.len()) {
            self.send_next_command();
        }
        
        while let State::Printing(_) = self.state { time::sleep(time::Duration::from_secs(2)).await; }
        println!("Print finished !");
    }

    pub fn cancel_print(&mut self){
        self.commands.clear();
        self.state = State::Idle;
    }


    fn handle_data(&mut self, data: &[u8]){
        match data[0] {
            b'D' => { println!("[Arduino] (debug) {}", str::from_utf8(&data[1..]).unwrap()); }
            b'I' => { println!("[Arduino] {}", str::from_utf8(&data[1..]).unwrap()); }
            b'E' => { println!("[Arduino] [ERROR] {}", str::from_utf8(&data[1..]).unwrap()); }
            b'_' => {
                if self.state == State::Initializing { // home finished
                    self.state = State::Idle;
                    _ = self.serial.write_all(&[5]);
                    _ = self.serial.write_all(&[b'P', 0, 0, 0, 0]);
                } else if let State::Printing(ref status) = self.state { // print in progress
                    if status.sent < self.commands.len() { self.send_next_command(); }
                    let State::Printing(ref mut status) = self.state else {unreachable!() };
                    status.completed += 1;
                    for command in &self.commands.get(status.completed-1).unwrap().1 {  command.run_pi(status); }
                    if status.completed == self.commands.len() { self.state = State::Idle; }
                }
            }
            _ => {}
        }
    }

    fn listen(&mut self){
        let mut serial = self.serial.try_clone().unwrap();
        thread::spawn(move || {
            let mut wait: Option<u32> = None;
            println!("Listening on serial port \"{}\"", PORT);
            loop {
                let n = serial.bytes_to_read().unwrap_or(0);
                if n == 0 {continue;}
                match wait {
                    None => {
                        let mut buf = [0 as u8];
                        _ = serial.read(&mut buf);
                        wait = Some(buf[0] as u32);
                    }
                    Some(w) => {
                        if n < w {continue;}
                        let mut data = vec![0; w as usize];
                        _ = serial.read(&mut data);
                        // println !("Received: {:?}", data);
                        get().handle_data(&data);
                        wait = None;
                    }
                }

            }
        });
    }

    fn send_next_command(&mut self){
        let State::Printing(ref mut status) = self.state else { unreachable!()};
        let data: &Vec<u8> = self.commands.get(status.sent).unwrap().0.data_arduino();
        status.sent+=1;
        _ = self.serial.write_all(&[data.len() as u8]);
        _ = self.serial.write_all(&data);
        // println!("Sending {:?}", data);
    }
}

pub struct Printer {
    state: State,
    commands: Vec<(Box<dyn Command>, Vec<Box<dyn Command>>)>,
    serial: Box<dyn serialport::SerialPort>,
    settings: Settings
}

pub fn get() -> &'static mut Printer {
    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(Printer {
                state: State::Idle,
                commands: Vec::new(),
                serial: serialport::new(PORT, 115_200).timeout(Duration::from_millis(10)).open().expect("Failed to open port"),
                settings: Settings {
                    x_coder: Encoder { resolution: 3, offset: 20 },
                    y_coder: Encoder { resolution: 3, offset: 20 },
                    z_coder: Encoder { resolution: 3, offset: 20 },
                    abs_pos: true, abs_ext: false
                }
            });
        }

        INSTANCE.as_mut().unwrap()
    }
}

static mut INSTANCE: Option<Printer> = None;

pub struct Settings {
    pub x_coder: Encoder,
    pub y_coder: Encoder,
    pub z_coder: Encoder,
    pub abs_pos: bool,
    pub abs_ext: bool,
}

pub struct Encoder {
    pub resolution: u16,
    pub offset: i16,
}

impl Encoder {
    fn postion(&self, mm: i16) -> u16 {((mm+self.offset) as u16)*self.resolution}
    fn mm(&self, position: u16) -> i16 {(position/self.resolution) as i16 - self.offset}
}