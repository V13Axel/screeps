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
    pub fn run(game_memory: &mut GameMemory) {
        for (room, room_tasks) in game_memory.room_task_queues.iter_mut() {
            for (minion_type, tasks) in room_tasks.iter_mut() {
                for task in tasks.iter_mut() {
                    info!("Running task {:?}", task);
                    task.run_workers(&mut game_memory.creeps);
                }
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
}
