use screeps::{Room, game};

use crate::mem::GameMemory;

use self::{task::TaskManager, spawn::SpawnManager};

mod spawn;
mod task;

pub struct Managers;

impl Managers {
    pub fn run(memory: &mut GameMemory) {
        // Only want to run managers if it's been 20 ticks since last time
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
