#![feature(lazy_cell, ptr_sub_ptr)]

use unity::prelude::*;
use engage::gamedata::*;
use engage::gamevariable::*;

mod cclearn;
mod learnlevel;

#[unity::hook("App", "JobData", "GetLearnJobSkillLevel")]
pub fn jobdata_getlearnjobskilllevel(this: &JobData, method_info: OptionalMethod) -> i32{
    let mut level = 5;
    // need to check if game is loaded into save data or crash
    //if get_maxlevel(this, None) < 40 as u8 {
    //    level = GameVariableManager::get_number(learnlevel::LEARNLEVEL_KEY);
    // } else {
    //    level = GameVariableManager::get_number(learnlevel::LEARNLEVEL_KEY);
    //    level = level + 20;
    // }
    level
}

#[unity::from_offset("App", "JobData", "get_MaxLevel")]
pub fn jobdata_get_maxlevel(this: &JobData, method_info: OptionalMethod) -> u8;

pub fn get_maxlevel(this: &JobData, _method_info: OptionalMethod) -> u8{ unsafe { jobdata_get_maxlevel(this, None) } }

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

