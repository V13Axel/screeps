use log::*;
use screeps::{
    find, game, prelude::*, Creep, Part, ResourceType, ReturnCode, RoomObjectProperties, StructureObject, ConstructionSite, JsHashMap, RawObjectId, StructureType, StructureSpawn, Source, ObjectId,
};

use serde::{Serialize, Deserialize};

use goal::CreepGoal;
use role::CreepRole;
use wasm_bindgen::prelude::*;

mod logging;
mod role;
mod goal;

#[derive(Clone, Serialize, Deserialize)]
enum StructureMemory {
    Spawner(i32),
}

#[derive(Clone, Serialize, Deserialize)]
enum CreepMemory {
    SimpleWorker(SimpleJob),
}

#[derive(Clone, Serialize, Deserialize)]
enum SimpleJob {
    ApproachSource(ObjectId<Source>),
}

// add wasm_bindgen to any function you would like to expose for call from js
#[wasm_bindgen]
pub fn setup() {
    logging::setup_logging(logging::Info);
}

// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    let structures = game::structures();
    let creeps = game::creeps();

    run_structures(&structures);
    run_creeps(&creeps);
    debug!("running spawns");
}

pub fn run_structures(structures: &JsHashMap<RawObjectId, StructureObject>) {
    structures.values().for_each(|structure| {
        run_structure(structure);
    })
}

pub fn run_structure(structure: StructureObject) {
    match structure.structure_type() {
        StructureType::Spawn => run_spawn(structure.try_into().unwrap()),
        st => warn!("Not yet implemented type: {:?}", st),
    }
}

pub fn run_spawn(spawn: StructureSpawn) {
    let creeps = game::creeps();
    if creeps.values().count() < 5 {
        let creep_name = format!("{}-{}", String::from("Creep"), game::time());
        spawn.spawn_creep(&[
            Part::Carry,
            Part::Move,
            Part::Work,
        ], &creep_name);
    }
}

pub fn run_creeps(creeps: &JsHashMap<String, Creep>) {
    creeps.values().for_each(|creep| {
        run_creep(creep);
    })
}

pub fn run_creep(creep: Creep) {
    info!("{:?}", creep.name());
}
