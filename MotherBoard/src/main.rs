mod printer;
mod network;

use std::time::Duration;
use tokio::{time::sleep, join};


#[tokio::main]
async fn main() {
    
    join!(printer::get().initialize(), network::get().initialize());
    

    printer::get().load_file("sphere_0.15mm_PLA_MK3S_1h0m.gcode");
    // printer::get().load_file("test.gcode");
    printer::get().print().await;

    loop { sleep(Duration::from_secs(10)).await; }
}
