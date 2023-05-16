
impl Network {

    pub async fn initialize(&mut self){
        // TODO impl
        // start wifi server
        // register packet handlers
        //     - gcode reception
        //     - print start / stop / pause
        //     - ...
    }
    
}

pub struct Network {
}

pub fn get() -> &'static mut Network {
    return unsafe { &mut INSTANCE }
}

static mut INSTANCE: Network = Network { };
