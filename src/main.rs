mod config_loader;
mod internal_coms;
mod barman;
mod threads_launcher;
mod key_matching;
mod manager;

extern crate clap;
extern crate core;

use clap::{Arg, App};

fn main() {
    /*
        Parameters:
            -c, --config :
                the config file containing all the keybind to be loaded: default macro-config.ron.
     */


    let matches = App::new("vagk")
        .version("U.w.U")
        .author("Someone on internet x)")
        .about("a global keybind daemon for Linux")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file, default macro-config.ron")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Sets a mode of output, default stdout, possible: stdout, name-pipe")
            .takes_value(true))
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or("macro-config.ron");

    let config = config_loader::new(config_file);
    let receiver = threads_launcher::start(config);

    let output_mode = matches.value_of("output").unwrap_or("stdout");
    loop{
        match receiver.try_recv(){
            Ok(x) => {outputting(output_mode, x)  },
            Err(..) => {}
        }
    }
}

fn outputting(mode: &str, msg: u32){
    match mode {
        "name-pipe" => {} //TODO
        "stdout" => {println!("{}", msg)}
        _ => {println!("{}", msg)}
    }
}