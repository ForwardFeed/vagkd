use std::fs::File;
use ron::de::from_reader;
use serde::Deserialize;

//This will be for general parameters of the program not macro specific
// ### to much of a hassle i'll forget about it for now ###
/*#[derive(Copy, Debug, Deserialize)]
pub struct GeneralParameters {
    pub(crate) buffer_size_for_events: u16, //might be useful, might be removed, who knows
}*/

//The Master Keybind, very important to me when doing macros, having your computer going crazy is funny but a fearfull experience.
//So this act like a safe net, just don't forget the key you put as a red button
#[derive(Clone, Debug, Deserialize)]
pub struct CfgBarman {
    pub(crate) event_path: String,
    pub(crate)freeze_key_code: u16, //Global freeze, all macro
    pub(crate)freeze_key_state: String,
    pub(crate)reload_key_code: u16, //reload all macros
    pub(crate)reload_key_state: String,
    pub(crate)shutdown_key_code: u16, //quit the whole program, all process are killed
    pub(crate)shutdown_key_state: String,
}

//this struct is just for the simple keycode and keystate, the keystate is comparable to a keyfunction
#[derive(Clone, Debug, Deserialize)]
pub struct CfgSubKeybind {
    pub(crate) key_code: u16,
    pub(crate) key_state: String,
}

// this will be a collection of couple keycode and keystate
#[derive(Clone, Debug, Deserialize)]
pub struct CfgKeybind {
    pub(crate) sub_keybinds: Vec<CfgSubKeybind>,//
    pub(crate) adr_name: String,
    pub(crate) timer_treshold: i64,
}

//this is a final unified struct that will splitted in piece after in the macro_decompositor
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    //pub(crate) general_parameters: GeneralParameters,
    pub(crate) barman: CfgBarman,
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