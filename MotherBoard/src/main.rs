mod printer;
mod network;

use std::time::Duration;
use tokio::{time::sleep, join};


#[tokio::main]
async fn main() {
    
    printer::get().initialize().await;
    network::get().launch().await;
    
    // printer::get().load_file("sphere_0.15mm_PLA_MK3S_1h0m.gcode");
    // printer::get().load_file("test.gcode");
    // printer::get().print().await;
}