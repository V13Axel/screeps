use std::{cmp::Ordering, collections::HashMap};

use log::info;
use screeps::{Room, find, HasTypedId, game, SharedCreepProperties, MaybeHasTypedId, StructureSpawn, Part, Creep, Terrain, LookResult};

use crate::{mem::{GameMemory, CreepMemory}, task::Task, minion::MinionType, util};

use self::{task::TaskManager, spawn::SpawnManager};

mod spawn;
mod task;

pub struct Managers {
}

impl Managers {
    pub fn run(memory: &mut GameMemory) {
        // Only want to run managers if it's been 20 ticks.
        if game::time() - memory.last_managers_tick < 20 {
            return;
        }

        Self::handle_managers(memory);

        memory.last_managers_tick = game::time();
    }

    fn handle_managers(memory: &mut GameMemory) {
        let rooms: Vec<Room> = game::rooms()
            .values()
            .collect();

        TaskManager::with_rooms(&rooms).scan(memory);
        TaskManager::assign(memory);

        SpawnManager::with_rooms(&rooms).spawn(memory);
    }
}
