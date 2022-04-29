/*
    Communications between the eventProducer/eventListener and , will hold the generation of communications elements
    the Producer is the barman and the managers are the listeners
    i rather call listenning the consomming because in term of memory management the data isn't consummed, it's cloned
 */


use std::sync::{RwLock, Arc};

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


/*
     smart pointers (ARC) for inter-thread coms with a RwLock to share data
*/
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