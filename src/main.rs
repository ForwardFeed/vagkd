mod config_loader;
mod key_matching;
mod manager;
mod main_loop;
mod extractor;
mod input_event;
mod generate;

extern crate clap;
extern crate core;


use clap::{Arg, App};
use crate::generate::Generate;

fn main() {
    /*
        Parameters:
            -c, --config :
                the config file containing all the keybind to be loaded: default macro-config.ron.
     */


    let matches = App::new("vagk")
        .version("1.0.0")
        .author("ForwardFeel")
        .about("a global keybind software for Linux")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file, if not specified: macro-config.ron")
            .takes_value(true))
        .arg(Arg::with_name("generate")
            .short("g")
            .long("generate")
            .help("Generate a custom config file")
            .takes_value(false))
        .get_matches();

    //check if we're in a generate mode
    if matches.is_present("generate"){
        Generate::new().start();
    }
    else{
        let config_file = matches.value_of("config").unwrap_or("macro-config.ron");
        let config = config_loader::new(config_file);
        main_loop::start(config);
    }


}

