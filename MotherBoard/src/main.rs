mod gcode_manager;

fn main() {
    gcode_manager::run_tests();
}

/*
-> reception de gcode par wifi
-> impression
    -> conversion de gcode en instruction moteur
    -> connection serie
 */