use std::collections::HashMap;

pub enum Command {
    G(u16),
    M(u16)
}

impl Command {
    pub fn new(command: &str) -> Self {
        let id: u16 = command[1..].parse().unwrap();
        match command.chars().nth(0) {
            Some('G') => Command::G(id),
            Some('M') => Command::M(id),
            _ => panic!("Unknwon command {command}")
        }
    }
}

pub struct GCode {
    command: Command,
    args: HashMap<char, f32>
}

pub fn parse(line: &str) -> GCode {
    let mut line: Vec<&str> = line.split(';').nth(0).unwrap().split_whitespace().collect();

    let command = Command::new(line.remove(0));    
    let mut args = line.iter().map(|l| (l.chars().nth(0).unwrap(), l[1..].parse().unwrap())).collect::<HashMap<char, f32>>();
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