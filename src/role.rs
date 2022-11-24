use log::{warn, debug, info};
use screeps::{Creep, ObjectId, StructureController, ResourceType, ReturnCode, SharedCreepProperties, Source, HasPosition, RoomPosition, find, HasTypedId, Position, Transferable, ConstructionSite};
use wasm_bindgen::JsValue;

use crate::{util::path::CreepPath, mem::CreepMemory, minion::MinionType, task::Task};

pub struct CreepAction;

impl CreepAction {
    pub fn idle(creep: &Creep, memory: &mut CreepMemory) {
        let controller = creep.room().unwrap().controller().unwrap().id();
        memory.current_task = Task::Upgrade { controller, worked_by: vec![] }
        // Self::move_near(
        //     creep, 
        //     creep.room().unwrap().find(find::MY_SPAWNS)[0].pos(), 
        //     memory
        // );
    }

    // Gets you to the position
    #[allow(dead_code)]
    pub fn move_to(creep: &Creep, position: Position, mut memory: &mut CreepMemory) { 
        memory.current_path = if creep.pos().is_equal_to(position) {
            None
        } else {
            Self::do_movement(creep, &position, &memory.current_path)
        };
    }
    
    // Gets you next to the position
    pub fn move_near(creep: &Creep, position: Position, memory: &mut CreepMemory) {
        memory.current_path = if creep.pos().is_near_to(position) {
            None
        } else {
            Self::do_movement(creep, &position, &memory.current_path)
        };
    }


    fn do_movement(creep: &Creep, position: &Position, current_path: &Option<CreepPath>) -> Option<CreepPath> {
        let path = match current_path {
            Some(path) => path.to_owned(),
            None => {
                CreepPath::from(creep.room()
                    .unwrap()
                    .find_path(
                        &RoomPosition::from(creep.pos()), 
                        &RoomPosition::from(position), 
                        None
                    )
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

    pub fn upgrade(creep: &Creep, controller_id: &ObjectId<StructureController>, memory: &mut CreepMemory) {
        debug!("Trying to upgrade");
        let keep_job = if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
            debug!("Creep has energy");
            match controller_id.resolve() {
                Some(controller) => {
                    debug!("Controller found");
                    let r = creep.upgrade_controller(&controller);
                    if r == ReturnCode::NotInRange {
                        debug!("Trying to move closer to controller");
                        Self::move_near(creep, controller.pos(), memory);
                        true
                    } else if r != ReturnCode::Ok {
                        warn!("couldn't upgrade: {:?}", r);
                        false
                    } else {
                        true
                    }
                }
                None => {
                    debug!("No controller found. ... what?");

                    false
                },
            }
        } else {
            debug!("Energy is empty");
            false
        };

        if !keep_job {
            let node = creep.room().unwrap().find(find::SOURCES)[0].id();
            memory.current_path = None;
            memory.current_task = Task::Harvest { node , worked_by: vec![], space_limit: 0 };
            memory.worker_type = MinionType::SimpleWorker;

            Self::harvest(creep, &node, memory)
        }
    }

    pub fn harvest(creep: &Creep, source_id: &ObjectId<Source>, memory: &mut CreepMemory) { 
        debug!("Trying to harvest");
        let keep_job = if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            debug!("Creep {:?} has some empty space", creep.name());
            let source = match source_id.resolve() {
                Some(source) => {
                    debug!("Source found");
                    if creep.pos().is_near_to(source.pos()) {
                        debug!("Creep is near source");
                        let r = creep.harvest(&source);
                        if r != ReturnCode::Ok {
                            debug!("couldn't harvest: {:?}", r);
                            false
                        } else {
                            true
                        }
                    } else {
                        let result = creep.pos().is_near_to(source.pos());
                        let range = creep.pos().get_range_to(source.pos());
                        debug!("Moving to source, got {:?}, range {:?}", result, range);
                        Self::move_near(creep, source.pos(), memory);
                        true
                    }
                }
                None => false,
            };

            source
        } else {
            debug!("Creep is full");
            false
        };

        if !keep_job {
            debug!("{} not keeping job", creep.name());
            memory.current_path = None;
            memory.current_task = Task::Upgrade { controller: creep.room().unwrap().controller().unwrap().id(), worked_by: vec![] };
            memory.worker_type = MinionType::SimpleWorker;
        }
    }

    pub fn build(creep: &Creep, site: &ConstructionSite, memory: &mut CreepMemory) {
        if creep.pos().is_near_to(site.pos()) {
            memory.current_task = match creep.build(&site) {
                ReturnCode::Ok => Task::Idle,
                code => {
                    info!("{:?} - Building code {:?}", creep.name(), code);
                
                    memory.current_task.to_owned()
                }
            }
        } else {
            Self::move_near(creep, site.pos(), memory);
        }
    }

    pub fn deposit(creep: &Creep, dest: &impl Transferable, memory: &mut CreepMemory) {
        if creep.pos().is_near_to(dest.pos()) {
            memory.current_path = None;

            match creep.transfer(dest, ResourceType::Energy, None) {
                ReturnCode::Ok => {
                    memory.current_task = Task::Idle;
                },
                code => {
                    info!("{:?}", code);
                }
            }
        } else {
            Self::move_near(creep, dest.pos(), memory)
        }
    }
}
