use crate::{extractor};
use crate::config_loader::Config;
use crate::manager::Manager;

pub fn start(config: Config){
    //TODO MODIFY THE CONFIG, remove barman references
    let mut extractor = extractor::Extractor::new(config.general_parameters.event_path);
    let mut managers = vec![];
    config.keybinds.into_iter().for_each(|keybind|{
        managers.push(Manager::new(keybind));
    });
    loop{
       let last_event = extractor.last_keyboard_event();
        for manager in &mut managers{
            if manager.try_match(last_event.clone()){
                println!("{}",manager.name);
            }
        }
    }
}