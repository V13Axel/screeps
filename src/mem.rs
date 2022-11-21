use std::collections::HashMap;

use screeps::{ObjectId, Structure};
use serde::{Serialize, Deserialize};

use crate::{minion::CreepWorkerType, task::Task, util::path::CreepPath};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameMemory {
    pub last_managers_tick: u32,
    pub needs_deserialized: bool,
    pub creeps: HashMap<String, CreepMemory>,
    pub room_memories: HashMap<String, RoomMemory>,
    pub structure_memories: HashMap<ObjectId<Structure>, StructureMemory>,
    pub tasks: HashMap<String, Vec<Task>>,
}

impl GameMemory {
    pub fn default() -> Self {
        GameMemory { 
            needs_deserialized: true,
            last_managers_tick: 0,
            creeps: HashMap::new(),
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
    pub worker_type: CreepWorkerType,
    pub current_path: Option<CreepPath>,
    pub current_task: Task,
}

impl Default for CreepMemory {
    fn default() -> CreepMemory {
        CreepMemory {
            worker_type: CreepWorkerType::SimpleWorker,
            current_path: None,
            current_task: Task::Idle,
        }
    }
}

impl Default for &CreepMemory {
    fn default() -> &'static CreepMemory {
        &CreepMemory {
            worker_type: CreepWorkerType::SimpleWorker,
            current_path: None,
            current_task: Task::Idle,
        }
    }
}
