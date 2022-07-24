use std::fs::File;
use ron::de::from_reader;
use serde::{Deserialize,Serialize};

//This will be for general parameters of the program not macro specific
//
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeneralParameters {
    pub(crate)event_path: String,
}


#[derive(Deserialize, Debug, Clone,Serialize)]
#[serde(untagged)]
pub enum KeyStates{//Was really fun to do it but was it the simplest?
    SpamPress{keybind_type: String, spam_press_time_span: u64, repetition:u16 },
    LongPress{keybind_type: String, press_duration: u64},
    Simple{keybind_type: String},
}

//this struct is just for the simple keycode and keystate, the keystate is comparable to a keyfunction
#[derive(Clone, Debug, Deserialize,Serialize)]
pub struct CfgSubKeybind {
    pub(crate) key_code: u16,
    pub(crate) key_state: KeyStates,
}

// this will be a collection of couple keycode and keystate
#[derive(Clone, Debug, Deserialize,Serialize)]
pub struct CfgKeybind {
    pub(crate) sub_keybinds: Vec<CfgSubKeybind>,//
    pub(crate) name: String,
    pub(crate) timer_threshold: u64,
}

//this is a final unified struct that will splitted in piece after in the threadlauncher
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    //pub(crate) general_parameters: GeneralParameters,
    pub(crate) general_parameters: GeneralParameters,
    pub(crate) keybinds: Vec<CfgKeybind>,
}



//i could have worked to make this config loader not cracking itself when something is wrong but rather skip it, more user friendly you know
pub fn new(config_file: &str) -> Config {

    let f = File::open(&config_file).expect("Failed opening file");
    return  match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
}
