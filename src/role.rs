use log::{debug, info};
use screeps::{Creep, ObjectId, StructureController, ResourceType, ReturnCode, Source, HasPosition, RoomPosition, find, Position, SharedCreepProperties};
use wasm_bindgen::JsValue;

use crate::{util::path::{CreepPath, MovementDistance}, mem::CreepMemory};

pub struct CreepAction;


impl CreepAction {
    pub fn idle(creep: &Creep, memory: &mut CreepMemory) {
        Self::move_near(
            creep, 
            creep.room().unwrap().find(find::MY_SPAWNS)[0].pos(), 
            memory
        );
    }

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
        info!("Running movement for {:?}", creep.name());
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

    pub fn upgrade(creep: &Creep, controller_id: &ObjectId<StructureController>, memory: &mut CreepMemory) {
        if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
            match controller_id.resolve() {
                Some(controller) => {
                    let r = creep.upgrade_controller(&controller);
                    if r == ReturnCode::NotInRange {
                        Self::move_near(creep, controller.pos(), memory);
                    } else if r != ReturnCode::Ok {
                    } else {
                    }
                }
                None => {
                },
            }
        };

        // if !keep_job {
        //     let node = creep.room().unwrap().find(find::SOURCES)[0].id();
        //     memory.current_path = None;
        //     memory.current_task = Task::Harvest { node , worked_by: vec![], space_limit: 0 };
        //     memory.worker_type = MinionType::SimpleWorker;
        //
        //     Self::harvest(creep, &node, memory)
        // }
    }

    pub fn harvest(creep: &Creep, source_id: &ObjectId<Source>, memory: &mut CreepMemory) { 
        info!("Harvesting");
        if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            info!("Have more storage");
            let source = match source_id.resolve() {
                Some(source) => {
                    info!("Resolved source");
                    if creep.pos().is_near_to(source.pos()) {
                        info!("Nearby");
                        let r = creep.harvest(&source);
                        if r != ReturnCode::Ok {
                            false
                        } else {
                            true
                        }
                    } else {
                        info!("too far");
                        let result = creep.pos().is_near_to(source.pos());
                        let range = creep.pos().get_range_to(source.pos());
                        Self::move_near(creep, source.pos(), memory);
                        true
                    }
                }
                None => false,
            };

            source
        } else {
            info!("whut");
            false
        };
    }

    // pub fn build(creep: &Creep, site: &ConstructionSite, memory: &mut CreepMemory) {
    //     if creep.pos().is_near_to(site.pos()) {
    //         memory.current_task = match creep.build(&site) {
    //             ReturnCode::Ok => Task::Idle,
    //             code => {
    //                 info!("{:?} - Building code {:?}", creep.name(), code);
    //             
    //                 memory.current_task.to_owned()
    //             }
    //         }
    //     } else {
    //         Self::move_near(creep, site.pos(), memory);
    //     }
    // }
    //
    // pub fn deposit(creep: &Creep, dest: &impl Transferable, memory: &mut CreepMemory) {
    //     if creep.pos().is_near_to(dest.pos()) {
    //         memory.current_path = None;
    //
    //         match creep.transfer(dest, ResourceType::Energy, None) {
    //             ReturnCode::Ok => {
    //                 memory.current_task = Task::Idle;
    //             },
    //             code => {
    //                 info!("{:?}", code);
    //             }
    //         }
    //     } else {
    //         Self::move_near(creep, dest.pos(), memory)
    //     }
    // }
}
