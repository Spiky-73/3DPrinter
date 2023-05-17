use std::{collections::VecDeque, cmp::min};

use tokio::time;

mod gcode;
mod serial;

#[repr(u8)]
pub enum Packets {
    StepDone = 1,
    HomeDone,
    SendHome,
    SendCommand,
    SendStep,
}

pub const BUFFERED_INSTRUCTIONS: u32 = 5;

impl Printer {

    pub async fn initialize(&mut self){
        self.serial.register_handler(Packets::StepDone as u8, on_command_completion);
        self.serial.register_handler(Packets::HomeDone as u8, on_home_completion);
        self.serial.listen();
        self.home().await;
    }
    
    pub async fn home(&mut self) {
        match self.state {
            State::Initializing => return,
            State::Idle => {
                self.instructions.clear();
                self.state = State::Initializing;
                self.serial.send(Packets::SendHome as u8, None);
                while self.state == State::Initializing { time::sleep(time::Duration::from_secs(2)).await; }
            }
            _ => {
                // TODO cannot print
            }
        }
    }

    pub async fn print(&mut self){
        if self.state != State::Idle || self.instructions.len() == 0 { return }
        
        self.state = State::Printing(0);
        
        for _ in 0..min(BUFFERED_INSTRUCTIONS, self.instructions.len() as u32) {
            let instruction = self.instructions.pop_front().unwrap();
            self.send_instruction(instruction);
        }

        while let State::Printing(i) = self.state { time::sleep(time::Duration::from_secs(2)).await; }
    }


    pub fn load_gcode(&mut self, code: &str){
        // ? converts the gcode to motor instructions now of when the command is sent
        self.instructions.clear();
        code.split('\n').for_each(|line| self.instructions.push_back(line.parse().unwrap()));
    }

    pub fn cancel_print(&mut self){
        self.instructions.clear();
        self.state = State::Idle;
    }


    pub fn on_command_completion(&mut self, data: &[u8]){
        if self.instructions.len() == 0 {
            self.state = State::Idle;
        } else if let State::Printing(_) = self.state {
            let instruction = self.instructions.pop_front().unwrap();
            self.send_instruction(instruction);
        }
    }

    pub fn on_home_completion(&mut self){
        self.state = State::Idle;
    }

    fn send_instruction(&mut self, instruction: gcode::Instruction){
        // TODO impl
        // parses the instruction 
        // send the data on the serial
    }
}

pub struct Printer {
    state: State,
    instructions: VecDeque<gcode::Instruction>, // TODO add a backup to allow 
    serial: serial::Serial
}

pub fn get() -> &'static mut Printer {
    return unsafe { &mut INSTANCE }
}

static mut INSTANCE: Printer = Printer { state:State::Initializing, instructions:VecDeque::new(), serial: serial::Serial::new(port, 115200)};


fn on_command_completion(data: &[u8]){ get().on_command_completion(data);}
fn on_home_completion(data: &[u8]) { get().on_home_completion();}


#[derive(PartialEq, Eq)]
enum State {

    Initializing,
    Idle,
    Printing(u32), // TODO add a struct 
    Paused
}

pub fn run_gcode_tests() {

    let gcodes_tests = [
        // Handled
        "G1 X20 Y-.5 ; a comment",
        "G1 X",
        "G1",
        "4 X20",
        "H1 X20 Y520",

        // TODO Unhandled
        "G7.5",
        "; a comment",
        "",

    ];

    for test in gcodes_tests{
        let code = test.parse::<gcode::Instruction>();
        println!("{code:?}");
    }
}