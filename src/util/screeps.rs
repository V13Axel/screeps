use log::info;
use screeps::{Creep, game, SharedCreepProperties};

use crate::{mem::{GameMemory, CreepMemory}, task::Action};

pub struct Screeps {}

impl Screeps {
    pub fn get_idle_screeps(game_memory: &GameMemory) -> Vec<Creep> {
        game::creeps().values().filter(|creep| -> bool {
            if !game_memory.creeps.contains_key(&creep.name()) {
                info!("Creep not found in game_memory.creeps for idle: {}", &creep.name());
                return true;
            }

            game_memory.creeps.get(&creep.name()).unwrap().current_task == Action::Idle
        }).collect()
    }

    pub fn get_screeps_doing(task: crate::task::Action, game_memory: &mut GameMemory) -> Vec<CreepMemory> {
        game_memory.creeps.iter().filter(|(_,creep_memory)| {
            creep_memory.current_task == task
        }).map(|(_,creep_memory)| creep_memory.to_owned())
          .collect()
    }
}
