use screeps::{Room, find, HasTypedId};

use crate::{mem::{GameMemory, RoomMemory}, task::Task};

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
            if !game_memory.tasks.contains_key(&room.name().to_string()) {
                game_memory.tasks.insert(room.name().to_string(), self.scan_room(&room));
            }
        }

        game_memory
    }

    pub fn scan_room(&self, room: &Room) -> Vec<Task> {
        let mut room_tasks: Vec<Task> = vec![];

        for source in room.find(find::SOURCES).iter() {
            room_tasks.push(Task::Harvest { node: source.id() });
        }

        room_tasks
    }
}
