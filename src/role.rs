use log::{warn, info, debug};
use screeps::{Creep, ObjectId, StructureController, ResourceType, ReturnCode, SharedCreepProperties, Source, HasPosition, RoomPosition, find, HasTypedId, Position};
use wasm_bindgen::JsValue;

use crate::{util::path::CreepPath, mem::CreepMemory, minion::CreepWorkerType, task::Task};


pub struct CreepPurpose {
    // name: String,
    // definition: Vec<String>
}

impl CreepPurpose {
    pub fn idle(creep: &Creep, memory: &mut CreepMemory) {
        Self::move_near(creep, creep.room().unwrap().find(find::MY_SPAWNS)[0].pos(), memory);
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

        // info!("{:?} - {:?}", path, result);

        match result {
            ReturnCode::Ok => { 
                Some(path) 
            },
            _ => {
                info!("-------\nMovement return code - {:?}\nPath - {:?}",  result, path);
                None
            }
        }
    }

    pub fn upgrade(creep: &Creep, controller_id: &ObjectId<StructureController>, memory: &mut CreepMemory) {
        let keep_job = if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
            match controller_id.resolve() {
                Some(controller) => {
                    let r = creep.upgrade_controller(&controller);
                    if r == ReturnCode::NotInRange {
                        // creep.move_to(&controller);
                        Self::move_near(creep, controller.pos(), memory);
                        true
                    } else if r != ReturnCode::Ok {
                        warn!("couldn't upgrade: {:?}", r);
                        false
                    } else {
                        true
                    }
                }
                None => false,
            }
        } else {
            false
        };

        if !keep_job {
            let node = creep.room().unwrap().find(find::SOURCES)[0].id();
            memory.worker_type = CreepWorkerType::SimpleWorker(Task::Harvest { node , worked_by: vec![], space_limit: 0 });
        }
    }

    pub fn harvest(creep: &Creep, source_id: &ObjectId<Source>, memory: &mut CreepMemory) { 
        info!("Trying to harvest");
        let keep_job = if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            info!("Creep {:?} has some empty space", creep.name());
            let source = match source_id.resolve() {
                Some(source) => {
                    info!("Source found");
                    if creep.pos().is_near_to(source.pos()) {
                        info!("Creep is near source");
                        let r = creep.harvest(&source);
                        if r != ReturnCode::Ok {
                            info!("couldn't harvest: {:?}", r);
                            false
                        } else {
                            true
                        }
                    } else {
                        let result = creep.pos().is_near_to(source.pos());
                        let range = creep.pos().get_range_to(source.pos());
                        info!("Moving to source, got {:?}, range {:?}", result, range);
                        Self::move_near(creep, source.pos(), memory);
                        true
                    }
                }
                None => false,
            };

            source
        } else {
            info!("Creep is full");
            false
        };

        if !keep_job {
            debug!("{} not keeping job", creep.name());
            memory.worker_type = CreepWorkerType::SimpleWorker(Task::Upgrade { controller: creep.room().unwrap().controller().unwrap().id(), worked_by: vec![] });
        }
    }
}
