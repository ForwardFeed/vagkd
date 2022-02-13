use crate::internal_coms::BusKey;
use std::sync::{RwLock, Arc};
use std::fs::File;
use std::io::Read;
use std::process::exit;
use crate::key_matching;
use crate::key_matching::{KeyMatching};
use crate::config_loader;

pub struct Barman {
    event_file: String,
    freeze: Box<dyn KeyMatching>,
    reload: Box<dyn KeyMatching>,
    shutdown: Box<dyn KeyMatching>,
    current: BusKey,
    bus: Arc<RwLock<Vec<BusKey>>>,
    is_freeze: bool,
    buffer_iterator: usize,
    mode: u8,
}
impl Barman {

    //i call it barman because it's the guy who serve every thread
    pub fn start(mut self) -> bool {//true mean reload, false mean quit
        let mut file = Self::open_event_file(self.event_file.clone());
        let mut buffer = [0; 24];
        loop {
            file.read(&mut buffer[..]).unwrap();
            let key_t = (buffer[16] as u16 | (buffer[17] as u16) << 8) as u16;
            let key_c = (buffer[18] as u16 | (buffer[19] as u16) << 8) as u16;
            let key_v = (buffer[20] as u32 | (buffer[21] as u32) << 8 | (buffer[22] as u32) << 16 | (buffer[23] as u32) << 24) as u32;
            if (key_t as u32 | key_c as u32 | key_v) == 0 {
                //println!("-----------SYN_EVENT------------------");
                continue;
            }
            if (key_t | key_c) == 4 || (key_t | key_c) == 0 {
                continue;
            }
            self.current.key_code= key_c; self.current.key_value=key_v;
            // is related to the macro manager? //i retrieved the freeze handler to a better function
            let rm = self.is_related_to_manager();
            match rm {
                0 => (), // do nothing
                2 => {
                    self.new_bus_com();
                    return true
                }, // will return the code 5 which mean reload everything please
                3 => {
                    self.new_bus_com();
                },
                4 => {
                    self.new_bus_com();
                }
                _ => panic!("output in is_related_to_manager return not valid, found {}, expected 0, 1 or 2", rm),
            }
            self.new_bus_com();
        }
    }
    fn is_related_to_manager(&mut self) -> u8 {

        //is shutdown?
        if self.shutdown.key_matching(self.current.key_code, self.current.key_value) {
            exit(8008135)//why am i so retarded?
        }
        // is time to reload?
        if  self.reload.key_matching(self.current.key_code, self.current.key_value){
            self.current.special=2;
            return 2;
        }
        if self.freeze.key_matching(self.current.key_code, self.current.key_value) {
            return if self.is_freeze {
                self.is_freeze = false;
                self.current.special = 3;
                3
            } else {
                self.is_freeze = true;
                self.current.special = 4;
                4
            }
        }
        return 0
    }

    //okay once the function new_bus_com has been lauched all thread will jump on it to eat all the data.
    fn new_bus_com(&mut self) { // will increment the comID in the barman thread, i should replace it with a pointer but later okay?
        /*
        let's make two modes A and B
        A represented by the value 0 and B by the value 0
        this bus is circular buffer.
        and the barman is gonna fill the whole buffer one time saying "mode A"
        so the people reading the barman stuff will be following
        if they see the next value is the current mode, lets say A, the value 0 they read it
        if they see the next value is the previous or next mode so in this case B, the value 255 they don't read it and wait.
        if they totally miss a whole buffer they will wait for the next buffer
         */
        if self.buffer_iterator==15{ //15 being the last size of buffer so it need to loop to 0 again
            self.buffer_iterator=0;
            self.mode=!self.mode;//invert mode
        }
        else{
            self.buffer_iterator+=1;
        }
        let mut bus = self.bus.write().unwrap();
        self.current.special=self.mode;
        //println!("{} {}",self.mode, self.buffer_iterator);
        bus[self.buffer_iterator]=self.current;

    }
    fn open_event_file(event_file: String) -> File {

        let file = File::open(event_file);
        match file {
            Ok(file) => return file,
            Err(e) => panic!("Error while opening event file : {}", e),
        };
    }

}

pub fn new(config: config_loader::CfgBarman, bus: Arc<RwLock<Vec<BusKey>>>) -> Barman {
    Barman {
        event_file: config.event_path,
        freeze: key_matching::new(config.freeze_key_code, config.freeze_key_state, 0, 0),
        reload: key_matching::new(config.reload_key_code, config.reload_key_state, 0,0),
        shutdown: key_matching::new(config.shutdown_key_code, config.shutdown_key_state, 0,0),
        current: BusKey::new(),
        bus,
        is_freeze: false,
        buffer_iterator: 15,
        mode: 0,
    }
}