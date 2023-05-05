mod gcode;
mod serial;

pub fn init() {
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
    // checks if can print
    
    // sends the X 1st commands to the printer
    // the rest of the print is handled in on_command_completion()
}

pub fn stop_print(){
    // stop the current prints
}


fn on_command_completion(){
    // send the next command to the printer
    // checks if the print is finished
}

fn send_Intrution(instruction: gcode::Instruction){
    // parses the instruction 
    // send the data on the serial
}

static mut instructions: Vec<gcode::Instruction> = Vec::new();


pub fn run_gcode_tests() {

    let gcodes_tests = [
        "G1 X20 Y-.5 ; a comment",
        "G1 X",
        "G1",

        // Errors
        "4 X20",
        "H1 X20 Y520",
        "; a comment",
        "", // Panic

    ];

    for test in gcodes_tests{
        let code = test.parse::<gcode::Instruction>();
        println!("{code:?}");
    }
}