
use crate::{internal_coms, barman, manager};
use std::thread;
use crate::config_loader::{Config};

/**
    Takes the config and launch every thread who will then intercommunicate
**/
pub fn start(config: Config){//TODO make error handling
    let keybinds= config.keybinds;
    let barman = config.barman;
    let barman_coms = internal_coms::BarmanComs::new().generate_arc_link();
    let barman_coms_for_keybinds = barman_coms.clone();

    // launch the macro manager who will launch the threads beneath him
    keybinds.into_iter().for_each(|keybind|{
        let arc_link = barman_coms_for_keybinds.clone();
        let name = format!("Keybind_manager_{}", keybind.adr_name);
        thread::Builder::new().name(name).spawn(move || {  manager::new(keybind, arc_link) }).unwrap();
    });

    //lets launch the Barman
    barman::new(barman, barman_coms).start();
}


