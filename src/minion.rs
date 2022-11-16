use screeps::{
    StructureSpawn, Source, ObjectId, StructureController,
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CreepWorkerType {
    SimpleWorker(SimpleJob),
    Upgrader(SimpleJob),
    Harvester(SimpleJob),
}


#[derive(Clone, Serialize, Deserialize, Debug, Copy)]
pub enum SimpleJob {
    ApproachSource(ObjectId<Source>),
    HarvestSource(ObjectId<Source>),
    ApproachController(ObjectId<StructureController>),
    UpgradeController(ObjectId<StructureController>),
    ApproachSpawn(ObjectId<StructureSpawn>),
    TransferToSpawn(ObjectId<StructureSpawn>),
}
