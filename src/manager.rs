use std::{cmp::Ordering, collections::HashMap};

use log::info;
use screeps::{Room, find, HasTypedId, game, Creep, SharedCreepProperties, MaybeHasTypedId};

use crate::{mem::{GameMemory, CreepMemory}, task::Task, minion::CreepWorkerType};

pub struct TaskManager {
    rooms: Vec<Room>,
}

impl TaskManager {
    pub fn with_rooms(rooms: Vec<Room>) -> Self {
        TaskManager {
            rooms
        }
    }

    pub fn assign(mut game_memory: GameMemory) -> GameMemory {
        // TODO: Filter for only idle creeps
        let creeps: Vec<Creep> = game::creeps().values().collect();

        for creep in creeps {
            let id = creep.try_id();
            if id.is_none() {
                continue;
            }


            let default_creep_memory = &CreepMemory::default(creep.room().unwrap());
            let creep_memory: CreepMemory = game_memory.creep_memories.get(&id.unwrap()).unwrap_or(default_creep_memory).to_owned();
            info!("{:?}", creep_memory);

            let current_task: Task = match &creep_memory.worker_type {
                CreepWorkerType::SimpleWorker(task) => task.to_owned()
            };

            game_memory.creep_memories.insert(id.unwrap(), creep_memory);
        }

        game_memory
    }

    pub fn scan(&self, mut game_memory: GameMemory) -> GameMemory {
        for room in self.rooms.iter() {
            game_memory.tasks.insert(room.name().to_string(), self.scan_room(&room));
        }

        game_memory
    }

    pub fn scan_room(&self, room: &Room) -> Vec<Task> {
        let mut room_tasks: Vec<Task> = vec![];
        let spawn = &room.find(find::MY_SPAWNS)[0];

        let mut sources = room.find(find::SOURCES);
        sources.sort_by(|a, b| {
            if spawn.pos().get_range_to(a) > spawn.pos().get_range_to(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        for source in sources.iter() {
            room_tasks.push(Task::Harvest { node: source.id(), worked_by: vec![] });
        }

        room_tasks
    }
}
