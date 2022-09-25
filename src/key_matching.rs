use std::time;
use std::time::{Duration, Instant};
use crate::input_event::InputEvent;
use crate::config_loader::KeyStates;

//this trait will in function of the object make the correct matching it's a bit hard to understand even to myself tbh
pub trait KeyMatching {
    fn key_matching(&mut self, last_event: InputEvent) -> bool;
}


struct Simple{
    key_code: u16,
    key_value: i32,
}

impl Simple {
    fn new(key_code: u16, key_value: i32) -> Simple{
        Simple{key_code, key_value}
    }
}

//simply compare the config with what kernel say, really simple
impl KeyMatching for Simple{
    fn key_matching(&mut self, last_event: InputEvent) -> bool {
        if self.key_code == last_event.key_code && self.key_value == last_event.key_value{
            return true
        }
        return false
    }
}


/*
This Struct will be be for press that have to be X time long before matching
 */

struct LongPress{
    has_started: bool,
    key_code: u16,
    press_duration: std::time::Duration,
    start_timer: time::Instant
}

impl LongPress {
    fn new(key_code: u16, press_duration: u64) ->LongPress{


        LongPress {
            has_started: false,
            key_code,
            press_duration: Duration::from_millis(press_duration),
            start_timer: Instant::now(),
        }
    }

    fn check_time(&mut self, timestamp: Instant) -> bool{

        let delta = timestamp.duration_since(self.start_timer);
        return match delta.checked_sub(self.press_duration) {
            None => { false }
            Some(_) => { true }
        }
    }
}

impl KeyMatching for LongPress{
    fn key_matching(&mut self, last_event: InputEvent) -> bool {
        if last_event.key_code == self.key_code{
            return match last_event.key_value {
                0 => {
                    self.has_started = false;
                    false

                },
                1 =>{
                    self.has_started = true;
                    self.start_timer=  Instant::now();
                    false
                },
                _ => {
                     false
                }
            }
        }else{
            if self.has_started{
                if self.check_time(last_event.timestamp){
                    true
                }else{
                    false
                }
            }else{
                false
            }
        }
    }
}


/*
This structure will handle keymatching for spam pressing of one key during a specific time
 */
struct SpamPress{
    key_code: u16,
    spam_press_time_span: Duration,
    start_timer: time::Instant,
    cfg_count_press: u16,
    current_count_press: u16,
}

impl SpamPress{

    /*
    i don't use the keybind_type for now maybe in the future?
     */
    fn new(key_code: u16, spam_press_time_span: u64, repetition: u16 ) ->SpamPress{

        SpamPress {
            key_code,
            spam_press_time_span: std::time::Duration::from_millis(spam_press_time_span),
            start_timer: time::Instant::now(),
            cfg_count_press: repetition,
            current_count_press: 0,
        }
    }
}

impl KeyMatching for SpamPress {
    fn key_matching(&mut self, last_event: InputEvent) -> bool {
        if self.key_code == last_event.key_code{
            //we press down the keyboard key
            if last_event.key_value == 1{
                //If it's the first count down start the timer
                if self.current_count_press == 0 {
                    self.start_timer = last_event.timestamp;
                    //start the increment
                    self.current_count_press+=1;
                    //println!("init: {}", self.cfg_count_press.clone())
                }
                else {
                    //simply increment
                    self.current_count_press+=1;
                    //println!("current: {}", self.current_count_press.clone());
                }
            }
            //when we finally release we check if the count has been reached or not
            if last_event.key_value==0{
                //Before anything just calc if we pressed into enough time?
                // just check if the time elapsed between the first press and the second press is more than the time we wanted
                if last_event.timestamp-self.start_timer > self.spam_press_time_span {
                    //reset time baby?
                    self.current_count_press = 0;
                    return false;
                }
                //did we reached the number of count?
                //the >= here is to prevent a bug if someone put zero press in the config file
                if self.current_count_press >= self.cfg_count_press{
                    //yes we did, we just released the last one
                    self.current_count_press = 0;
                    return true;
                }
                return false;
            }
            //if the key_value is 2 we don't care
        }
        return false
    }
}


/*
  
 */
pub fn new(cfg_key_code: u16, cfg_key_state: KeyStates) -> Box<dyn KeyMatching>{

    match cfg_key_state {
        KeyStates::Simple{key_value} => Box::new(Simple::new(cfg_key_code, key_value)),
        KeyStates::LongPress{press_duration} => Box::new(LongPress::new(cfg_key_code, press_duration)),
        KeyStates::SpamPress{spam_press_time_span, repetition} => Box::new(SpamPress::new(cfg_key_code,spam_press_time_span ,repetition)),
    }

}