use std::str::FromStr;

use super::{Settings, PrintStatus};

mod mcommands;
mod gcommands;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseFieldError;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseGcodeError;

pub mod scope {
    pub const CONFIG: u8 = 0b0001;
    pub const PI: u8 = 0b0010;
    pub const ARDUINO: u8 = 0b0100;
}

pub struct Void;
impl Command for Void {
    fn new(_: &[Field], _: &Settings) -> Result<Self, ParseGcodeError> where Self: Sized { Ok(Void) }
}

pub struct Unhandled{
    letter: char,
    number: u16,
}
impl Command for Unhandled {
    fn new(fields: &[Field], _: &Settings) -> Result<Self, ParseGcodeError> where Self: Sized {
        Ok(Unhandled { letter: fields[0].letter, number: fields[0].number.unwrap() as u16 })
    }
    fn scope(&self) -> u8 { scope::CONFIG }
    fn edit_config(&self, _: &mut Settings) {
        println!("Unhandled command {}{}", self.letter, self.number)
    }
}

static EMPTY: Vec<u8> = Vec::new();

pub trait Command {
    fn new(fields: &[Field], setting: &Settings) -> Result<Self, ParseGcodeError> where Self: Sized;
    
    fn scope(&self) -> u8 {0}

    fn edit_config(&self, settings: &mut Settings) {}
    fn data_arduino(&self) -> &Vec<u8> { &EMPTY }
    fn run_pi(&self, status: &mut PrintStatus) {}
}

pub fn parse(code: &str, setting: &Settings) -> Result<Box<dyn Command>, ParseGcodeError> {
    let code = code.split(';').nth(0).ok_or(ParseGcodeError)?.trim();

    if code == "" { return Ok(Box::new(Void)) }
    let (label, params) = code.split_once(" ").unwrap_or((code, ""));
    
    let params: Vec<Field> = match params {
        "" => Vec::new(),
        _ => params.split_whitespace().map(|f| f.parse::<Field>().map_err(|_| ParseGcodeError)).collect::<Result<_,_>>()?,
        
    };
    return match label {
        "G0" | "G1" => Ok(Box::new(gcommands::G0::new(&params, setting)?)),
        "G4"=> Ok(Box::new(gcommands::G4::new(&params, setting)?)),
        "G28"=> Ok(Box::new(gcommands::G28::new(&params, setting)?)),
        "G92"=> Ok(Box::new(gcommands::G92::new(&params, setting)?)),
        "M73" => Ok(Box::new(mcommands::M73::new(&params, setting)?)),
        "M84" => Ok(Box::new(mcommands::M84::new(&params, setting)?)),
        "G90" | "M140" => Ok(Box::new(Void)), // simple bed
        "M104" | "M109" | "M106" | "M107" | "M900"=> Ok(Box::new(Void)), // no extruder yet
        "G21" | "G80" | "M83" | "M201" | "M203" | "M204" | "M190"=> Ok(Box::new(Void)), // no settings
        "M115" | "M862" | "M907" | "M221" => Ok(Box::new(Void)), // dont know
        _ => Err(ParseGcodeError)
    };
}


pub struct Field {
    letter: char,
    number: Option<f32>
}


impl FromStr for Field {
    type Err = ParseFieldError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (letter, number) = s.split_at(1);
        let letter = letter.chars().nth(0).unwrap();
        if !letter.is_ascii_uppercase() {return Err(ParseFieldError)}
        
        let number: Option<f32> = match number {
            "" => None::<f32>,
            _ => Some(number.parse().map_err(|_| ParseFieldError)?)
        }; 
        
        Ok(Field{letter, number})
    }
}


/* Codes for a sphere
M73,83,84,93,104,106,107,109,115,140,190,201,203,204,205,221,862,907,900
G1,4,21,28,80,90,92



G0: rapid move, G1: interpolation lineaire
    - Xnnn, Ynnn, Znnn: position mm
    - Ennn: qty extruded mm
    - Fnnn: speed mm/min

G4: Pause
    - Snnn: wait sec

G21: set units to mm

G28: return to home
    - X Y Z: home specific axis

G80: bed level

G90: set to abs positioning

G92: define position
    - X Y Z E

M73: Set/Get build percentage
 - Pn Rn Q S (C D) 

M83: Set extruder to relative mode

M84: Stop idle hold

M93: Send axis_steps_per_unit
 - ???

M104: Set Extruder Temperature
 - C Dn Sn rN

M106: Fan On

M107: Fan Off

M109: Set Extruder Temperature and Wait

M115: Get Firmware Version and Capabilities

M140: Set Bed Temperature (Fast)

M190: Wait for bed temperature to reach target temp

M201: Set max acceleration

M203: Set maximum feedrate (Firmware dependant)
 - Pn Tn

M205: Advanced settings (Firmware dependant)

M221: Set extrude factor override percentage
 - Sn Dn

M862: Print checking

M907: Set digital trimpot motor

M900 Set Linear Advance Scaling Factors
 - Kn Rn Wn Hn Dn
*/