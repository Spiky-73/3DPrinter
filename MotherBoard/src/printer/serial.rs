use std::io::IoSlice;
use std::{time::Duration, collections::HashMap};

use std::thread;

use serialport::{self, SerialPort};


impl Serial {
    
    pub fn new(port: &str, baud: u32) -> Self {
        Serial { 
            port: serialport::new(port, baud).timeout(Duration::from_millis(10)).open().expect("Failed to open port"), 
            handlers: HashMap::new(),
        }
    }
    
    pub fn register_handler(&mut self, packet_id: u8, handler: fn(&[u8])) { self.handlers.insert(packet_id, handler); } 
    
    pub fn listen(&mut self) { 
        thread::spawn(move || {
            let mut buf = [0 as u8; 256];
            loop {
                let n = self.port.read(&mut buf);
                match self.handlers.get(&buf[0]) {
                    Some(f) => f(&buf[1..]),
                    None => (),
                }
            }
        });
    }

    pub fn send(&mut self, data: &[u8]) {
        self.port.write_vectored(&[IoSlice::new(&[data.len() as u8]), IoSlice::new(&data)]);
    }
}
pub struct Serial {
    port: Box<dyn SerialPort>,
    handlers: HashMap<u8, fn(&[u8])>,
}