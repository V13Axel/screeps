use log::{warn, info};
use screeps::{Creep, ObjectId, StructureController, ResourceType, ReturnCode, SharedCreepProperties, Source, HasPosition, Position, RoomPosition, Find, find, game, HasTypedId};
use serde_wasm_bindgen::to_value;

use crate::{util::path::CreepPath, mem::CreepMemory, minion::CreepWorkerType, task::Task};


pub struct CreepPurpose {
    // name: String,
    // definition: Vec<String>
}

impl CreepPurpose {
    pub fn idle(creep: &Creep, mut memory: CreepMemory) -> CreepMemory {
        Self::move_near(creep, RoomPosition::from(creep.room().unwrap().find(find::MY_SPAWNS)[0].pos()), &mut memory);
        // creep.move_to(&creep.room().unwrap().find(find::MY_SPAWNS)[0]);

        memory
    }
    // Gets you to the position
    pub fn move_to<'a>(creep: &Creep, position: RoomPosition, mut memory: &'a mut CreepMemory) -> &'a CreepMemory {
        memory.current_path = if creep.pos().is_near_to(position.to_owned().into()) {
            None
        } else {
            Self::do_movement(creep, &position, &memory.current_path)
        };

        info!("{:?}", memory);

        memory
    }
    
    // Gets you next to the position
    pub fn move_near<'a>(creep: &Creep, position: RoomPosition, mut memory: &'a mut CreepMemory) -> &'a CreepMemory {
        memory.current_path = if creep.pos().is_near_to(position.to_owned().into()) {
            None
        } else {
            Self::do_movement(creep, &position, &memory.current_path)
        };

        memory
    }

    fn do_movement(creep: &Creep, position: &RoomPosition, current_path: &Option<CreepPath>) -> Option<CreepPath> {
        let room = creep.room().unwrap();
        let path = room.find_path(
            &RoomPosition::from(creep.pos()), 
            &RoomPosition::from(room.find(find::MY_SPAWNS)[0].pos()),
            None
        );

        info!("{:?}", path);

        creep.move_by_path(&path); // -> InvalidArgs

        None
        // let path = match current_path {
        //     Some(path) => path.to_owned(),
        //     None => {
        //         info!("{:?}", creep.room()
        //             .unwrap()
        //             .find_path(
        //                 &RoomPosition::from(creep.pos()), 
        //                 &position, 
        //                 None
        //             )
        //         );
        //         CreepPath::from(creep.room()
        //             .unwrap()
        //             .find_path(
        //                 &RoomPosition::from(creep.pos()), 
        //                 &position, 
        //                 None
        //             )
        //         )
        //     }
        // };
        //
        // let path_serialized = to_value(&path).unwrap();
        //
        //
        // let result = creep.move_by_path(&path_serialized);
        //
        // match result {
        //     ReturnCode::Ok => { 
        //         Some(path) 
        //     },
        //     _ => {
        //         info!("-------\nMovement return code - {:?}\nPath - {:?}\nPath ser - {:?}",  result, path, path_serialized);
        //         None
        //     }
        // }
    }

    pub fn upgrade(creep: &Creep, controller_id: &ObjectId<StructureController>, mut memory: CreepMemory) -> CreepMemory {
        let keep_job = if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
            match controller_id.resolve() {
                Some(controller) => {
                    let r = creep.upgrade_controller(&controller);
                    if r == ReturnCode::NotInRange {
                        // creep.move_to(&controller);
                        Self::move_to(creep, controller.pos().into(), &mut memory);
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

        memory
    }

    pub fn harvest(creep: &Creep, source_id: &ObjectId<Source>, mut memory: CreepMemory) -> CreepMemory {
        let keep_job = if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            let source = match source_id.resolve() {
                Some(source) => {
                    if creep.pos().is_near_to(source.pos()) {
                        let r = creep.harvest(&source);
                        if r != ReturnCode::Ok {
                            info!("couldn't harvest: {:?}", r);
                            false
                        } else {
                            true
                        }
                    } else {
                        // creep.move_to(&source);
                        Self::move_to(creep, source.pos().into(), &mut memory);
                        true
                    }
                }
                None => false,
            };

            source
        } else {
            false
        };

        if !keep_job {
            info!("{} not keeping job", creep.name());
            memory.worker_type = CreepWorkerType::SimpleWorker(Task::Upgrade { controller: creep.room().unwrap().controller().unwrap().id(), worked_by: vec![] });
        }

        memory
    }
}
