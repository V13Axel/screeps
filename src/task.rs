use screeps::{ObjectWithId, Path, Creep, HasPosition};

use crate::{mem::CreepMemory, util::path::CreepPath};

mod upgrade;

pub enum TaskStyle {
    Perpetual,
    Once,
}

pub struct TaskProps {
    target: Option<dyn ObjectWithId + HasPosition>,
    style: TaskStyle,
    min_room_level: usize,
}

pub trait Task {
    fn get_target(&self) -> Option<ObjectWithId>;
    fn get_path_to(&self, creep: &Creep, memory: &mut CreepMemory) -> Path;
    fn is_finished(&self) -> bool;
}

pub struct Upgrade {
    props: TaskProps,
}

impl Task for Upgrade {
    fn get_target(&self) -> Option<ObjectWithId> {
        self.props.target
    }

    fn get_path_to(&self, creep: &Creep, memory: &mut CreepMemory) -> Path {
        match self.get_target() {
            Some(target) => CreepPath::determine(
                creep.room()
                    .unwrap(),
                creep.pos(), 
                target.pos(), 
            ),
            None => Path::from(vec![])
        }
    }

    fn is_finished(&self) -> bool {
        false
    }
}
