use log::{warn, info};
use screeps::{Creep, ObjectId, StructureController, ResourceType, ReturnCode, SharedCreepProperties, Source, HasPosition, Position, RoomPosition};
use serde_wasm_bindgen::to_value;

use crate::{util::path::CreepPath, mem::CreepMemory};


pub struct CreepPurpose {
    name: String,
    definition: Vec<String>
}

impl CreepPurpose {
    // Gets you to the position
    pub fn move_to(creep: &Creep, position: RoomPosition, mut memory: CreepMemory) -> CreepMemory {
        creep.room().unwrap().find_path(
            &RoomPosition::from(creep.pos()), 
            &position, 
            None
        ).iter().map(|value| {
                info!("{:?}", value);

                value
            });
        // let path = match memory.current_path {
        //     Some(path) => path,
        //     None => {
        //         CreepPath::from(
        //             creep.room().unwrap().find_path(
        //                 &RoomPosition::from(creep.pos()), 
        //                 &position, 
        //                 None
        //             ).iter().map(|value| {
        //                     info!("{:?}", value);
        //
        //                     value
        //                 })
        //         )
        //     }
        // };

        // match creep.move_by_path(path) {
        //     return_code => info!("{:?}", return_code)
        // };
        //
        // if creep.pos().is_equal_to(position.into()) {
        //     memory.current_path = None;
        // }

        memory
    }
    
    // Gets you next to the position
    pub fn move_near(creep: &Creep, position: RoomPosition, mut memory: CreepMemory) -> CreepMemory {
        // memory.current_path = if creep.pos().is_near_to(position.into()) {
        //     None
        // } else {
        //     match memory.current_path {
        //         Some(path) => Some(path),
        //         None => {
        //                 Some(
        //                     CreepPath::from(
        //                         creep.room()
        //                             .unwrap()
        //                             .find_path(
        //                                 &RoomPosition::from(creep.pos()), 
        //                                 &position, 
        //                                 None
        //                             )
        //                     )
        //                 )
        //         }
        //     }
        // };
        //
        memory
    }

    pub fn upgrade(creep: &Creep, controller_id: &ObjectId<StructureController>, mut memory: CreepMemory) -> CreepMemory {
        let keep_job = if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
            match controller_id.resolve() {
                Some(controller) => {
                    let r = creep.upgrade_controller(&controller);
                    if r == ReturnCode::NotInRange {
                        creep.move_to(&controller);
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

        memory
    }

    pub fn harvest(creep: &Creep, source_id: &ObjectId<Source>, memory: CreepMemory) -> CreepMemory {
        let keep_job = if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            match source_id.resolve() {
                Some(source) => {
                    if creep.pos().is_near_to(source.pos()) {
                        let r = creep.harvest(&source);
                        if r != ReturnCode::Ok {
                            warn!("couldn't harvest: {:?}", r);
                            false
                        } else {
                            true
                        }
                    } else {
                        creep.move_to(&source);
                        true
                    }
                }
                None => false,
            }
        } else {
            false
        };

        memory
    }
}
