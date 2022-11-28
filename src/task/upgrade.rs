use std::collections::HashMap;

use log::{debug, info};
use screeps::{StructureController, ObjectId, game, Room, Creep, Source, ResourceType, find, SharedCreepProperties, HasId, HasTypedId, RawObjectId, ReturnCode, Position, MaybeHasTypedId};

use crate::{mem::CreepMemory, action::{CreepAction, ActionStep}, minion::MinionType};

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
    fn get_workable_name(&self) -> super::WorkableTask {
        super::WorkableTask::Upgrade
    }

    fn run_workers(&mut self, creep_memories: &mut HashMap<String, CreepMemory>) {
        self.props.clean_up_workers();

        for creep_id in self.props.worked_by.to_owned().iter() {
            if let Some(creep) = creep_id.resolve() {
                self.run(&creep, creep_memories.entry(creep.name()).or_default());
            }
        }
    }

    fn run(&mut self, creep: &Creep, memory: &mut CreepMemory) {
        info!("Running {:?}", creep.name());
        memory.current_task_step = match memory.current_task_step {
            Some(step) => match step {
                ActionStep::Harvesting => {
                    if creep.store().get_free_capacity(Some(ResourceType::Energy)) == 0 {
                        Some(ActionStep::Upgrading)
                    } else {
                        let mut sources = creep.room().unwrap().find(find::SOURCES);
                        let source_id = sources.pop().unwrap();

                        CreepAction::harvest(creep, &source_id.id(), memory);

                        Some(ActionStep::Harvesting)
                    }
                },
                ActionStep::Upgrading => {
                    if creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0 {
                        Some(ActionStep::Harvesting)
                    } else {
                        info!("Creep has energy");
                        let controller = self.resolve();

                        match creep.upgrade_controller(&controller) {
                            ReturnCode::NotInRange => {
                                info!("Not in range, moving");
                                CreepAction::move_near(creep, Position::from(controller.pos()), memory);
                                Some(ActionStep::Upgrading)
                            },
                            ReturnCode::NotEnough => {
                                Some(ActionStep::Harvesting)
                            },
                            ReturnCode::Ok => {
                                info!("Upgrade succeeded");
                                Some(ActionStep::Upgrading)
                            },
                            r => {
                                info!("Upgrade error by {:?}: {:?}", creep.name(), r);
                                Some(ActionStep::Upgrading)
                            }
                        }
                        
                    }
                },
                ActionStep::CollectingFrom => {
                    // For now, this just sets to harvesting... but later maybe it'll
                    // find the nearest full container and use it?
                    Some(ActionStep::Harvesting)
                },
                _ => Some(ActionStep::Harvesting)
            },
            None => Some(ActionStep::Harvesting)
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
        info!("Upgrade workers length: {}", self.props.worked_by.len());
        self.props.worked_by.len() < 4
    }

    fn assign_creep(&mut self, creep: &Creep) {
        info!("Assigning creep to upgrade: {:?}", creep.name());
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
