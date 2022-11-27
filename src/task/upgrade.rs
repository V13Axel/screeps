use log::info;
use screeps::{StructureController, ObjectId, game, Room, Creep, Source, ResourceType, find, SharedCreepProperties, HasId, HasTypedId, RawObjectId, ReturnCode, Position};

use crate::{mem::CreepMemory, role::CreepAction, util::path::{CreepPath, MovementDistance}, minion::MinionType};

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
