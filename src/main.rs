mod config_loader;
mod internal_coms;
mod barman;
mod threads_launcher;
mod key_matching;
mod manager;
mod subkeybind;

extern crate clap;
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
        .about("a global keybind matcher for Linux")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file, default macro-config.ron")
            .takes_value(true))
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or("macro-config.ron");
    let config = config_loader::new(config_file);
    threads_launcher::start(config);
}

