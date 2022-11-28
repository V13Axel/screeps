use std::{fmt::{Debug, Display}, collections::HashMap};

use dyn_clone::DynClone;

use screeps::{Creep, ObjectId, RawObjectId};
use serde::{Serialize, Deserialize};

use crate::{mem::CreepMemory, minion::MinionType};

pub mod upgrade;
pub mod harvest;

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
    worked_by: Vec<ObjectId<Creep>>
}

impl TaskProps {
    pub fn includes(&self, creep_id: ObjectId<Creep>) -> bool {
        self.worked_by.contains(&creep_id)
    }

    pub fn clean_up_workers(&mut self) {
        self.worked_by.retain(|creep_id| creep_id.resolve().is_some())
    }
}

impl Display for TaskProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for TaskProps {
    fn default() -> Self {
        Self {
            target: None,
            style: TaskStyle::Once,
            min_room_level: 1,
            worked_by: vec![],
        }
    }
}

pub trait Task: Debug + DynClone {
    // Actionable results
    fn run(&mut self, creep: &Creep, memory: &mut CreepMemory);
    fn run_workers(&mut self, creep_memories: &mut HashMap<String, CreepMemory>);

    // Target
    fn get_target(&self) -> Option<RawObjectId>;
    fn set_target(&mut self, target: RawObjectId);

    // Props
    fn get_props(&self) -> TaskProps;
    fn set_props(&mut self, props: TaskProps);
    fn get_workable_name(&self) -> WorkableTask;

    // Do we need creeps?
    fn needs_creeps(&mut self) -> bool;

    // What kind do we need?
    fn needed_type(&self) -> MinionType;

    // Has it been completed?
    fn is_finished(&self) -> bool;
    fn assign_creep(&mut self, creep: &Creep);
}

dyn_clone::clone_trait_object!(Task);

impl Serialize for dyn Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.get_props().serialize(serializer)
    }
}

