use futures::{join, executor::block_on};

mod printer;
mod network;

fn main() {
    // printer::run_gcode_tests();

    block_on(initialize());

    loop {
        // TODO
        // broadcast printer info via wifi 
        //     - print status
        //     - sensor status    
    }
}

async fn initialize(){
    join!(printer::get().initialize(), network::get().initialize());
}