use screeps::{Source, ConstructionSite, StructureController, ObjectId, StructureSpawn};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Task {
    Harvest { node: ObjectId<Source> },
    Build { site: ObjectId<ConstructionSite> },
    Upgrade { controller: ObjectId<StructureController> },
    Deposit { dest: ObjectId<StructureSpawn> },
}
