use log::{debug, info};
use screeps::{Creep, ObjectId, ResourceType, ReturnCode, Source, HasPosition, RoomPosition, Position, SharedCreepProperties, StructureController, ConstructionSite, find, HasTypedId};
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;

use crate::{util::path::{CreepPath, MovementDistance}, mem::CreepMemory, task::Action};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ActionStep {
    Harvesting,
    CollectingFrom,
    Upgrading,
    Building,
    Depositing,
    Moving(Position),
    Panic(ReturnCode),
}

pub struct CreepAction;

impl CreepAction {
    // Gets you to the position
    #[allow(dead_code)]
    pub fn move_to(creep: &Creep, position: Position, mut memory: &mut CreepMemory) { 
        memory.current_path = if creep.pos().is_equal_to(position) {
            None
        } else {
            Self::do_movement(creep, &position, &memory.current_path, MovementDistance::At)
        };
    }
    
    // Gets you next to the position
    pub fn move_near(creep: &Creep, position: Position, memory: &mut CreepMemory) {
        memory.current_path = if creep.pos().is_near_to(position) {
            None
        } else {
            Self::do_movement(creep, &position, &memory.current_path, MovementDistance::Near)
        };
    }

    fn do_movement(creep: &Creep, position: &Position, current_path: &Option<CreepPath>, distance: MovementDistance) -> Option<CreepPath> {
        // info!("Running movement for {:?}", creep.name());
        let path = match current_path {
            Some(path) => path.to_owned(),
            None => {
                CreepPath::determine(
                    creep.room()
                    .unwrap(),
                        &RoomPosition::from(creep.pos()), 
                        &RoomPosition::from(position), 
                    distance
                )
            }
        };

        let result = creep.move_by_path(&JsValue::from_str(&path.value));

        debug!("-------\nMovement return code - {:?}\nPath - {:?}",  result, path);

        match result {
            ReturnCode::Ok => { 
                Some(path) 
            },
            _ => {
                None
            }
        }
    }

    pub fn harvest(creep: &Creep, source_id: &ObjectId<Source>, memory: &mut CreepMemory) {
        memory.current_task_step = match memory.current_task_step {
            Some(step) => match step {
                ActionStep::Panic(return_code) => {
                    creep.say(&format!("{:?}", return_code), true);

                    Some(ActionStep::Harvesting)
                }
                ActionStep::Harvesting => {
                    if let Action::Harvest(source_id) = memory.current_task {
                        if creep.store().get_free_capacity(Some(ResourceType::Energy)) == 0 {
                            Some(ActionStep::Depositing)
                        } else {
                            if let Some(source) = source_id.resolve() {
                                let harvest_result = creep.harvest(&source);
                                // info!("{:?}", harvest_result);
                                match harvest_result {
                                    ReturnCode::Ok => Some(ActionStep::Harvesting),
                                    ReturnCode::Full => Some(ActionStep::Depositing),
                                    ReturnCode::NotInRange => {
                                        Self::move_near(creep, source.pos(), memory);

                                        Some(ActionStep::Harvesting)
                                    },
                                    ReturnCode::Tired => Some(ActionStep::Harvesting),
                                    _ => Some(ActionStep::Panic(harvest_result))
                                }
                            } else {
                                // Dunno why we wouldn't be able to resolve a source ID we already
                                // have, but weirder things have happened.
                                memory.current_task = Action::Idle;

                                None
                            }
                        }
                    } else {
                        // Dunno when we'd ever get an invalid source, but ... hey, for now no
                        // assumptions.
                        memory.current_task = Action::Idle;
                        None
                    }
                },
                ActionStep::Depositing => {
                    if let Some(spawn) = creep.room().unwrap().find(find::MY_SPAWNS).pop() {
                        if creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0 {
                            Some(ActionStep::Harvesting)
                        } else {
                            let return_code = creep.transfer(&spawn, ResourceType::Energy, None);
                            match return_code {
                                ReturnCode::Ok => Some(ActionStep::Harvesting),
                                ReturnCode::Full => Some(ActionStep::Depositing),
                                ReturnCode::Tired => Some(ActionStep::Depositing),
                                ReturnCode::NotInRange => {
                                    Self::move_near(creep, spawn.pos(), memory);

                                    Some(ActionStep::Depositing)
                                },
                                _ => Some(ActionStep::Panic(return_code)),
                            }
                        }
                    } else {
                        memory.current_task = Action::Idle;
                        None
                    }
                },
                _ => Some(ActionStep::Harvesting),
            },
            None => Some(ActionStep::Harvesting),
        }
    }

    pub fn upgrade(creep: &Creep, controller_id: &ObjectId<StructureController>, memory: &mut CreepMemory) {
        let controller = controller_id.resolve().unwrap();

        memory.current_task_step = match memory.current_task_step {
            Some(step) => match step {
                ActionStep::Harvesting => {
                    if creep.store().get_free_capacity(Some(ResourceType::Energy)) == 0 {
                        Some(ActionStep::Upgrading)
                    } else {
                        let source = creep.room().unwrap().find(find::SOURCES).pop().unwrap();
                        let harvest_result = creep.harvest(&source);
                        // info!("{:?}", harvest_result);
                        match harvest_result {
                            ReturnCode::Ok => Some(ActionStep::Harvesting),
                            ReturnCode::Full => Some(ActionStep::Upgrading),
                            ReturnCode::NotInRange => {
                                Self::move_near(creep, source.pos(), memory);

                                Some(ActionStep::Harvesting)
                            },
                            ReturnCode::Tired => Some(ActionStep::Harvesting),
                            _ => Some(ActionStep::Panic(harvest_result))
                        }
                    }
                }
                ActionStep::Upgrading => {
                    if creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0 {
                        Some(ActionStep::Harvesting)
                    } else {
                        match creep.upgrade_controller(&controller) {
                            ReturnCode::NotInRange => {
                                CreepAction::move_near(creep, Position::from(controller.pos()), memory);
                                Some(ActionStep::Upgrading)
                            },
                            ReturnCode::NotEnough => {
                                Some(ActionStep::Harvesting)
                            },
                            ReturnCode::Ok => {
                                Some(ActionStep::Upgrading)
                            },
                            r => {
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

    pub fn build(creep: &Creep, site: &ObjectId<ConstructionSite>, memory: &mut CreepMemory) {
        info!("Build called");
    }
}
