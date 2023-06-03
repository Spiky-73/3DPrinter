use std::{cmp::min, str, time::Duration, thread, fs::File, io::Read};

use tokio::time;
mod gcode;

pub const BUFFERED_INSTRUCTIONS: usize = 5;
pub const PORT: &str = "/dev/serial0";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initializing,
    Idle,
    Printing(PrintStatus),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PrintStatus {
    pub sent: usize,
    pub completed: usize,
    pub progess: u8,
    pub time_left: u16,
    pub silent_progess: u8,
    pub silent_time_left: u16,
}
impl Printer {

    pub async fn initialize(&mut self){
        println!("Setting up printer...");
        self.listen();
        time::sleep(time::Duration::from_millis(100)).await;
        self.home().await;
        println!("Printer ready !")
    }

    pub async fn home(&mut self) {
        match self.state {
            State::Initializing => return,
            State::Idle => {
                println!("Initialising controller...");
                _ = self.serial.write(&[1, b'H']);
                self.instructions.clear();
                self.state = State::Initializing;
                while self.state == State::Initializing {
                    time::sleep(time::Duration::from_secs(2)).await;
                }
                println!("Controller initialised");
            }
            _ => {
                // TODO cannot init
            }
        }
    }

    pub const fn get_state(&self) -> State { self.state }


    pub fn load_file(&mut self, path: &str){
        println!("Loading for file \"{path}\"...");
        let mut file = File::open(path).unwrap();
        let mut code = String::new();
        _ = file.read_to_string(&mut code);
        self.load_gcode(&code);
    }
    
    pub fn load_gcode(&mut self, code: &str){ // Not supported: M862.3, M862.1
        println!("Parsing gcode...");
        self.instructions.clear();
        for line in code.split("\n") {
            let line = line.strip_suffix('\r').unwrap_or(line);
            let inst: gcode::Instruction = line.parse().unwrap();
            if inst.command != gcode::Command::None { self.instructions.push(inst); }
        }
        println!("Sucess!");
    }

    pub async fn print(&mut self){
        if self.state != State::Idle || self.instructions.len() == 0 {
            println!("Cannot start a print");
            return
        }

        self.state = State::Printing(PrintStatus { sent: 0, completed: 0, progess: 0, time_left: 0, silent_progess: 0, silent_time_left: 0 });
        
        println!("Printing...");
        for _ in 0..min(BUFFERED_INSTRUCTIONS, self.instructions.len()) {
            self.send_next_instruction();
        }
        
        while let State::Printing(_) = self.state { time::sleep(time::Duration::from_secs(2)).await; }
        println!("Print finished !");
    }

    pub fn cancel_print(&mut self){
        self.instructions.clear();
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
                } else if let State::Printing(ref mut status) = self.state { // print in progress
                    status.completed += 1;
                    if status.sent < self.instructions.len() { self.send_next_instruction(); }
                    else if status.completed == self.instructions.len() { self.state = State::Idle; }
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

    fn send_next_instruction(&mut self){
        let State::Printing(ref mut status) = self.state else { unreachable!()};
        let instruction: &gcode::Instruction = self.instructions.get(status.sent).unwrap();
        
        let mut data: Vec<u8> = Vec::new();
        match instruction.command {
            gcode::Command::G(0 | 1) => {
                let mut try_add_motor_position = |axis: u8, res: u16, offset: u16| {
                    if instruction.params.contains_key(&(axis as char)){
                        if let Some(mm) = instruction.params.get(&(axis as char)).unwrap() {
                            let position = (*mm * (res as f32) + (offset as f32)) as u16;
                            data.push(axis);
                            data.extend(position.to_le_bytes());
                        }
                    }
                };
                try_add_motor_position(b'X', 3, 0);
                try_add_motor_position(b'Y', 3, 0);
                try_add_motor_position(b'Z', 3, 0);
                // TODO simultaneous F and XYZ
                // TODO implement E F H R S
            }
            gcode::Command::G(4) => {
                let mut pause: u32 = 0;
                if let Some(sec) = instruction.params.get(&'S').unwrap_or(&None) { pause += (*sec as u32)*1000; }
                if let Some(ms) = instruction.params.get(&'P').unwrap_or(&None) { pause += *ms as u32; }
                data.push(b'P');
                data.extend(pause.to_le_bytes())
            }
            gcode::Command::G(28) => {
                let all = instruction.params.len() == 0;
                let mut try_reset = |axis: u8| {
                    if all != instruction.params.contains_key(&(axis as char)) {
                        data.push(axis);
                        data.extend((0 as u16).to_le_bytes());
                    }
                };
                try_reset(b'X');
                try_reset(b'Y');
                try_reset(b'Z');
                // ? impl
            }
            gcode::Command::G(92) => {

                let all = instruction.params.len() == 0;
                let mut try_force_position = |axis: u8, res: u16, offset: u16| {
                    if all {
                        data.push(axis+32);
                        data.extend((0 as u16).to_le_bytes());
                    } else if instruction.params.contains_key(&(axis as char)) {
                        if let Some(mm) = instruction.params.get(&(axis as char)).unwrap() {
                            let position = (*mm * (res as f32) + (offset as f32)) as u16;
                            data.push(axis+32);
                            data.extend(position.to_le_bytes());
                        }
                    }
                };
                try_force_position(b'X', 3, 0);
                try_force_position(b'Y', 3, 0);
                try_force_position(b'Z', 3, 0);
            }
            gcode::Command::G(21 | 80 | 90) => {} // Does nothing but supported
            
            gcode::Command::M(73) => {
                let mut silent = true;

                if let Some(percent) = instruction.params.get(&'P'){
                    if let Some(percent) = percent {
                        silent = false;
                        status.progess = *percent as u8;
                        if status.progess == 0 { println!("Build start"); }
                    }
                }
                if let Some(time) = instruction.params.get(&'R'){
                    if let Some(time) = time {
                        silent = false;
                        status.time_left = *time as u16;
                    }
                }
                if let Some(percent) = instruction.params.get(&'Q'){
                    if let Some(percent) = percent {
                        status.silent_progess = *percent as u8;
                    }
                }
                if let Some(time) = instruction.params.get(&'S'){
                    if let Some(time) = time {
                        status.silent_time_left = *time as u16;
                    }
                }
                if !silent { println!("{}% done ({} min remaining)", status.progess, status.time_left)};
                if !silent && status.progess == 100 { println!("Build start"); }

            }
            gcode::Command::M(104) => {
                // TODO impl
            }
            gcode::Command::M(109) => {
                // TODO impl
            }
            gcode::Command::M(201 | 203 | 204) => {
                // TODO impl
            }
            gcode::Command::M(900) => {
                // TODO impl
            }
            gcode::Command::M(83 | 84 | 106 | 107 | 115 | 140 | 190 | 862 | 907) => {} // Does nothing but supported
            _ => {} // Non supported commands, includes M93, M205, M221
        };
        status.sent+=1;
        if data.len() != 0 {
            _ = self.serial.write_all(&[data.len() as u8]);
            _ = self.serial.write_all(&data);
        } else {
            self.handle_data(&[b'_']);
        }
    }
}

pub struct Printer {
    state: State,
    instructions: Vec<gcode::Instruction>,
    serial: Box<dyn serialport::SerialPort>
}

pub fn get() -> &'static mut Printer {
    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some( Printer {
                    state:State::Idle, instructions:Vec::new(),
                    serial: serialport::new(PORT, 115_200).timeout(Duration::from_millis(10)).open().expect("Failed to open port")
            } );
        }
        return INSTANCE.as_mut().unwrap();
    }
}
static mut INSTANCE: Option<Printer> = None;

pub fn run_gcode_tests() {

    let gcodes_tests = [
        // Handled
        "G1 X20 Y-.5 ; a comment",
        "G1 X",
        "G1",
        "4 X20",
        "H1 X20 Y520",
        "; a comment",
        "",

        // TODO Unhandled
        "G7.5",

    ];

    for test in gcodes_tests{
        let code = test.parse::<gcode::Instruction>();
        println!("{code:?}");
    }
}