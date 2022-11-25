use std::collections::HashMap;

use screeps::{ObjectId, Structure};
use serde::{Serialize, Deserialize};

use crate::{minion::MinionType, task::Task, util::path::CreepPath};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameMemory<'a> {
    // Administrative
    pub last_managers_tick: u32,
    pub needs_deserialized: bool,

    // Memory
    pub creeps: HashMap<String, CreepMemory<'a>>,
    pub room_memories: HashMap<String, RoomMemory>,
    pub structure_memories: HashMap<ObjectId<Structure>, StructureMemory>,

    // Task queues
    #[serde(skip_deserializing)]
    pub room_task_queues: HashMap<String, HashMap<MinionType, Vec<Box<dyn Task>>>>,
}

impl GameMemory<'_> {
    pub fn default() -> Self {
        GameMemory { 
            needs_deserialized: true,
            last_managers_tick: 0,
            creeps: HashMap::new(),
            room_memories: HashMap::new(),
            structure_memories: HashMap::new(),
            room_task_queues: HashMap::new(),
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
pub struct CreepMemory<'a> {
    pub worker_type: MinionType,
    pub current_path: Option<CreepPath>,

    #[serde(skip_deserializing)]
    pub current_task: Option<&'a Box<dyn Task>>,
}

impl Default for CreepMemory<'_> {
    fn default() -> CreepMemory<'static> {
        CreepMemory {
            worker_type: MinionType::SimpleWorker,
            current_path: None,
            current_task: None,
        }
    }
}

impl Default for &CreepMemory<'_> {
    fn default() -> &'static CreepMemory<'static> {
        &CreepMemory {
            worker_type: MinionType::SimpleWorker,
            current_path: None,
            current_task: None,
        }
    }
}
