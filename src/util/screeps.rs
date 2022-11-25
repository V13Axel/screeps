use screeps::{Creep, game, MaybeHasTypedId, SharedCreepProperties};

use crate::{mem::GameMemory, task::Task};

pub struct Screeps {}

impl Screeps {
    pub fn get_idle_screeps(game_memory: &GameMemory) -> Vec<Creep> {
        game::creeps().values().filter(|creep| {
            if !game_memory.creeps.contains_key(&creep.name()) {
                return true;
            }

            match &game_memory.creeps.get(&creep.name()).unwrap().current_task {
                Task::Idle => true,
                _ => false
            } 
        }).collect()
    }
}
