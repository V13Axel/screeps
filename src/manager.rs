use std::cmp::Ordering;

use screeps::{Room, find, HasTypedId};

use crate::{mem::GameMemory, task::Task};

pub struct TaskManager {
    rooms: Vec<Room>,
}

impl TaskManager {
    pub fn with_rooms(rooms: Vec<Room>) -> Self {
        TaskManager {
            rooms
        }
    }

    pub fn run(&self, mut game_memory: GameMemory) -> GameMemory {
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
