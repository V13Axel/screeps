use std::collections::HashMap;
use std::fmt::Display;

use log::info;
use screeps::{Creep, game, MaybeHasTypedId, SharedCreepProperties, Position, Room};
use serde::{Serialize, Deserialize};
use crate::{mem::{CreepMemory, GameMemory}, task::{Upgrade, TaskProps}};

use wasm_bindgen::JsValue;


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

impl Display for MinionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimpleWorker => write!(f, "Worker"),
                Self::Upgrader => write!(f, "Upgrader"),
               Self::Harvester => write!(f, "Harvester"),
        }
    }
}

pub fn run_creep(creep: &Creep, memory: &mut CreepMemory) {
    if memory.current_path.is_some() {
        info!("Moving {:?}", creep.name());
        let path = memory.current_path.to_owned().unwrap();

        let mut unserialized = Room::deserialize_path(&path.value);

        info!("{:?}", unserialized);

        let last_position = unserialized.pop().unwrap();
        let creep_position = creep.pos();

        info!("({:?}, {:?}) ({:?},{:?})", last_position.x, last_position.y, creep_position.x(), creep_position.y());


        if last_position.x == <u8 as Into<u32>>::into(creep_position.x()) && last_position.y == <u8 as Into<u32>>::into(creep_position.y()) {
            memory.current_path = None;
        } else {
            memory.current_path = match creep.move_by_path(&JsValue::from_str(&path.value)) {
                screeps::ReturnCode::Ok => Some(path),
                screeps::ReturnCode::Tired => Some(path),
                r => {
                    info!("Returncode of move: {:?}", r);

                    None
                }
            };
        }

        return;
    }

    info!("{:?}", memory.current_path);
    if memory.current_task.is_some() {
        let mut task = memory.current_task.to_owned();

        task.as_mut().unwrap().run(creep, memory);

        memory.current_task = task;
    } else {
        memory.current_task = Some(Box::new(Upgrade { is_harvesting: false, props: TaskProps::default() }));
    }

    // let worker_type = memory.worker_type.to_owned();

    // info!("{:?}", memory.current_task);

    // match worker_type {
    //     minion::MinionType::SimpleWorker => {
    //         match memory.current_task {
    //             Task::Harvest { node, .. } => CreepAction::harvest(creep, &node, memory),
    //             Task::Upgrade { controller, .. } => CreepAction::upgrade(creep, &controller, memory),
    //             Task::Deposit { dest, .. } => CreepAction::deposit(creep, &dest.resolve().unwrap(), memory),
    //             Task::Build { site, .. } => CreepAction::build(creep, &site.resolve().unwrap(), memory),
    //             _ => {
    //                 // Basically ... If it's not one of the above, we'll just upgrade the
    //                 // controller
    //                 CreepAction::idle(creep, memory);
    //             }
    //         }     
    //     },
    //     minion::MinionType::Upgrader => {
    //         match memory.current_task {
    //             Task::Idle =>  CreepAction::idle(creep, memory),
    //             Task::Harvest { node, .. } => CreepAction::harvest(creep, &node, memory),
    //             Task::Upgrade { controller, .. } => CreepAction::upgrade(creep, &controller, memory),
    //             _ => {
    //                 CreepAction::idle(creep, memory);
    //             }
    //         }
    //     },
    //     minion::MinionType::Harvester => {
    //         match memory.current_task {
    //             Task::Idle => CreepAction::idle(creep, memory),
    //             Task::Harvest { node, .. } => CreepAction::harvest(creep, &node, memory),
    //             Task::Deposit { dest, .. } => CreepAction::deposit(creep, &dest.resolve().unwrap(), memory),
    //             _ => CreepAction::idle(creep, memory),
    //         }
    //     }
    // }
}

pub fn run_creeps(memories: &mut HashMap<String, CreepMemory>) {
    for creep in game::creeps().values() {
        if let Some(_) = creep.try_id() {
            run_creep(
                &creep,
                memories.entry(creep.name())
                    .or_default()
            );
        }
    }
}

pub fn clean_up_dead_creeps(game_memory: &mut GameMemory) {
    let existing_names = game::creeps()
        .values()
        .map(|creep| creep.name())
        .collect::<Vec<String>>();

    game_memory.creeps.retain(|name, _| existing_names.contains(name));
}
