#![feature(lazy_cell, ptr_sub_ptr)]

use unity::prelude::*;
use engage::gamedata::*;
use engage::gamevariable::*;

mod cclearn;
mod learnlevel;

#[unity::hook("App", "JobData", "GetLearnJobSkillLevel")]
pub fn jobdata_getlearnjobskilllevel(this: &JobData, method_info: OptionalMethod) -> i32{
    // Check if the save is in a sequence where the function can actually not crash.
    // Function is called before title screen, so it crashes without a check for a valid save, unsure if this covers all that can be loaded.
    if unsafe { gamesavedata_isgmapsequence(None) } || unsafe { gamesavedata_ishubsequence(None) } || unsafe { gamesavedata_issortieormapsequence(None) } {
        // Set levels to the appropriate levels if the variable is initialized.
        if GameVariableManager::get_number(learnlevel::LEARNLEVEL_KEY) != 0 {
            // Check if class can reach level 40.
            if unsafe { jobdata_get_maxlevel(this, None) } < 40 as u8 {
                let level = GameVariableManager::get_number(learnlevel::LEARNLEVEL_KEY);
                return level;
            } else {
                let level = GameVariableManager::get_number(learnlevel::LEARNLEVEL_KEY) + 20;
                return level;
            }
        // Return default function value if the either condition is not met.     
        } else {
            return call_original!(this, method_info);
        }    
    } else {
        return call_original!(this, method_info);
    }               
}

#[unity::from_offset("App", "JobData", "get_MaxLevel")]
pub fn jobdata_get_maxlevel(this: &JobData, method_info: OptionalMethod) -> u8;

// Somniel
#[unity::from_offset("App", "GameSaveData", "IsHubSequence")]
pub fn gamesavedata_ishubsequence(method_info: OptionalMethod) -> bool;

// World Map
#[unity::from_offset("App", "GameSaveData", "IsGmapSequence")]
pub fn gamesavedata_isgmapsequence(method_info: OptionalMethod) -> bool;

// Before or During Chapter
#[unity::from_offset("App", "GameSaveData", "IsSortieOrMapSequence")]
pub fn gamesavedata_issortieormapsequence(method_info: OptionalMethod) -> bool;

#[skyline::main(name = "learnskl")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };


        let err_msg = format!(
            "Skill Level plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            420,
            "Skill Level plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hook!(jobdata_getlearnjobskilllevel);
    learnlevel::learnlevel_install();
    cclearn::cclearn_install();
}

