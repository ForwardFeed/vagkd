
use crate::{config_loader};

use std::process::exit;
use std::io::{stdin, stdout, Write};
use std::fs::File;
use std::time;
use ron::from_str;
use ron::ser::{PrettyConfig, to_string_pretty};
use crate::config_loader::{CfgKeybind, CfgSubKeybind, Config, KeyStates};
use crate::extractor::{ConfigKeyboardEventReader};


pub struct Generate{
    keyboard_input_reader: ConfigKeyboardEventReader,
    event_file_path: String,
}

impl Generate {
    pub fn new() -> Generate {
        let event_file_path = loop {
            let path = retrieve_from_io("Enter the event path (Ex. /dev/input/event3 ) :");
            match File::open(path.clone()){
                Ok(_) => break path,
                Err(e) => println!("Error while opening the event file : {}", e)
            }
        };
        Generate{
            keyboard_input_reader: ConfigKeyboardEventReader::new(event_file_path.clone()),
            event_file_path,
        }
    }

    pub fn start(mut self){
        let mut keybinds: Vec<CfgKeybind> = vec![];
        let pretty = PrettyConfig::new();

        loop {
            keybinds.push(self.create_keybind());
            match yes_no_prompt("Would you like to continue to create Keybinds to this configuration file? (Y/n) :"){
                Ok(()) => continue,
                Err(()) => break
            }
        }
        let config_generated = Config{ general_parameters: config_loader::GeneralParameters{event_path : self.event_file_path.clone()}, keybinds};
        let config_as_string = match to_string_pretty(&config_generated, pretty) {
            Ok(x) => {x}
            Err(e) => {panic!("Couldn't generate configuration file, Serialization failed: {}", e)}
        };
        write_into_file(config_as_string);
        exit(0);
    }

    fn create_keybind(&mut self) -> CfgKeybind {
        loop {
            println!("Creation of a Keybind");
            let name= retrieve_from_io("What should be the name of this keybind? : ");
            let  timer_threshold: u64 =
                loop {
                    match  from_str::<u64>(&*retrieve_from_io("Please enter the time threshold for this keybind (in milliseconds) : ")){
                        Ok(x) => break x,
                        Err(x) => println!("Please enter a positive integer: {}",x)
                    }
                };
            let mut  sub_keybinds:Vec<CfgSubKeybind> = vec![];
            loop{
                sub_keybinds.push(self.create_sub_keybind());
                match yes_no_prompt("Would you like to continue to add other key to this keybind? (Y/n) :"){
                    Ok(()) => continue,
                    Err(()) => break
                }
            }
            return CfgKeybind{sub_keybinds ,name, timer_threshold}

        }

    }

    fn create_sub_keybind(&mut self) -> CfgSubKeybind {
        print_flush("What key should be sub-keybinded? (press the key in question in 1 sec)");
        std::thread::sleep(time::Duration::from_secs(1));
        print_flush("Now");
        let key_code = self.keyboard_input_reader.last_keycode_pressed();
        println!();
        let key_state = loop {
            match &retrieve_from_io("Please enter a keystates, or anything else to print help : ") as &str{
                "simple" => {
                    break simple()
                },
                "longpress" =>{
                    break longpress()
                },
                "spampress" =>{
                    break spampress()
                },
                _=>{
                    print_help_keystates()
                }
            }
        };
        return config_loader::CfgSubKeybind{key_code,key_state}
    }
}

pub fn retrieve_from_io(msg: &str) -> String {
    print_flush(msg);
    let mut event_file_input = String::new();
    loop {
        match stdin().read_line(&mut event_file_input) {
            Ok(_) => {break}
            Err(e) => {println!("Error : {}", e)}
        }

    }
    event_file_input.pop();
    return event_file_input;
}

fn yes_no_prompt(msg: &str) -> Result<(), ()>{
    return loop {
        match &retrieve_from_io(msg) as &str {
            "N" | "n" | "no" | "No" | "NO" => { break Err(()) },
            "Y" | "y" | "yes" | "Yes" | "YES" => { break Ok(()) }
            _ => { }
        }
        print_flush("N/n/no/No/NO OR Y/y/yes/Yes/YES"  );
    }
}

fn print_flush(message: &str){
    print!("{}",message);
    stdout().flush().unwrap();
}

fn print_help_keystates(){
    println!("There is for now three type of keyStates\n\
    spampress which is when you spam a key X time in Y number of time\n\
    longpress which is when you hold a key for X time\n\
    simple    which is just when you simply press it")
}

fn simple() -> KeyStates {
    return KeyStates::Simple {keybind_type: "press".to_string()}
}
fn write_into_file(text : String){
    let mut config_file;

    loop {
        let mut name = retrieve_from_io("Writing to file? default 'macro-config.ron' :");
        if name.is_empty() {
            name = std::string::String::from("macro-config.ron")
        }

        config_file = match File::create(name.clone()) {
            Ok(x) => x,
            Err(e) => {
                println!("Cannnot write to file {}", e);
                continue;
            }
        };
        break;
    }
    match writeln!(config_file,"{}", text ) {
        Ok(..) => {},
        Err(e) => println!("Couldn't write to file : {} outputting instead : \n {}", e,text)
    }
}

fn longpress() -> KeyStates{
    let press_duration = loop {
        match  from_str::<u64>(&*retrieve_from_io("Set up the long press duration (in milliseconds) :")){
            Ok(x) => break x,
            Err(x) => println!("Please enter a positive integer: {}",x)
        }
    };
    return KeyStates::LongPress{ keybind_type: "longpress".to_string(), press_duration }
}

fn spampress()-> KeyStates{
    let spam_press_time_span = loop {
        match  from_str::<u64>(&*retrieve_from_io("Set up the time span for the spam (in milliseconds) : ")){
            Ok(x) => break x,
            Err(x) => println!("Please enter a positive integer: {}",x)
        }
    };
    let repetition = loop {
        match  from_str::<u16>(&*retrieve_from_io("Set the number of  repetition of the key please : ")){
            Ok(x) => break x,
            Err(x) => println!("Please enter a positive integer: {}",x)
        }
    };

    return KeyStates::SpamPress {keybind_type: "spampress".to_string(), spam_press_time_span, repetition }

}

