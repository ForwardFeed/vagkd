use std::fs::File;
use serde::{Deserialize};

//This will be for general parameters of the program not macro specific
//
#[derive(Clone, Debug, Deserialize)]
pub struct GeneralParameters {
    pub(crate)event_path: String,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum KeyStates{//Was really fun to do it but was it the simplest?
SpamPress{spam_press_time_span: u64, repetition:u16 },
    LongPress{press_duration: u64},
    Simple{key_value: i32},
}

//this struct is just for the simple keycode and keystate, the keystate is comparable to a keyfunction
#[derive(Clone, Debug, Deserialize)]
pub struct CfgSubKeybind {
    pub(crate) key_code: u16,
    //pub(crate) keybind_type: String,
    pub(crate) key_state: KeyStates,
}

// this will be a collection of couple keycode and keystate
#[derive(Clone, Debug, Deserialize)]
pub struct CfgKeybind {
    pub(crate) sub_keybinds: Vec<CfgSubKeybind>,//
    pub(crate) name: String,
    pub(crate) timer_threshold: u64,
}

//this is a final unified struct that will splitted in piece after in the threadlauncher
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub(crate) general_parameters: GeneralParameters,
    pub(crate) keybinds: Vec<CfgKeybind>,
}



//exit here don't feel being a valid use
pub fn new(file: File) -> Config {
    return  match serde_json::from_reader(file) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(2);
        }
    };
}