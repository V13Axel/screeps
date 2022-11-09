use screeps::{
    ObjectId, Source, StructureController,
};

// this enum will represent a creep's lock on a specific target object, storing a js reference to the object id so that we can grab a fresh reference to the object each successive tick, since screeps game objects become 'stale' and shouldn't be used beyond the tick they were fetched
#[derive(Clone, Debug)]
pub enum CreepGoal {
    Upgrade(ObjectId<StructureController>),
    Harvest(ObjectId<Source>),
}
