use screeps::{
    StructureSpawn, Source, ObjectId, StructureController,
};
use serde::{Serialize, Deserialize};

use crate::task::Task;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CreepWorkerType {
    SimpleWorker(Task),
    // Upgrader(Task),
    // Harvester(Task),
}
