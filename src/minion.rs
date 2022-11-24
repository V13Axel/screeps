use std::collections::HashMap;

use screeps::{Creep, game, ObjectId, MaybeHasTypedId};
use serde::{Serialize, Deserialize};
use crate::mem::{CreepMemory, GameMemory};
use crate::role::CreepAction;
use wasm_bindgen::JsValue;

use crate::{minion, task::Task};


// Type structs
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Harvester;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Builder;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Upgrader;

#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum MinionType {
    SimpleWorker,
    Upgrader,
    Harvester,
}

pub fn run_creep(creep: &Creep, memory: &mut CreepMemory) {
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
        minion::MinionType::SimpleWorker => {
            match memory.current_task {
                Task::Harvest { node, .. } => CreepAction::harvest(creep, &node, memory),
                Task::Upgrade { controller, .. } => CreepAction::upgrade(creep, &controller, memory),
                Task::Deposit { dest, .. } => CreepAction::deposit(creep, &dest.resolve().unwrap(), memory),
                Task::Build { site, .. } => CreepAction::build(creep, &site.resolve().unwrap(), memory),
                _ => {
                    // Basically ... If it's not one of the above, we'll just skip it
                    CreepAction::idle(creep, memory)
                }
            }     
        },
        minion::MinionType::Upgrader => {
            match memory.current_task {
                Task::Idle =>  CreepAction::idle(creep, memory),
                Task::Harvest { node, .. } => CreepAction::harvest(creep, &node, memory),
                Task::Upgrade { controller, .. } => CreepAction::upgrade(creep, &controller, memory),
                _ => {
                    CreepAction::idle(creep, memory);
                }
            }
        },
        minion::MinionType::Harvester => {
            match memory.current_task {
                Task::Idle => CreepAction::idle(creep, memory),
                Task::Harvest { node, .. } => CreepAction::harvest(creep, &node, memory),
                Task::Deposit { dest, .. } => CreepAction::deposit(creep, &dest.resolve().unwrap(), memory),
                _ => CreepAction::idle(creep, memory),
            }
        }
    }
}

pub fn run_creeps(memories: &mut HashMap<ObjectId<Creep>, CreepMemory>) {
    for creep in game::creeps().values() {
        if let Some(id) = creep.try_id() {
            run_creep(
                &creep,
                memories.entry(id)
                    .or_default()
            );
        }
    }
}

pub fn clean_up_dead_creeps(game_memory: &mut GameMemory) {
    let existing_names = game::creeps()
        .values()
        .map(|creep| creep.try_id())
        .filter(|id_option| id_option.is_none())
        .map(|id_option| id_option.unwrap())
        .collect::<Vec<ObjectId<Creep>>>();

    game_memory.creeps.retain(|name, _| existing_names.contains(name));
}
