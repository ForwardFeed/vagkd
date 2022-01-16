mod config_loader;
mod internal_coms;
mod barman;
mod threads_launcher;
mod key_matching;
mod manager;
mod subkeybind;


fn main() {
    let config = config_loader::new();
    threads_launcher::start(config);
}

