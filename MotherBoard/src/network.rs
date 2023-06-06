mod webui;

use crate::printer::{self, State};

impl Network<'_> {

    pub fn get_printer_state_ref(&self) -> &State {
        (self.state_getter)()
    }

    pub fn load_printer_gcode(&self, code: &str) {
        (self.gcode_setter)(code)
    }

    pub async fn initialize(&mut self) {
        // TODO impl
        // start wifi server
        // register packet handlers
        //     - gcode reception
        //     - print start / stop / pause
        //     - ...
        webui::launch().await;
    }
    
}

pub struct Network<'a> {
    state_getter: &'a dyn Fn() -> &'a State,
    gcode_setter: &'a dyn Fn(&str)
}

pub fn get() -> &'static mut Network<'static> {
    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(Network {
                state_getter: &|| {printer::get().get_state_ref()},
                gcode_setter: &|code: &str| {printer::get().load_gcode(code)}
            })
        }

        INSTANCE.as_mut().unwrap()
    }
}

static mut INSTANCE: Option<Network> = None;