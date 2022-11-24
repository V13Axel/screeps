use screeps::{Creep, game, MaybeHasTypedId};

use crate::{mem::GameMemory, task::Task};

pub struct Screeps {}

impl Screeps {
    pub fn get_idle_screeps(game_memory: &GameMemory) -> Vec<Creep> {
        game::creeps().values().filter(|creep| {
            if let Some(id) = &creep.try_id() {
                if !game_memory.creeps.contains_key(id) {
                    return true;
                }

                match &game_memory.creeps.get(id).unwrap().current_task {
                    Task::Idle => true,
                    _ => false
                } 
            } else {
                false
            }
        }).collect()
    }
}
