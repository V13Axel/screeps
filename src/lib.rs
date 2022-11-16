use std::{collections::HashMap, cell::RefCell, panic};

use js_sys::JsString;
use log::*;
use screeps::{
    StructureSpawn, Source, ObjectId, RawMemory, StructureController,
};

use serde::{Serialize, Deserialize};

use wasm_bindgen::prelude::*;

use crate::mem::GameMemory;

mod logging;
mod role;
mod mem;
mod util;

thread_local! {
    static GAME_MEMORY: RefCell<GameMemory> = RefCell::new(
        GameMemory::default()
    );
}

#[derive(Clone, Serialize, Deserialize, Debug)]
enum CreepWorkerType {
    SimpleWorker(SimpleJob),
    Upgrader(SimpleJob),
    Harvester(SimpleJob),
}


#[derive(Clone, Serialize, Deserialize, Debug, Copy)]
enum SimpleJob {
    ApproachSource(ObjectId<Source>),
    HarvestSource(ObjectId<Source>),
    ApproachController(ObjectId<StructureController>),
    UpgradeController(ObjectId<StructureController>),
    ApproachSpawn(ObjectId<StructureSpawn>),
    TransferToSpawn(ObjectId<StructureSpawn>),
}


fn panic_handler(info: &panic::PanicInfo) {
    error!("{:?}", info.to_string());
}


// add wasm_bindgen to any function you would like to expose for call from js
#[wasm_bindgen]
pub fn setup() {
    logging::setup_logging(logging::Debug);
    panic::set_hook(Box::new(panic_handler));
    retrieve_memory();
}

pub fn retrieve_memory() {
    GAME_MEMORY.with(|game_memory_refcell| {
        if game_memory_refcell.borrow().needs_deserialized {
            let memory_string = match RawMemory::get().as_string() {
                Some(memory) => {
                    info!("Retrieved memory as: {}", memory);

                    memory
                },
                None => String::from("{}"), 
            };

            let mut deserialized: GameMemory = match serde_json::from_str(&memory_string) {
                Ok(deserialized) => {
                    info!("Successfully deserialized memory to: {:?}", deserialized);

                    deserialized
                },
                Err(_) => GameMemory::default(),
            };

            deserialized.needs_deserialized = false;

            info!("Replacing refcell");
            game_memory_refcell.replace(deserialized);
            info!("Replaced refcell");
        }
    })
}

pub fn reset_memory() {
    GAME_MEMORY.with(|game_memory_refcell| {
        game_memory_refcell.replace(GameMemory::default());
    })
}

fn save_memory(game_memory: GameMemory) {
    let stringified = serde_json::to_string(&game_memory);

    info!("{:?}", &stringified);

    match stringified {
        Ok(stringified) => RawMemory::set(&JsString::from(stringified)),
        Err(error) => info!("Could not serialize memory contents! Error: {:?}", error),
    }
}

// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    let mut new_structure_memories = HashMap::new();
    let mut new_creep_memories = HashMap::new();

    GAME_MEMORY.with(|game_memory_refcell| {
        let GameMemory { 
            creep_memories,
            structure_memories,
            room_memories: _,
            needs_deserialized: _ 
        } = game_memory_refcell.borrow_mut().to_owned();
    });

    info!("Whoo");

    // Serialize and save to memory.
    save_memory(GameMemory {
        creep_memories: new_creep_memories,
        structure_memories: new_structure_memories,
        room_memories: HashMap::new(),
        needs_deserialized: false,
    });
}
