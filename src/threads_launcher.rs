use std::sync::mpsc::{channel, sync_channel};
use std::sync::{Arc, Mutex};
use crate::{internal_coms, barman, manager};
use std::thread;
use ron::Value::String;
use crate::config_loader::{Config};

/**
    Takes the config and launch every thread who will then intercommunicate
**/
pub fn start(config: Config) -> std::sync::mpsc::Receiver<u32> {
    let keybinds= config.keybinds;
    let barman = config.barman;
    let barman_coms = internal_coms::BarmanComs::new().generate_arc_link();
    let barman_coms_for_keybinds = barman_coms.clone();

    let (tx, rx) = channel::<u32>();

    // launch the macro manager who will launch the threads beneath him
    keybinds.into_iter().for_each(|keybind|{
        let tx = tx.clone();
        let arc_link = barman_coms_for_keybinds.clone();
        let name = format!("Keybind_manager_{}", keybind.id);
        thread::Builder::new().name(name).spawn(move || {  manager::new(keybind, arc_link, tx) }).unwrap();
    });

    //lets launch the Barman
    let name = format!("Barman");
    thread::Builder::new().name(name).spawn(move || {  barman::new(barman, barman_coms).start() }).unwrap();
    return rx;
}


