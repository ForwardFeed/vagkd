use std::sync::{Arc, RwLock, Mutex};
use crate::internal_coms::{BusKey};
use crate::config_loader::{CfgKeybind};
use crate::key_matching;
use crate::key_matching::KeyMatching;

extern crate timer;
extern crate chrono;

/*
this struct will now keep track of matching of each sub keybind
 */

struct ManagerWorkSpace{
    //the subkeybind worker
    sub_keybind: Box<dyn KeyMatching>,
    //put a timer
    timer: timer::Timer,
    //put the timer guard, to cancel at any time
    guard: Option<timer::Guard>,
    //and now the signal, on or off
    signal: Arc<Mutex<bool>>,
}

impl ManagerWorkSpace{
    pub fn new(sub_keybind: Box<dyn KeyMatching>) ->ManagerWorkSpace{
        ManagerWorkSpace{
            sub_keybind,
            timer: timer::Timer::new(),
            guard: None,
            signal: Arc::new(Mutex::new(false)),
        }
    }

}




pub fn new(config: CfgKeybind, master_bus: Arc<RwLock<Vec<BusKey>>>) {

    let mut sub_keybind_management = vec![];
    let cfg_theshhold = config.timer_threshold;
    config.sub_keybinds.into_iter().for_each(|config| {

        sub_keybind_management.push(
            ManagerWorkSpace::new(
            key_matching::new(config.key_code, config.key_state)
            )
        );
    });


    let mut _current = BusKey::new();
    let mut mode: u8 = 255;
    let mut buffer_iterator: usize = 0;

    let mut are_all_keylistener_matched: u8;

    loop {
        match master_bus.try_read() {
            Ok(bus) => {
                _current = bus[buffer_iterator];
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
        if (_current.special ^ mode) != 0 {
            continue;
            //later i should check the freeze moment or other messages
        }
        if buffer_iterator == 15 { //15 being the last size of buffer so it need to loop to 0 again
            buffer_iterator = 0;
            mode = !mode;
        } else {
            buffer_iterator += 1;
        }
        //It's now time to try to match each keybinds
        sub_keybind_management.iter_mut().for_each(|workspace| {
            //Does this subkeybind is matched?
            if workspace.sub_keybind.key_matching(_current.key_code, _current.key_value) {
                //if one sub_keybind call a matched set the whole to true
                *workspace.signal.lock().unwrap()=true;
                //we reset the timer
                match &workspace.guard{
                    Some(p) => drop(p),
                    _ => {}
                }
                let lock_for_timer= workspace.signal.clone();
                //we reset the timer
                workspace.guard = Some(workspace.timer.schedule_with_delay(chrono::Duration::milliseconds(cfg_theshhold.clone() as i64), move ||{
                    *lock_for_timer.lock().unwrap()=false;
                }))
            }
        });
        //check all matched
        let counts_of_sub_keybinds = sub_keybind_management.len(); // get the number of elements inside the vector to compare if all said it matched
        are_all_keylistener_matched=0;
        sub_keybind_management.iter().for_each(|is_matched|{
            if *is_matched.signal.lock().unwrap() == true{
                are_all_keylistener_matched+=1;
            }
        });
        if are_all_keylistener_matched == counts_of_sub_keybinds as u8 {
            // Just stdout it and we're done
            println!("{}", config.name.clone());
            //Reset now all timer and set all to false it's perfectly fine we got a match
            sub_keybind_management.iter_mut().for_each(|workspace| {
                match &workspace.guard{
                    Some(p) => drop(p),
                    _ => {}
                }
                *workspace.signal.lock().unwrap()=false;
                //some keybinds needs to be reset to prevent light bug
                workspace.sub_keybind.reset();
            });
        }

    }
}
