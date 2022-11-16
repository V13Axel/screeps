use std::collections::HashMap;

use screeps::{ObjectId, Creep, Structure, Position, Room, find, HasTypedId};
use serde::{Serialize, Deserialize};

use crate::{minion::{CreepWorkerType, SimpleJob}, task::Task};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameMemory {
    pub needs_deserialized: bool,
    pub creep_memories: HashMap<ObjectId<Creep>, CreepMemory>,
    pub room_memories: HashMap<String, RoomMemory>,
    pub structure_memories: HashMap<ObjectId<Structure>, StructureMemory>,
    pub tasks: HashMap<String, Vec<Task>>,
}

impl GameMemory {
    pub fn default() -> Self {
        GameMemory { 
            needs_deserialized: true,
            creep_memories: HashMap::new(),
            room_memories: HashMap::new(),
            structure_memories: HashMap::new(),
            tasks: HashMap::new()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RoomMemory {
    controller_level: usize,
}

impl RoomMemory {
    pub fn default() -> Self {
        Self {
            controller_level: 1
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum StructureMemory {
    Spawner(i32),
    Controller(ControllerMemory),
    Empty,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ControllerMemory {
    controller_level: usize
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreepMemory {
    worker_type: CreepWorkerType,
    current_path: Option<CreepPath>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreepPath {
    steps: Vec<Position>,
}

impl CreepMemory {
    pub fn default(room: Room) -> CreepMemory {
        CreepMemory {
            worker_type: CreepWorkerType::SimpleWorker(
                SimpleJob::ApproachSpawn(
                    room.find(find::MY_SPAWNS)[0].id()
                )
            ),
            current_path: None
        }
    }
}
