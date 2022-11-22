use std::collections::HashMap;

use log::{debug, info};
use screeps::{Creep, SharedCreepProperties, game, ObjectId, Room};
use serde::{Serialize, Deserialize};
use crate::mem::{CreepMemory, GameMemory, RoomMemory};
use crate::role::CreepPurpose;
use wasm_bindgen::JsValue;

use crate::{minion, task::Task};


// Super struct
struct Minion<Role: MinionRole> {
    role: Role,
    creep: ObjectId<Creep>,
}

// Trait that makes the struct support a role object
trait MinionRole {
    fn run(&self, creep: &Creep, memory: &CreepMemory);
    fn needed_in_room(&self, room: Room) -> u32;
}

// Implementation that passes through to roles
impl<T: MinionRole> Minion<T> {
    fn run(&self, creep: &Creep, memory: &mut CreepMemory) {
        self.role.run(creep, memory);
    }

    fn needed_in_room(&self, room: Room) -> u32 {
        self.role.needed_in_room(room)
    }
}

// Type structs
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Harvester;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Builder;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Upgrader;

// Implementations for each role
impl MinionRole for Harvester {
    fn run(&self, creep: &Creep, memory: &CreepMemory) {
        info!("Would harvest");
    }

    fn needed_in_room(&self, room: Room) -> u32 {
        return 6;
    }
}

impl MinionRole for Builder {
    fn run(&self, creep: &Creep, memory: &CreepMemory) {
        info!("Would build");
    }

    fn needed_in_room(&self, room: Room) -> u32 {
        return 1;
    }
}

impl MinionRole for Upgrader {
    fn run(&self, creep: &Creep, memory: &CreepMemory) {
        info!("Would upgrade");
    }

    fn needed_in_room(&self, room: Room) -> u32 {
        return 1;
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MinionType {
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
        minion::MinionType::SimpleWorker => {
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
        minion::MinionType::Upgrader => {
            match memory.current_task {
                Task::Idle =>  CreepPurpose::idle(creep, memory),
                Task::Harvest { node, .. } => CreepPurpose::harvest(creep, &node, memory),
                Task::Upgrade { controller, .. } => CreepPurpose::upgrade(creep, &controller, memory),
                _ => {
                    CreepPurpose::idle(creep, memory);
                }
            }
        },
        minion::MinionType::Harvester => {
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
    let existing_names = game::creeps()
        .values()
        .map(|creep| creep.name())
        .collect::<Vec<String>>();

    game_memory.creeps.retain(|name, _| existing_names.contains(name));
}
