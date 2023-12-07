use unity::prelude::*;
use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use skyline::patching::Patch;
use engage::gamevariable::*;
pub const CCLEARN_KEY: &str = "G_CCLEARN";

pub fn patch(active: bool) {
    if active {
        Patch::in_text(0x01BE9508).bytes([0x04, 0x00, 0x00, 0x14]).unwrap();
    } else {
        Patch::in_text(0x01BE9508).bytes([0x88, 0x00, 0x00, 0x34]).unwrap();
    }

}

pub struct CCLearnSettings;
impl ConfigBasicMenuItemSwitchMethods for CCLearnSettings {
    fn init_content(this: &mut ConfigBasicMenuItem) {
        GameVariableManager::make_entry_norewind(CCLEARN_KEY, 0);
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let active = GameVariableManager::get_bool(CCLEARN_KEY);
        let result = ConfigBasicMenuItem::change_key_value_b(active);
        if active != result { 
            if result {
                GameVariableManager::set_bool(CCLEARN_KEY, true);
            } else {
                GameVariableManager::set_bool(CCLEARN_KEY, false);
            }
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            patch(result);
            this.update_text();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }
    
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        let active = GameVariableManager::get_bool(CCLEARN_KEY);
        if active {
            this.help_text = format!("Learn Class Skill on Class Change (Unit still needs to be the correct level)").into();

        } else {
            this.help_text = format!("Do not Learn Class Skill on Class Change").into();
        } 
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        let active = GameVariableManager::get_bool(CCLEARN_KEY);
        if active {
            this.command_text = format!("On").into();
        } else {
            this.command_text = format!("Off").into();
        }
    }
}


#[no_mangle]
extern "C" fn cclearn_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_switch::<CCLearnSettings>("Learn Class Skill on Class Change")
}


pub fn cclearn_install() {
    cobapi::install_game_setting(cclearn_callback);
}