use log::info;
use screeps::{Source, ObjectId, ReturnCode, ResourceType, find, HasPosition, SharedCreepProperties};

use crate::{action::CreepAction, mem::CreepMemory};

use super::TaskProps;

#[derive(Debug, Clone)]
pub struct Harvest {
    pub props: TaskProps,
    pub source_id: ObjectId<Source>,
    pub spaces_available: usize,
}

impl Harvest {
    fn run(&mut self, creep: &screeps::Creep, memory: &mut CreepMemory) {
        info!("Harvesting");
        if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            info!("Have more storage");
            let source = match self.source_id.resolve() {
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
                        CreepAction::move_near(creep, source.pos().into(), memory);
                        true
                    }
                }
                None => false,
            };

            source
        } else {
            if let Some(spawn) = creep.room().unwrap().find(find::MY_SPAWNS).pop() {
                if creep.pos().is_near_to(spawn.pos()) {
                    match creep.transfer(&spawn, ResourceType::Energy, None) {
                        ReturnCode::Ok => {
                            info!("Transferred to spawn");
                        },
                        r => {
                            info!("Tried {:?}", r);
                        }
                    }
                } else {
                    CreepAction::move_near(creep, spawn.pos(), memory);
                }
            }
            false
        };
    }
}
