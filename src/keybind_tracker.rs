
use std::time::Instant;
use crate::key_matching::KeyMatching;

/*
this struct will now keep track of matching of each sub keybind
 */

pub struct KeybindTrackers {
    //record the time
    pub(crate) last_time_matched: Instant,
    //the keybinding worker
    pub(crate) keybind: Box<dyn KeyMatching>,
    //is the keybinding matched?
    pub(crate) has_matched: bool,
}

impl KeybindTrackers {
    pub fn new(keybind: Box<dyn KeyMatching>) -> KeybindTrackers {
        let last_matched = Instant::now();
        KeybindTrackers {
            keybind,
            last_time_matched: last_matched,
            has_matched: false,
        }
    }

}
