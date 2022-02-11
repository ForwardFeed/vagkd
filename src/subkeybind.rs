use std::sync::{Arc, RwLock, Mutex};
use crate::internal_coms::{BusKey};
use crate::config_loader::CfgSubKeybind;
use crate::key_matching;

pub fn start(config: CfgSubKeybind, manager_bus: Arc<Mutex<bool>>, barman_bus: Arc<RwLock<Vec<BusKey>>>) {
    let mut cfg = key_matching::new(config.key_code, config.key_state, config.longpress_threshold);
    let mut _current = BusKey::new();
    let mut mode :u8 = 255;
    let mut buffer_iterator: usize = 0;
    loop {
        match barman_bus.try_read() {
            Ok(bus) => {
                _current=bus[buffer_iterator];
            }
            Err(_) => {
                continue;
            }
        };
        /*
        this check is uhh
        if the mode is A, value 0: if it read a value that isn't the mode B, value 255, it will just loop until the mode is A, value 0.
        okay, let me be clear, this system is complex for nothing, i just like it

         */
        if (_current.special^mode)!=0 {
            continue;
            //later i should check the freeze moment or other messages

        }
        if buffer_iterator==15{ //15 being the last size of buffer so it need to loop to 0 again
            buffer_iterator=0;
            mode=!mode;
        }
        else{
            buffer_iterator+=1;
        }
        if cfg.key_matching(_current.key_code, _current.key_value){
            *manager_bus.lock().unwrap() = true;
        }
    }
}