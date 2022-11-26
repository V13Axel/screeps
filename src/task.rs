use std::fmt::{Debug, Display};

use dyn_clone::DynClone;
use log::info;
use screeps::{Path, Creep, RoomObject, ObjectId, Room};
use serde::{Serialize, Deserialize};

use crate::{mem::CreepMemory, util::path::CreepPath, minion::MinionType};

mod upgrade;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStyle {
    Perpetual,
    Once,
}

#[derive(Debug, Serialize, Clone)]
pub struct TaskProps {
    #[serde(skip_serializing)]
    target: Option<Box<RoomObject>>,

    style: TaskStyle,
    min_room_level: usize,
    worked_by: Vec<ObjectId<Creep>>
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
    fn run(&mut self, creep: &Creep, memory: &mut CreepMemory);
    fn set_target(&mut self, target: RoomObject);
    fn get_target(&self) -> Option<Box<RoomObject>>;
    fn get_path_to(&self, creep: &Creep, memory: &mut CreepMemory) -> CreepPath;
    fn is_finished(&self) -> bool;
    fn needs_creeps(&self) -> bool;
    fn needed_type(&self) -> MinionType;
    fn get_props(&self) -> TaskProps;
}

dyn_clone::clone_trait_object!(Task);

impl Serialize for dyn Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.get_props().serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub struct Upgrade {
    props: TaskProps,
}

impl Upgrade {
    pub fn for_room(room: &Room) -> Self {
        let controller = room.controller().unwrap().to_owned();

        Self {
            props: TaskProps {
                target: Some(Box::new(controller.into())),
                style: TaskStyle::Perpetual,
                ..Default::default()
            }
        }
    }
}

impl Task for Upgrade {
    fn run(&mut self, creep: &Creep, memory: &mut CreepMemory) {
        info!("Would upgrade");
    }

    fn needed_type(&self) -> MinionType {
        MinionType::Upgrader
    }

    fn get_target(&self) -> Option<Box<RoomObject>> {
        self.props.target.to_owned()
    }

    fn set_target(&mut self, target: RoomObject) {
        self.props.target = Some(Box::new(target));
    }

    fn get_path_to(&self, creep: &Creep, memory: &mut CreepMemory) -> CreepPath {
        match self.get_target() {
            Some(target) => CreepPath::determine(
                creep.room()
                    .unwrap(),
                &creep.pos(), 
                &target.pos(), 
            ),
            None => CreepPath::from(Path::Serialized("".to_string()))
        }
    }

    fn is_finished(&self) -> bool {
        false
    }

    fn needs_creeps(&self) -> bool {
        true
    }

    fn get_props(&self) -> TaskProps {
        self.props.to_owned()
    }
}
