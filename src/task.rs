use std::fmt::Debug;

use screeps::{ObjectId, RawObjectId, StructureController, StructureSpawn, ConstructionSite, Source, Creep};
use serde::{Serialize, Deserialize};

use crate::{mem::CreepMemory, action::CreepAction};

pub mod upgrade;
pub mod harvest;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Action {
    Idle,
    Upgrade(ObjectId<StructureController>),
    Harvest(ObjectId<Source>),
    Build(ObjectId<ConstructionSite>),
}

impl Action {
    pub fn run(&self, creep: &Creep, memory: &mut CreepMemory) {
        match self {
            Self::Idle => {},
            Self::Upgrade(controller_id) => CreepAction::upgrade(creep, controller_id, memory),
            Self::Harvest(source) => CreepAction::harvest(creep, source, memory),
            Self::Build(site) => CreepAction::build(creep, site, memory),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStyle {
    Perpetual,
    Once,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WorkableTask {
    Idle,
    Upgrade,
    Harvest,
    Build,
}

#[derive(Debug, Serialize, Clone)]
pub struct TaskProps {
    target: Option<RawObjectId>,
    style: TaskStyle,
    min_room_level: usize,
}

impl Default for TaskProps {
    fn default() -> Self {
        Self {
            target: None,
            style: TaskStyle::Once,
            min_room_level: 1,
        }
    }
}
