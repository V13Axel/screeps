use log::info;
use screeps::{Source, RawObjectId, ObjectId, ReturnCode, ResourceType, Creep, MaybeHasTypedId};

use crate::action::CreepAction;

use super::{Task, TaskProps};

#[derive(Debug, Clone)]
pub struct Harvest {
    pub props: TaskProps,
    pub source_id: ObjectId<Source>,
    pub spaces_available: usize,
}

impl Task for Harvest {
    fn run(&mut self, creep: &screeps::Creep, memory: &mut crate::mem::CreepMemory) {
        info!("Harvesting");
        if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            info!("Have more storage");
            let source = match self.source_id.resolve() {
                Some(source) => {
                    info!("Resolved source");
                    if creep.pos().is_near_to(&source.pos()) {
                        info!("Nearby");
                        let r = creep.harvest(&source);
                        if r != ReturnCode::Ok {
                            false
                        } else {
                            true
                        }
                    } else {
                        info!("too far");
                        let result = creep.pos().is_near_to(&source.pos());
                        let range = creep.pos().get_range_to(&source.pos());
                        CreepAction::move_near(creep, source.pos().into(), memory);
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

    fn assign_creep(&mut self, creep: &Creep) {
        self.props.worked_by.push(creep.try_id().unwrap())
    }

    fn get_props(&self) -> super::TaskProps {
        self.props.to_owned()
    }

    fn set_props(&mut self, props: super::TaskProps) {
        self.props = props;
    }

    fn get_target(&self) -> Option<screeps::RawObjectId> {
        Some(self.source_id.into())
    }

    fn set_target(&mut self, target: screeps::RawObjectId) {
        self.source_id = target.into();
    }

    fn needed_type(&self) -> crate::minion::MinionType {
        crate::minion::MinionType::Harvester
    }

    fn needs_creeps(&mut self) -> bool {
        self.props.clean_up_workers();
        self.props.worked_by.len() < self.spaces_available
    }

    fn is_finished(&self) -> bool {
        false
    }
}
