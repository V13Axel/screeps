use std::collections::HashMap;

use log::debug;
use screeps::{Creep, SharedCreepProperties, game};
use serde::{Serialize, Deserialize};
use crate::mem::{CreepMemory, GameMemory};
use crate::role::CreepPurpose;
use wasm_bindgen::JsValue;

use crate::{minion, task::Task};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CreepWorkerType {
    SimpleWorker,
    Upgrader,
    Harvester,
}

pub fn run_creep(creep: &Creep, memory: &mut CreepMemory) {
    debug!("Running {:?}", creep.name());
    if memory.current_path.is_some() {
        let path = memory.current_path.to_owned().unwrap();
        memory.current_path = match creep.move_by_path(&JsValue::from_str(&path.value)) {
            screeps::ReturnCode::Ok => Some(path),
            screeps::ReturnCode::Tired => Some(path),
            _ => None
        }
    }

    let worker_type = memory.worker_type.to_owned();

    match worker_type {
        minion::CreepWorkerType::SimpleWorker => {
            match memory.current_task {
                Task::Harvest { node, .. } => CreepPurpose::harvest(creep, &node, memory),
                Task::Upgrade { controller, .. } => CreepPurpose::upgrade(creep, &controller, memory),
                Task::Deposit { dest, .. } => CreepPurpose::deposit(creep, &dest.resolve().unwrap(), memory),
                Task::Build { site, .. } => CreepPurpose::build(creep, &site.resolve().unwrap(), memory),
                _ => {
                    // Basically ... If it's not one of the above, we'll just skip it
                    CreepPurpose::idle(creep, memory)
                }
            }     
        },
        minion::CreepWorkerType::Upgrader => {
            match memory.current_task {
                Task::Idle =>  CreepPurpose::idle(creep, memory),
                Task::Harvest { node, .. } => CreepPurpose::harvest(creep, &node, memory),
                Task::Upgrade { controller, .. } => CreepPurpose::upgrade(creep, &controller, memory),
                _ => {
                    CreepPurpose::idle(creep, memory);
                }
            }
        },
        minion::CreepWorkerType::Harvester => {
            match memory.current_task {
                Task::Idle => CreepPurpose::idle(creep, memory),
                Task::Harvest { node, .. } => CreepPurpose::harvest(creep, &node, memory),
                Task::Deposit { dest, .. } => CreepPurpose::deposit(creep, &dest.resolve().unwrap(), memory),
                _ => CreepPurpose::idle(creep, memory),
            }
        }
    }
}

pub fn run_creeps(memories: &mut HashMap<String, CreepMemory>) {
    for creep in game::creeps().values() {
        let name = creep.name();
        let memory = memories.entry(name).or_default();
        run_creep(&creep, memory);
    }
}

pub fn clean_up_dead_creeps(game_memory: &mut GameMemory) {
    game_memory.creeps.retain(
        |name, _| game::creeps()
            .values()
            .map(
                |creep| creep.name()
            )
            .collect::<Vec<String>>()
            .contains(name)
    );
}
