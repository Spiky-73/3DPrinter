mod gcode;
mod serial;

pub const BUFFERED_INSTRUCTIONS: u32 = 5;

pub fn init() {
    // TODO 
    // clear the saved data

    // reset the sensor and actuators
}

pub fn load_gcode(code: &str){
    // ? converts the gcode to motor instructions now of when the command is sent
    unsafe {
        instructions.clear();
        code.split('\n').for_each(|line| instructions.push(line.parse().unwrap()));
    }
}

pub fn start_print(){

    // TODO checks if can print (instructions loaded, printer initialized, ...)

    unsafe {
        printing = true;
        for _ in 0..BUFFERED_INSTRUCTIONS { // TODO check out of bounds if you have < 5 instructions
            send_instruction(instructions.remove(0));
        }
    }
    // the rest of the print is handled in on_command_completion()
}

pub fn stop_print(){
    // stop the current prints
    unsafe {
        printing = false;
    }
}


fn on_command_completion(){
    unsafe {
        if instructions.len() == 0 {
            printing = false;
        } else {
            send_instruction(instructions.remove(0));
        }
    }
}

fn send_instruction(instruction: gcode::Instruction){
    // TODO
    // parses the instruction 
    // send the data on the serial
}

static mut printing: bool = false;
static mut instructions: Vec<gcode::Instruction> = Vec::new();



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