use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Command { G(u16), M(u16) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseFieldError;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseGcodeError;


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
        
        let value: Option<f32> = match number {
            "" => None::<f32>,
            _ => Some(number.parse().map_err(|_| ParseFieldError)?)
        }; 
        
        Ok(Field{letter, number: value})
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub command: Command,
    pub params: HashMap<char, Option<f32>>
}


impl FromStr for Instruction {
    type Err = ParseGcodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, params) = s.split(';').nth(0).ok_or(ParseGcodeError)?.split_once(" ").unwrap_or((s, ""));
            
        let command: Field = command.parse().map_err(|_| ParseGcodeError)?;
        let number = command.number.ok_or(ParseGcodeError)? as u16;
        let command = match command.letter {
            'G' => Command::G(number),
            'M' => Command::M(number),
            _ => return Err(ParseGcodeError)
        };
        let params: HashMap<char, Option<f32>> = match params {
            "" => HashMap::new(),
            _ => {
                let mut map : HashMap<char, Option<f32>> = HashMap::new();
                for param in params.split_whitespace() {
                    let field: Field = param.parse().map_err(|_| ParseGcodeError)?;
                    map.insert(field.letter, field.number);
                }
                map
            },
        };
        Ok(Instruction { command, params })
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