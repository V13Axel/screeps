use log::info;
use screeps::{Room, Creep, ResourceType, find, SharedCreepProperties, HasTypedId, ReturnCode, Position};

use crate::{mem::CreepMemory, action::{CreepAction, ActionStep}};

use super::{TaskProps, Action};

#[derive(Debug, Clone)]
pub struct Upgrade {
    pub props: TaskProps,
    pub is_harvesting: bool,
}

impl Upgrade {
    pub fn for_room(room: &Room) -> Action {
        let controller = room.controller().unwrap().to_owned();

        Action::Upgrade(controller.id())
    }

    fn run(&mut self, creep: &Creep, memory: &mut CreepMemory) {
        if let Action::Upgrade(controller_id) = memory.current_task {
            let controller = controller_id.resolve().unwrap();

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
        } else {
            info!("A creep was asked to run upgrade, but isn't upgrading!!");
        }
    }
}
