use screeps::{Source, ConstructionSite, StructureController, ObjectId, StructureSpawn, Creep};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum Task {
    Harvest { node: ObjectId<Source>, worked_by: Vec<ObjectId<Creep>>, space_limit: usize },
    Build { site: ObjectId<ConstructionSite>, worked_by: Vec<ObjectId<Creep>> },
    Upgrade { controller: ObjectId<StructureController>, worked_by: Vec<ObjectId<Creep>> },
    Deposit { dest: ObjectId<StructureSpawn>, worked_by: Vec<ObjectId<Creep>> },
    Idle,
}

