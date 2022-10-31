mod config_loader;
mod key_matching;
mod manager;
mod main_loop;
mod extractor;
mod input_event;
mod ordered_manager;
mod keybind_tracker;
mod listen_and_convert;

extern crate clap;
extern crate core;


use std::fs::File;
use clap::{Arg, App};

fn main() {
    let matches = App::new("vagkd")
        .version("1.0.0")
        .author("ForwardFeed")
        .about("a global keybind software for Linux")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Sets a custom config file, if not specified: macro-config.json")
            .takes_value(true))
        .get_matches();

    let config_file = matches.value_of("input").unwrap_or("macro-config.json");
    let file =
    match  File::open(config_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error while opening {} : {}",config_file, e);
            std::process::exit(1);
        },
    };
    let config = config_loader::new(file);
    main_loop::start(config);

}

