use serde::{Serialize, Deserialize};

use crate::task::Task;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CreepWorkerType {
    SimpleWorker,
    Upgrader,
    Harvester,
}
