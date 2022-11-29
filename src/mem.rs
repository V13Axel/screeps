use std::collections::HashMap;

use screeps::{ObjectId, Structure, Position};
use serde::{Serialize, Deserialize};

use crate::{minion::MinionType, task::Action, util::path::CreepPath, action::ActionStep};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameMemory {
    // Administrative
    pub last_managers_tick: u32,
    pub needs_deserialized: bool,

    // Memory
    pub creeps: HashMap<String, CreepMemory>,
    pub rooms: HashMap<String, RoomMemory>,
    pub structure_memories: HashMap<ObjectId<Structure>, StructureMemory>,

    // Task queues
    pub room_task_queues: HashMap<String, HashMap<MinionType, Vec<Action>>>,
}

impl GameMemory {
    pub fn default() -> Self {
        GameMemory { 
            needs_deserialized: true,
            last_managers_tick: 0,
            creeps: HashMap::new(),
            rooms: HashMap::new(),
            structure_memories: HashMap::new(),
            room_task_queues: HashMap::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RoomMemory {
    controller_level: usize,
}

impl Default for RoomMemory {
    fn default() -> Self {
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
    pub worker_type: MinionType,
    pub current_path: Option<CreepPath>,
    pub last_position: Option<Position>,
    pub current_task: Action,
    pub current_task_step: Option<ActionStep>,
}

impl Default for CreepMemory {
    fn default() -> CreepMemory {
        CreepMemory {
            worker_type: MinionType::SimpleWorker,
            last_position: None,
            current_path: None,
            current_task: Action::Idle,
            current_task_step: None,
        }
    }
}

impl Default for &CreepMemory {
    fn default() -> &'static CreepMemory {
        &CreepMemory {
            worker_type: MinionType::SimpleWorker,
            last_position: None,
            current_path: None,
            current_task: Action::Idle,
            current_task_step: None,
        }
    }
}
