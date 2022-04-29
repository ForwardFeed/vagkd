use std::process::exit;
use std::io::{stdin, stdout, Write};
use std::io::Read;
use std::fs::File;
use std::time;
use ron::from_str;
use ron::ser::{PrettyConfig, to_string_pretty};

use crate::config_loader;

pub struct Generate{
    event_file_path: std::string::String,
}
pub fn new() -> Generate {

    /*First we need to retrieve the event path*/
    print!("Enter the event path (Ex. /dev/input/event3 ) :");
    stdout().flush().unwrap();
    let mut event_file_input = std::string::String::new();
    stdin().read_line(&mut event_file_input).unwrap();
    event_file_input.pop(); // we trim the newline char.
    let event_file = File::open(event_file_input.clone());
    /* just check we can read the file */
    match event_file {
        Ok(..) => return Generate{event_file_path: event_file_input},
        Err(e) => panic!("Error while opening event file ({}) : {}",event_file_input, e),
    };
}
impl Generate {

    pub fn start(&mut self){
        let mut keybinds: Vec<config_loader::CfgKeybind> = vec![];
        let pretty = PrettyConfig::new();
        let config_generated;
        /*we will make a keybind but since we can make multiple of them i'll loop a function*/
        loop{
            println!("Creation of a Keybind");
            keybinds.push(self.create_keybinds());
            self.print_flush("Would you like to continue to create Keybinds to this configuration file? (Y/n) :");
            let mut input = std::string::String::new();
            stdin().read_line(&mut input).unwrap();
            input.pop();
            match &input  as &str {
                "y" | "Y" => continue,
                _ => break
            }
        }
        config_generated = config_loader::Config{barman: config_loader::CfgBarman{event_path : self.event_file_path.clone()}, keybinds};

        let config_as_string = to_string_pretty(&config_generated, pretty).expect("Serialization failed");
        self.write_into_file(config_as_string);
        exit(0);
    }

    fn create_keybinds(&mut self) -> config_loader::CfgKeybind{
        let mut sub_keybinds: Vec<config_loader::CfgSubKeybind> = vec![];
        let name : std::string::String;
        let timer_threshold: u64;

        /* we need to get the name of the  macro */
        loop {
            self.print_flush("What should be the name of this keybind? : ");
            let mut name_input = std::string::String::new();
            stdin().read_line(&mut name_input).unwrap();
            name_input.pop();
            match name_input.parse() {
                Ok(x) => {
                    name = x;
                    break;
                },
                Err(..) => ()
            }
        }


        loop{
            self.print_flush("Please enter the time threshold for this keybind (in milliseconds) :");
            let mut timer_threshold_input =std::string::String::new();
            stdin().read_line(&mut timer_threshold_input).unwrap();
            timer_threshold_input.pop();
            match from_str::<u64>(&*timer_threshold_input) {
                Ok(x) => {
                    timer_threshold = x;
                    break;
                },
                Err(x) => println!("Please enter a positive integer: {}",x)
            }
        }
        loop {
            sub_keybinds.push(self.create_sub_keybind());
            self.print_flush("Would you like to continue to create sub-Keybinds to this keybind? (Y/n) :");
            let mut input =std::string::String::new();
            stdin().read_line(&mut input).unwrap();
            input.pop();
            match &input as &str {
                "Y" | "y" => continue,
                _ => break
            }
        }
        return config_loader::CfgKeybind{sub_keybinds ,name, timer_threshold};

    }

    fn create_sub_keybind(&mut self) -> config_loader::CfgSubKeybind{
        let key_code: u16;
        let key_state: config_loader::KeyStates;
        key_code = self.get_key();
        key_state = self.create_keystates();
        return config_loader::CfgSubKeybind{key_code,key_state}
    }

    fn create_keystates(&self) -> config_loader::KeyStates {
        loop {
            self.print_flush("Please enter a keystates, or \"help/h\" to get help : ");
            let mut key_states = std::string::String::new();
            stdin().read_line(&mut key_states).unwrap();
            key_states.pop();
            match &key_states as &str {
                "simple"=> {
                return config_loader::KeyStates::Simple {keybind_type: "press".to_string()}
                },
                "longpress"=>{
                 let press_duration: u64;
                 loop {
                     self.print_flush("Set up the long press duration (in milliseconds) :");
                     let mut press_duration_input = std::string::String::new();
                     stdin().read_line(&mut press_duration_input).unwrap();
                     press_duration_input.pop();
                     match from_str::<u64>(&*press_duration_input) {
                         Ok(x) => {
                             press_duration=x;
                             break;},
                         Err(..) => println!("Please enter a positive integer"),
                     }
                 }
                 return config_loader::KeyStates::LongPress{ keybind_type: "longpress".to_string(), press_duration }
                },
                "spampress"=>{
                let spam_press_time_span  :u64;
                let repetition :u16;
                loop {
                    self.print_flush("Set up the time span for the spam (in milliseconds) : ");
                    let mut spam_press_time_span_input = std::string::String::new();
                    stdin().read_line(&mut spam_press_time_span_input).unwrap();
                    spam_press_time_span_input.pop();
                    match from_str::<u64>(&*spam_press_time_span_input) {
                        Ok(x) => {
                            spam_press_time_span = x;
                            break;
                        },
                        Err(..) => println!("Please enter a positive integer")
                    }
                }
                loop{
                    self.print_flush("Set the number of  repetition of the key please : ");
                    let mut repetition_input = std::string::String::new();
                    stdin().read_line(&mut repetition_input).unwrap();
                    repetition_input.pop();
                    match from_str::<u16>(&*repetition_input){
                        Ok(x) => {
                            repetition=x;
                            break;},
                        Err(..) => println!("Please enter a positive integer")
                    }
                }
                    return config_loader::KeyStates::SpamPress {keybind_type: "spampress".to_string(), spam_press_time_span, repetition }
                },
                _ =>{
                self.print_help_keystates();
                }
            }
        }
    }

    fn get_key(&mut self) -> u16{
        self.print_flush("Now what key should be sub-keybinded? (press it in 2 sec)");
        std::thread::sleep(time::Duration::from_secs(2));
        let event_file = File::open(self.event_file_path.clone());
        match event_file {
            Ok(event_file) => return self.read_key(event_file),
            Err(e) => panic!("Error while opening event file ({}) : {}",self.event_file_path, e),
        };

    }

    fn read_key(&mut self, mut event_file: File) -> u16 {
        let mut buffer  = [0; 24];
        loop {
            event_file.read(&mut buffer[..]).unwrap();
            let key_t = (buffer[16] as u16 | (buffer[17] as u16) << 8) as u16;
            let key_c = (buffer[18] as u16 | (buffer[19] as u16) << 8) as u16;
            let key_v = (buffer[20] as u32 | (buffer[21] as u32) << 8 | (buffer[22] as u32) << 16 | (buffer[23] as u32) << 24) as u32;
            if (key_t as u32 | key_c as u32 | key_v) == 0 {
                //println!("-----------SYN_EVENT------------------");
                continue;
            }
            if (key_t | key_c) == 4 || (key_t | key_c) == 0 {
                continue;
            }
            println!("OK : {}", key_c.clone());
            return key_c;
        }
    }

    fn print_help_keystates(&self){
        println!("There is for now three type of keyStates\n\
    spampress which is when you spam a key X time in Y number of time\n\
    longpress which is when you hold a key for X time\n\
    simple    which is just when you simply press it")
    }

    fn print_flush(&self , message: &str){
        print!("{}",message);
        stdout().flush().unwrap();
    }

    fn write_into_file(&self, text : String){
        let mut name = std::string::String::new();
        let mut config_file;

        loop {
            self.print_flush("Writing to file? default 'macro-config.ron'");
            stdin().read_line(&mut name).unwrap();
            name.pop();
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
}
