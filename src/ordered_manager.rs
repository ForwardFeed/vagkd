/*
use std::time::Duration;
use crate::config_loader::CfgKeybind;
use crate::{input_event, key_matching};
use crate::keybind_tracker::KeybindTrackers;


pub struct OrderedManager{
    keybind_team: Vec<KeybindTrackers>,
    cfg_threshold: u64,
    pub name: String,
}

impl OrderedManager {
    pub fn new(config: CfgKeybind) -> OrderedManager{
        let mut keybind_team: Vec<KeybindTrackers> = vec![];
        let cfg_threshold = config.timer_threshold;

        config.sub_keybinds.into_iter().for_each(|config| {
            keybind_team.push(
                KeybindTrackers::new(key_matching::new(config.key_code, config.key_state))
            );
        });
        OrderedManager{
            keybind_team,
            cfg_threshold,
            name: config.name,
        }
    }

    pub fn try_match(&mut self, last_event: input_event::InputEvent) -> bool {
        //we check if one of the keybind from the team has matched
        let mut has_one_matched = false;
        self.keybind_team.iter_mut().for_each(|mut keybind_tracker| {
            if keybind_tracker.keybind.key_matching(last_event.clone()) {
                keybind_tracker.has_matched = true;
                keybind_tracker.last_time_matched = last_event.timestamp.clone();
                has_one_matched = true;
            }
        });
        //if one matched we look if all have matched
        //And we check if they all matched in less time than the time threshold
        if !has_one_matched {
            return false
        }
        // I'll now pick each one, one by one and check if they matched in the time threshold
        let mut are_all_matched = true;
        let time_threshold = Duration::from_millis(self.cfg_threshold.clone());
        self.keybind_team.iter_mut().for_each(|mut keybind_tracker| {
            if keybind_tracker.has_matched {
                if last_event.timestamp - keybind_tracker.last_time_matched > time_threshold  { //check if it's over time
                    keybind_tracker.has_matched = false;
                    are_all_matched = false;
                }
            }else {
                are_all_matched=false;
            }
        });
        //put all as matched so there
        if are_all_matched{
            self.keybind_team.iter_mut().for_each(|mut keybind_tracker| {
                keybind_tracker.has_matched=false;
            })
        }
        return are_all_matched;
    }

}
*/