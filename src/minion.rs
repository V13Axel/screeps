use std::{fmt::Display, collections::HashMap};

use log::info;
use screeps::{game, SharedCreepProperties, Creep};
use serde::{Serialize, Deserialize};
use crate::mem::GameMemory;


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
        let creeps = game::creeps()
            .values()
            .map(|creep| (creep.name(), creep))
            .collect::<HashMap<String, Creep>>();

        game_memory.creeps.iter_mut().for_each(|(creep_name, creep_memory)| {
            if let Some(creep) = creeps.get(&creep_name.to_owned()) {
                let task = creep_memory.current_task.to_owned();
                task.run(creep, creep_memory);
            }
        })
    }

    pub fn clean_up_dead_creeps(game_memory: &mut GameMemory) {
        let existing_names = game::creeps()
            .values()
            .map(|creep| creep.name())
            .collect::<Vec<String>>();

        game_memory.creeps.retain(|name, _| existing_names.contains(name));
    }
}
