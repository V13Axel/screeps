use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CreepWorkerType {
    SimpleWorker,
    Upgrader,
    Harvester,
}
