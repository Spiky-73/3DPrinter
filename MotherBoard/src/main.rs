use tokio::join;

mod printer;
mod network;

#[tokio::main]
async fn main() {
    
    // printer::run_gcode_tests();

    join!(printer::get().initialize(), network::get().initialize());

    loop {
        // TODO impl
        // broadcast printer info via wifi 
        //     - print status
        //     - sensor status    
    }
}