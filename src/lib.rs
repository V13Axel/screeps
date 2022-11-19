use std::{collections::HashMap, cell::RefCell, panic};

use js_sys::JsString;
use log::*;
use manager::{TaskManager, SpawnManager};
use mem::CreepMemory;
use role::CreepPurpose;
use screeps::{RawMemory, game, Room, Creep, SharedCreepProperties};

use task::Task;
use wasm_bindgen::prelude::*;

use crate::mem::GameMemory;

mod logging;
mod role;
mod mem;
mod util;
mod minion;
mod manager;
mod task;

thread_local! {
    static GAME_MEMORY: RefCell<GameMemory> = RefCell::new(
        GameMemory::default()
    );
}

pub fn run_managers(mut memory: GameMemory) -> GameMemory {
    let rooms: Vec<Room> = game::rooms()
        .values()
        .collect();

    memory = TaskManager::with_rooms(&rooms).scan(memory);
    memory = TaskManager::assign(memory);

    memory = SpawnManager::with_rooms(&rooms).spawn(memory);

    memory
}

pub fn run_creep(creep: &Creep, creep_memory: CreepMemory) -> CreepMemory {
    // info!("{:?}\n{:?}", creep.name(), creep_memory);
    let new_memory = match creep_memory.worker_type {
        minion::CreepWorkerType::SimpleWorker(ref task) => match task {
            Task::Idle => creep_memory,
            Task::Harvest { node, worked_by: _ } => CreepPurpose::harvest(creep, &node, creep_memory.to_owned()),
            _ => {
                todo!("Not yet implemented: {:?}", task);
            }
        }
    };

    new_memory
}

pub fn actual_game_loop(mut memory: GameMemory) -> GameMemory {
    if game::time() % 50 == 0 {
        memory = run_managers(memory);
    }

    for creep in game::creeps().values() {
        let creep_memory = memory.creep_memories.get(&creep.name()).unwrap_or_default().to_owned();
        run_creep(&creep, creep_memory);
    }

    memory
}

// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    // Get our local heap memory and do the actual game logic
    GAME_MEMORY.with(|game_memory_refcell| {
        let mut game_memory = game_memory_refcell.borrow_mut().to_owned();

        game_memory = actual_game_loop(game_memory);

        // Persist to memory refcell after game logic executes
        game_memory_refcell.replace(game_memory);
    });

    // Serialize and save to memory. This is done separately to avoid weirdness.
    GAME_MEMORY.with(|game_memory_refcell| {
        save_memory(game_memory_refcell.borrow().to_owned());
    });

    info!("Game loop finished: {}", game::time());
}

/**
* 
*   Setup/memory management stuff
*
*/

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

    // info!("{:?}", &stringified);

    match stringified {
        Ok(stringified) => RawMemory::set(&JsString::from(stringified)),
        Err(error) => info!("Could not serialize memory contents! Error: {:?}", error),
    }
}

