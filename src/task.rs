use std::fmt::{Debug, Display};

use dyn_clone::DynClone;
use log::{info, debug};
use screeps::{Creep, ObjectId, Room, ReturnCode, ResourceType, Position, StructureController, RawObjectId, HasId, SharedCreepProperties, Source, HasTypedId, find, game};
use serde::{Serialize, Deserialize};

use crate::{mem::CreepMemory, util::path::{CreepPath, MovementDistance}, minion::MinionType, role::CreepAction};

mod upgrade;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStyle {
    Perpetual,
    Once,
}

#[derive(Debug, Serialize, Clone)]
pub struct TaskProps {
    #[serde(skip_serializing)]
    target: Option<RawObjectId>,

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
    // Actionable results
    fn run(&mut self, creep: &Creep, memory: &mut CreepMemory);
    fn get_path_to(&self, creep: &Creep, memory: &mut CreepMemory) -> CreepPath;

    // Target
    fn get_target(&self) -> Option<RawObjectId>;
    fn set_target(&mut self, target: RawObjectId);

    // Props
    fn get_props(&self) -> TaskProps;
    fn set_props(&mut self, props: TaskProps);

    // Do we need creeps?
    fn needs_creeps(&self) -> bool;

    // What kind do we need?
    fn needed_type(&self) -> MinionType;

    // Has it been completed?
    fn is_finished(&self) -> bool;
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
    pub props: TaskProps,
    pub is_harvesting: bool,
}

impl Upgrade {
    pub fn resolve(&self) -> StructureController {
        let controller_id: ObjectId<StructureController> = match self.get_target() {
            Some(target) => target.into(),
            None => { 
                let mut rooms: Vec<Room> = game::rooms().values().into_iter().collect();
                let room = rooms.pop().unwrap();

                room.controller().unwrap().id()
            }
        };
        
        controller_id.resolve().expect("How the hell did you manage that")
    }
    pub fn for_room(room: &Room) -> Self {
        let controller = room.controller().unwrap().to_owned();

        Self {
            props: TaskProps {
                target: Some(controller.raw_id()),
                style: TaskStyle::Perpetual,
                ..Default::default()
            },
            is_harvesting: false
        }
    }
}

impl Task for Upgrade {
    fn run(&mut self, creep: &Creep, memory: &mut CreepMemory) {
        info!("Running {:?}", creep.name());
        if self.is_harvesting {
            info!("Is harvesting");
            let mut sources: Vec<Source> = creep.room().unwrap().find(find::SOURCES);
            let source_id = sources.pop().expect("No sources in room?!").id();
            CreepAction::harvest(creep, &source_id, memory);
            if creep.store().get_free_capacity(Some(ResourceType::Energy)) == 0 {
                info!("Harvesting stopped");
                self.is_harvesting = false;
            }
        } else {
            if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
                info!("Creep has energy");
                let controller = self.resolve();

                match creep.upgrade_controller(&controller) {
                    ReturnCode::NotInRange => {
                        info!("Not in range, moving");
                        CreepAction::move_near(creep, Position::from(controller.pos()), memory);
                    },
                    ReturnCode::NotEnough => {
                        info!("Not enough energy, moving closer");
                        let mut sources: Vec<Source> = creep.room().unwrap().find(find::SOURCES);
                        let source_id = sources.pop().expect("No sources in room?!").id();

                        self.is_harvesting = true;
                        CreepAction::harvest(creep, &source_id, memory);
                    },
                    ReturnCode::Ok => {
                        info!("Upgrade succeeded");
                    },
                    r => {
                        info!("{:?}", r);
                    }
                }
            } else {
                info!("grrrr");
                self.is_harvesting = true;
            }
        };
    }

    fn set_target(&mut self, target: RawObjectId) {
        self.props.target = Some(target);
    }

    fn get_target(&self) -> Option<RawObjectId> {
        self.props.target
    }

    fn get_path_to(&self, creep: &Creep, memory: &mut CreepMemory) -> CreepPath {
        CreepPath::determine(
            creep.room()
                .unwrap(),
            &creep.pos(), 
            &self.resolve().pos(), 
            MovementDistance::At
        )
    }

    fn is_finished(&self) -> bool {
        false
    }

    fn needs_creeps(&self) -> bool {
        true
    }

    fn needed_type(&self) -> MinionType {
        MinionType::Upgrader
    }

    fn get_props(&self) -> TaskProps {
        self.props.to_owned()
    }

    fn set_props(&mut self, props: TaskProps) {
        self.props = props;
    }
}
