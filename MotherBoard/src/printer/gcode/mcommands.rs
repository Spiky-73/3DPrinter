use super::super::{Settings, PrintStatus};
use super::{Command, ParseGcodeError, Field, scope};

pub struct M73 {
    progress: u8,
    time: u16,
    silent: bool,
}
impl Command for M73 {
    fn new(fields: &[Field], setting: &Settings) -> Result<Self, ParseGcodeError> where Self: Sized {
        let mut progress: u8 = 0;
        let mut time: u16 = 0;
        let mut silent: bool = false;
        for field in fields {
            if let Some(number) = field.number {
                match field.letter {
                    'P' => { progress = number as u8; }
                    'R' => { time = number as u16; }
                    'Q' => { progress = number as u8; }
                    'S' => {
                        time = number as u16;
                        silent = true;
                    }
                    _ => {}
                }
            } else {
                return Err(ParseGcodeError)
            }
        }
        Ok(M73 { progress, time, silent})
    }
    fn scope(&self) -> u8 { scope::CONFIG | scope::PI }
    fn edit_config(&self, settings: &mut Settings) {
        if self.progress%20 != 0 {return;}
        if !self.silent { println!("{}% done", self.progress); }
    }
    fn run_pi(&self, state: &mut PrintStatus) {
        if self.silent {
            state.silent_progress = self.progress;
            state.silent_time_left = self.time;
        } else {
            state.progress = self.progress;
            state.time_left = self.time;
            println!("{}% done ({} min remaining)", self.progress, self.time);
        }
    }
}
