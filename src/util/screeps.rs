use screeps::{Creep, game, SharedCreepProperties};

use crate::{mem::GameMemory, task::WorkableTask};

pub struct Screeps {}

impl Screeps {
    pub fn get_idle_screeps(game_memory: &GameMemory) -> Vec<Creep> {
        game::creeps().values().filter(|creep| -> bool {
            if !game_memory.creeps.contains_key(&creep.name()) {
                return true;
            }

            game_memory.creeps.get(&creep.name()).unwrap().current_task == WorkableTask::Idle
        }).collect()
    }
}
