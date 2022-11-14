use std::collections::HashMap;

use log::*;
use screeps::{
    find, game, prelude::*, Creep, Part, ResourceType, ReturnCode, RoomObjectProperties, StructureObject, ConstructionSite, JsHashMap, RawObjectId, StructureType, StructureSpawn, Source, ObjectId, RawMemory, StructureController,
};

use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::{from_value, to_value};

use goal::CreepGoal;
use role::{CreepRole, CreepPurpose};
use wasm_bindgen::prelude::*;

mod logging;
mod role;
mod goal;


#[derive(Clone, Serialize, Deserialize, Debug)]
struct RoomMemory {
    controller_level: usize,
}

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
    HarvestSource(ObjectId<Source>),
    MoveToController(ObjectId<StructureController>),
    UpgradeController(ObjectId<StructureController>),
    // MoveToSpawn(ObjectId<StructureSpawn>),
    // TransferToSpawn(ObjectId<StructureSpawn>),
}

// add wasm_bindgen to any function you would like to expose for call from js
#[wasm_bindgen]
pub fn setup() {
    logging::setup_logging(logging::Debug);
}

// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    let structures = game::structures();
    let creeps = game::creeps();

    run_structures(&structures);
    run_creeps(&creeps);
}

pub fn run_structures(structures: &JsHashMap<RawObjectId, StructureObject>) {
    structures.values().for_each(|structure| {
        run_structure(structure);
    })
}

pub fn run_structure(structure: StructureObject) {
    match structure.structure_type() {
        StructureType::Spawn => run_spawn(structure.try_into().unwrap()),
        StructureType::Controller => run_controller(structure.try_into().unwrap()),
        st => warn!("Not yet implemented type: {:?}", st),
    }
}

pub fn run_controller(controller: StructureController) {
    let room = controller.room().to_owned().unwrap();
    // let mut room_memory: RoomMemory = from_value(&room.memory()).unwrap();
    let mut room_memory: RoomMemory = from_value::<RoomMemory>(
        room.memory()
    ).unwrap_or(RoomMemory{
        controller_level: 1,
    });

    if room_memory.controller_level < controller.level().into() {
        room_memory.controller_level = controller.level().into();
        info!("Controller upgraded!");
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
