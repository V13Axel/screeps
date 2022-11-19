use screeps::{Creep, game, SharedCreepProperties};

use crate::{mem::GameMemory, minion::CreepWorkerType, task::Task};

pub struct Screeps {}

impl Screeps {
    pub fn get_idle_screeps(game_memory: &GameMemory) -> Vec<Creep> {
        game::creeps().values().filter(|creep| {
            if !game_memory.creep_memories.contains_key(&creep.name()) {
                return true;
            }

            match &game_memory.creep_memories.get(&creep.name()).unwrap().worker_type {
                CreepWorkerType::SimpleWorker(task) => match task {
                    Task::Idle => true,
                    _ => false
                } 
            } 
        }).collect()
    }
}
