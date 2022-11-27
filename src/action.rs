use log::{debug, info};
use screeps::{Creep, ObjectId, ResourceType, ReturnCode, Source, HasPosition, RoomPosition, Position, SharedCreepProperties};
use wasm_bindgen::JsValue;

use crate::{util::path::{CreepPath, MovementDistance}, mem::CreepMemory};

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
}
