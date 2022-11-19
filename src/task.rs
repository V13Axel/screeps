use screeps::{Source, ConstructionSite, StructureController, ObjectId, StructureSpawn, Creep, Part};
use serde::{Serialize, Deserialize};

use crate::mem::CreepMemory;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Task {
    Harvest { node: ObjectId<Source>, worked_by: Vec<ObjectId<Creep>> },
    Build { site: ObjectId<ConstructionSite>, worked_by: Vec<ObjectId<Creep>> },
    Upgrade { controller: ObjectId<StructureController>, worked_by: Vec<ObjectId<Creep>> },
    Deposit { dest: ObjectId<StructureSpawn>, worked_by: Vec<ObjectId<Creep>> },
    SpawnCreep { minimum_body: Vec<Part>, memory: Box<Option<CreepMemory>> },
    Idle,
}
