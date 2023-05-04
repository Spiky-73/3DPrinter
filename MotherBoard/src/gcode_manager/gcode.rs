use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    G(u16),
    M(u16)
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCommandError;

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (code, id) = s.to_uppercase().split_at(1);
        let id: u16 = id.parse()?.or(Err(ParseCommandError));

        match code {
            "G" => Ok(Command::G(id)),
            "M" => Ok(Command::M(id)),
            _ => Err(ParseCommandError)
        }
    }
}

pub enum Arg {
    X(f32), Y(f32), Z(f32),
    E(f32), F(f32), S(f32),
    NotImplementedYet(String)
}

pub struct GCode {
    command: Command,
    args: HashMap<char, f32>
}

pub fn parse_line(line: &str) -> GCode {
    let (command, args) = line.split(';').nth(0).unwrap().split_once(" ").unwrap();
    
    let command = Command::new(command);

    let args: Vec<(&str, &str)> = args
        .split_whitespace()
        .map(|arg| (arg.chars().next().unwrap(), arg[1..].parse().unwrap()))
        .collect();

    let args = args
        .iter()
        .map(|(letter, number)| ())
    let args = args.iter().map


    GCode { command, args }
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