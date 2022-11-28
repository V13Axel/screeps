use std::collections::HashMap;
use std::fmt::Display;

use log::{debug, info};
use screeps::{Creep, game, MaybeHasTypedId, SharedCreepProperties, Room};
use serde::{Serialize, Deserialize};
use crate::{mem::{CreepMemory, GameMemory}, task::Task};

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

pub struct Minions;

impl Minions {
    pub fn run_creep(creep: &Creep, task: &mut dyn Task, memory: &mut CreepMemory) {
        // // TODO: Clean this up. Basically just a sanity check for whether the creep successfully
        // // actually moved.
        // if memory.current_path.is_some() {
        //     debug!("Moving {:?}", creep.name());
        //     let path = memory.current_path.to_owned().unwrap();
        //
        //     let mut unserialized = Room::deserialize_path(&path.value);
        //
        //     debug!("{:?}", unserialized);
        //
        //     let last_position = unserialized.pop().unwrap();
        //     let creep_position = creep.pos();
        //
        //     debug!("({:?}, {:?}) ({:?},{:?})", last_position.x, last_position.y, creep_position.x(), creep_position.y());
        //
        //
        //     if last_position.x == <u8 as Into<u32>>::into(creep_position.x()) && last_position.y == <u8 as Into<u32>>::into(creep_position.y()) {
        //         memory.current_path = None;
        //     } else {
        //         memory.current_path = match creep.move_by_path(&JsValue::from_str(&path.value)) {
        //             screeps::ReturnCode::Ok => Some(path),
        //             screeps::ReturnCode::Tired => Some(path),
        //             r => {
        //                 debug!("Returncode of move: {:?}", r);
        //
        //                 None
        //             }
        //         };
        //     }
        //
        //     return;
        // }
        //
        // debug!("{:?}", memory.current_path);
        // if memory.current_task.is_some() {
        //     let mut task = memory.current_task.to_owned();
        //
        //     task.as_mut().unwrap().run(creep, memory);
        //
        //     memory.current_task = task;
        // }
        //
        // memory.last_position = Some(creep.pos().into());
    }

    pub fn run(game_memory: &mut GameMemory) {
        for (room, room_tasks) in game_memory.room_task_queues.iter_mut() {
            for (minion_type, tasks) in room_tasks.iter_mut() {
                for task in tasks.iter_mut() {
                    info!("Running task {:?}", task);
                    task.run_workers(&mut game_memory.creeps);
                }
            }
        }

        // for creep in game::creeps().values() {
        //     if let Some(_) = creep.try_id() {
        //         run_creep(
        //             &creep,
        //             memories.entry(creep.name())
        //                 .or_default()
        //         );
        //     }
        // }
    }

    pub fn clean_up_dead_creeps(game_memory: &mut GameMemory) {
        let existing_names = game::creeps()
            .values()
            .map(|creep| creep.name())
            .collect::<Vec<String>>();

        game_memory.creeps.retain(|name, _| existing_names.contains(name));
    }
}
