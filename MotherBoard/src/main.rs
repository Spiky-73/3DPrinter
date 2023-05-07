mod printer;

fn main() {

    printer::run_gcode_tests();
    
    /*
    (init)
    start wifi server
    register packet handlers
        - gcode reception
        - print start / stop / pause
        - ...
    */

    // printer::get().initialize();

    /*
    (loop)
    broadcast printer info via wifi 
        - print status
        - sensor status    
    */
}

/*
-> reception de gcode par wifi
-> impression
    -> conversion de gcode en instruction moteur
    -> connection serie
 */