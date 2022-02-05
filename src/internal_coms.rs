//ITT communications between the eventProducteur/eventListener and , will hold the generation of communications elements


use std::sync::{RwLock, Arc, Mutex};
/*
key_code: will be the kernel keycode, between 1 and 255 i think
key_value: will be the event associated with the keypress
special: will indicate some meta code, should be considered as a signal
    0 => the data is outdated
    1 => the data is good
    3 => mean freeze
    4 => mean unfreeze
 */
#[derive(Clone, Copy, Debug)]
pub struct BusKey{
    pub(crate) key_code: u16,
    pub(crate) key_value: u32,
    pub(crate) special: u8,
}
impl BusKey {
    pub fn new() -> BusKey {
        BusKey {
            key_code: 0,
            key_value: 0,
            special: 0,
        }
    }
}


//smart pointer for inter-thread coms with a RwLock to share data

pub struct BarmanComs {
    bus_key: Arc<RwLock<Vec<BusKey>>>
}
impl BarmanComs {
    pub fn new()-> BarmanComs {
        BarmanComs {
            bus_key: Arc::new(RwLock::new(vec![BusKey::new(); 16])) // the 16 here mean the buffer will be 16 only long, not very good i'd rather a macro but whatever
            /*
            okay this need some explanation: i need to put a buffer inside this shared memory to cover the lag between the barman and all sub_keybinds
             */
        }
    }
    pub fn generate_arc_link(&self)-> Arc<RwLock<Vec<BusKey>>>{
        return Arc::clone(&self.bus_key);//fuck i can't pass the reference of a smart pointer
    }
}

//Commmunication between the manager of a macro and the submacros
//for each subthread the manager will share an ear (this bool) who will hear a matching or not situation
pub struct ManagerKeybindsComs {
    manager_keybinds_coms: Arc<Mutex<bool>>
}

impl ManagerKeybindsComs {
    pub fn new()-> ManagerKeybindsComs {
        ManagerKeybindsComs {
            manager_keybinds_coms: Arc::new(Mutex::new( false))
        }
    }
    pub fn generate_arc_link(&self)->Arc<Mutex<bool>> {return  Arc::clone(&self.manager_keybinds_coms)}
}