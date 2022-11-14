use log::warn;
use screeps::{Creep, ObjectId, StructureController, ResourceType, ReturnCode, SharedCreepProperties, Source};

// enum Role {
//     Harvester,
//     Builder,
//     Upgrader,
// }
//

pub struct RolePriority {

}

impl RolePriority {
    pub fn of(role: CreepRole) -> usize{
        match role {
            CreepRole::Harvester => 1,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum CreepRole {
    Harvester,
}

pub struct CreepPurpose {
    // name: String,
    // definition: Vec<String>
}

impl CreepPurpose {
    pub fn upgrade(creep: &Creep, controller_id: &ObjectId<StructureController>) -> bool {
        if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
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
        }
    }

    pub fn harvest(creep: &Creep, source_id: &ObjectId<Source>) -> bool {
        if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            match source_id.resolve() {
                Some(source) => {
                    if creep.pos().is_near_to(&source.pos()) {
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
        }
    }
}