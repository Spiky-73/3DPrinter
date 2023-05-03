mod gcode;

struct GcodeManager{
    pub Printing: bool
    _gcode: Vec<gcode::GCode>
}

impl GcodeManager {
    pub fn new() -> Self{
        GcodeManager{ _gcode: Vec::new() }
    }

    pub fn load_gcode(&mut self, code: &str){
        self._gcode.clear();
        for line in code.split('\n'){
            self._gcode.push(gcode::parse(line));
        }
    }
    
    pub async fn start_printing(&mut self){
        if(self._gcode.len() == 0){
            print!("No gcode to print, please load gcode fisrt.");
            return;
        }
        // send the x fist intruction to be buffered
    }

    pub fn on_command_end(&mut self){
        if(self._gcode.len() > 0){

        }
    }
}



/*
wifi
gcode
impr
*/

// gcode reception ant storage
// send packets to the printer 


// event on task completion by the arduino

/* Workflow
receive gcode
send X fisrt instruction
 -> convert gcode to motor rotations / speed / ...

on instruction complete send the next to be buffered
when all instruction are sent and done, finished
 */