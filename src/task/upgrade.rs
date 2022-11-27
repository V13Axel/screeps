use log::debug;
use screeps::{StructureController, ObjectId, game, Room, Creep, Source, ResourceType, find, SharedCreepProperties, HasId, HasTypedId, RawObjectId, ReturnCode, Position, MaybeHasTypedId};

use crate::{mem::CreepMemory, action::CreepAction, minion::MinionType};

use super::{TaskProps, TaskStyle, Task};

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
        debug!("Running {:?}", creep.name());
        if self.is_harvesting {
            debug!("Is harvesting");
            let mut sources: Vec<Source> = creep.room().unwrap().find(find::SOURCES);
            let source_id = sources.pop().expect("No sources in room?!").id();
            CreepAction::harvest(creep, &source_id, memory);
            if creep.store().get_free_capacity(Some(ResourceType::Energy)) == 0 {
                debug!("Harvesting stopped");
                self.is_harvesting = false;
            }
        } else {
            if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
                debug!("Creep has energy");
                let controller = self.resolve();

                match creep.upgrade_controller(&controller) {
                    ReturnCode::NotInRange => {
                        debug!("Not in range, moving");
                        CreepAction::move_near(creep, Position::from(controller.pos()), memory);
                    },
                    ReturnCode::NotEnough => {
                        debug!("Not enough energy, moving closer");
                        let mut sources: Vec<Source> = creep.room().unwrap().find(find::SOURCES);
                        let source_id = sources.pop().expect("No sources in room?!").id();

                        self.is_harvesting = true;
                        CreepAction::harvest(creep, &source_id, memory);
                    },
                    ReturnCode::Ok => {
                        debug!("Upgrade succeeded");
                    },
                    r => {
                        debug!("{:?}", r);
                    }
                }
            } else {
                debug!("grrrr");
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

    fn is_finished(&self) -> bool {
        false
    }

    fn needs_creeps(&mut self) -> bool {
        self.props.clean_up_workers();
        self.props.worked_by.len() < 4
    }

    fn assign_creep(&mut self, creep: &Creep) {
        self.props.worked_by.push(creep.try_id().unwrap())
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
