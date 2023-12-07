use unity::prelude::*;
use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use engage::gamevariable::*;
pub const LEARNLEVEL_KEY: &str = "G_LEARNLEVEL";

pub struct LearnLevelSettings;
impl ConfigBasicMenuItemSwitchMethods for LearnLevelSettings {
    fn init_content(this: &mut ConfigBasicMenuItem) {
        GameVariableManager::make_entry_norewind(LEARNLEVEL_KEY, 5);
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let value = GameVariableManager::get_number(LEARNLEVEL_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(value, 1, 20, 1);
        if value != result { 
            GameVariableManager::set_number(LEARNLEVEL_KEY, result);
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            this.update_text();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }
    
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        let value = GameVariableManager::get_number(LEARNLEVEL_KEY);
        let value_sp = value + 20;
        this.help_text = format!("Advanced Level: {}, Special Level: {}", value, value_sp).into();

    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        let value = GameVariableManager::get_number(LEARNLEVEL_KEY);
        this.command_text = format!("{}", value).into();
    }
}

#[no_mangle]
extern "C" fn learnlevel_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_switch::<LearnLevelSettings>("Skill Learn Level")
}


pub fn learnlevel_install() {
    cobapi::install_game_setting(learnlevel_callback);
}