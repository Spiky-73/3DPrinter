use super::super::Settings;
use super::{Command, ParseGcodeError, Field, scope};

pub struct G0 { data: Vec<u8> }
impl Command for G0 {
    fn new(fields: &[Field], setting: &Settings) -> Result<Self, ParseGcodeError> where Self: Sized {
        let mut data: Vec<u8> = Vec::new();
        for field in fields {
            if let Some(value) = field.number {
                match field.letter {
                    'X' => {
                        data.push(b'X');
                        data.extend(setting.x_coder.postion(value as i16).to_le_bytes());
                    }
                    'Y' => {
                        data.push(b'Y');
                        data.extend(setting.y_coder.postion(value as i16).to_le_bytes());
                    }
                    'Z' => {
                        data.push(b'Z');
                        data.extend(setting.z_coder.postion(value as i16).to_le_bytes());
                    }
                    _ => {}
                }
            } else { return Err(ParseGcodeError) }
        }

        Ok(G0{data})
    }
    fn scope(&self) -> u8 { scope::ARDUINO }
    fn data_arduino(&self) -> &Vec<u8> { &self.data }
}

pub struct G4 { data: Vec<u8> }
impl Command for G4 {
    fn new(fields: &[Field], _: &Settings) -> Result<Self, ParseGcodeError> where Self: Sized {
        let mut pause: u32 = 0;
        for field in fields {
            if let Some(value) = field.number {
                match field.letter {
                    'P' => { pause += value as u32}
                    'S' => { pause += (value as u32)*1000}
                    _ => {}
                }
            } else { return Err(ParseGcodeError) }
        }
        
        let mut data: Vec<u8> = Vec::new();
        data.push(b'P');
        data.extend(pause.to_le_bytes());
        Ok(G4{data})
    }
    fn scope(&self) -> u8 { scope::ARDUINO }
    fn data_arduino(&self) -> &Vec<u8> { &self.data }
}

pub struct G28 { data: Vec<u8> }
impl Command for G28 {
    fn new(fields: &[Field], _: &Settings) -> Result<Self, ParseGcodeError> where Self: Sized {
        let mut data: Vec<u8> = Vec::new();
        data.push(b'H');
        let mut val: u8 = 0b000;
        for field in fields {
            if field.number.is_none() {
                match field.letter {
                    'X' => {val |= 0b001;}
                    'Y' => {val |= 0b010;}
                    'Z' => {val |= 0b100;}
                    _ => {}
                }
            } else { return Err(ParseGcodeError) }
        }
        if val == 0 {val = 0b111};
        data.push(val);
        Ok(G28{data})
    }
    fn scope(&self) -> u8 { scope::ARDUINO }
    fn data_arduino(&self) -> &Vec<u8> { &self.data }
}

pub struct G92 { data: Vec<u8> }
impl Command for G92 {
    fn new(fields: &[Field], setting: &Settings) -> Result<Self, ParseGcodeError> where Self: Sized {
        let mut data: Vec<u8> = Vec::new();
        for field in fields {
            if let Some(value) = field.number {
                match field.letter {
                    'X' => {
                        data.push(b'x');
                        data.extend(setting.x_coder.postion(value as i16).to_le_bytes());
                    }
                    'Y' => {
                        data.push(b'y');
                        data.extend(setting.y_coder.postion(value as i16).to_le_bytes());
                    }
                    'Z' => {
                        data.push(b'z');
                        data.extend(setting.z_coder.postion(value as i16).to_le_bytes());
                    }
                    _ => {}
                }
            } else { return Err(ParseGcodeError) }
        }
        Ok(G92{data})
    }
    fn scope(&self) -> u8 { scope::ARDUINO }
    fn data_arduino(&self) -> &Vec<u8> { &self.data }
}