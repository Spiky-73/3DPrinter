use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Command { G(u16), M(u16) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseFieldError;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseGcodeError;

impl FromStr for Command {
    type Err = ParseFieldError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (code, id) = s.split_at(1);
        let id: u16 = id.parse().map_err(|_| ParseFieldError)?;
        match code {
            "G" => Ok(Command::G(id)),
            "M" => Ok(Command::M(id)),
            _ => Err(ParseFieldError)
        }
    }
}

#[derive(Debug)]
pub struct GCode {
    pub command: Command,
    pub params: HashMap<char, Option<f32>>
}


impl FromStr for GCode {
    type Err = ParseGcodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, params) = s.split(';').nth(0).ok_or(ParseGcodeError)?.split_once(" ").unwrap_or((s, ""));
            
        let command: Command = command.parse().map_err(|_| ParseGcodeError)?;

        let params: HashMap<char, Option<f32>> = match params {
            "" => HashMap::new(),
            _ => {
                params.split_whitespace()
                    .map(|p| Ok(parse_field(p).map_err(|_| ParseGcodeError)?))
                    .collect::<Result<HashMap<char, Option<f32>>, ParseGcodeError>>()?
            }
        };
        Ok(GCode { command, params })
    }
}

fn parse_field(s: &str) -> Result<(char, Option<f32>), ParseFieldError> {
    let (code, number) = s.split_at(1);
    let code = code.chars().nth(0).unwrap();
    if !code.is_ascii_uppercase() {return Err(ParseFieldError)}
    
    let number: Option<f32> = match number {
        "" => None::<f32>,
        _ => Some(number.parse().map_err(|_| ParseFieldError)?)
    }; 
    
    Ok((code, number))
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
    - X, Y, Z: home specific axis

G80: bed level

G90: set to abs positioning

G92: define position
    - X, Y, Z, E
*/