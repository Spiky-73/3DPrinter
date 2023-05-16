use std::collections::VecDeque;

mod gcode;
mod serial;

pub const BUFFERED_INSTRUCTIONS: u32 = 5;

impl Printer {

    pub async fn initialize(&mut self){
        self.start_home();

        while self.state == State::Initializing {}
        
    }
    
    pub async fn print(&mut self){
        self.start_print();
        while self.state == State::Printing {}
    }

    pub fn start_home(&mut self) {
        match self.state {
            State::Initializing => return,
            State::Idle => {
                self.state = State::Initializing;
                self.instructions.clear()

                // start the serial listenning
                // reset the sensor and actuators
            }
            _ => {
                // TODO cannot print
            }
        }
    }


    pub fn load_gcode(&mut self, code: &str){
        // ? converts the gcode to motor instructions now of when the command is sent
        self.instructions.clear();
        code.split('\n').for_each(|line| self.instructions.push_back(line.parse().unwrap()));
    }

    pub fn start_print(&mut self){
        if self.state != State::Idle || self.instructions.len() == 0 { return }
        self.state = State::Printing;
        
        for _ in 0..BUFFERED_INSTRUCTIONS {
            if self.instructions.len() == 0 { break; }
            let instruction = self.instructions.pop_front().unwrap();
            self.send_instruction(instruction);
        }
    }

    pub fn stop_print(&mut self){
        self.state = State::Idle;
    }


    fn on_command_completion(&mut self){
        if self.instructions.len() == 0 {
            self.state = State::Idle;
        } else if self.state == State::Printing {
            let instruction = self.instructions.pop_front().unwrap();
            self.send_instruction(instruction);
        }
    }

    fn send_instruction(&mut self, instruction: gcode::Instruction){
        // TODO
        // parses the instruction 
        // send the data on the serial
    }
}

pub struct Printer {
    state: State,
    instructions: VecDeque<gcode::Instruction>,
}

pub fn get() -> &'static mut Printer {
    return unsafe { &mut INSTANCE }
}

static mut INSTANCE: Printer = Printer { state:State::Initializing, instructions:VecDeque::new() };

#[derive(PartialEq, Eq)]
enum State {

    Initializing,
    Idle,
    Printing,
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