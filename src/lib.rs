use std::{collections::HashMap, cell::RefCell, panic};

use js_sys::JsString;
use log::*;
use manager::{TaskManager, SpawnManager};
use mem::CreepMemory;
use role::CreepPurpose;
use screeps::{RawMemory, game, Room, Creep, SharedCreepProperties};

use task::Task;
use wasm_bindgen::prelude::*;
use web_sys::console;

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
    console::log_1(&JsString::from("<script>angular.element(document.getElementsByClassName('fa fa-trash ng-scope')[0].parentNode).scope().Console.clear()</script>"));
    let rooms: Vec<Room> = game::rooms()
        .values()
        .collect();

    memory = TaskManager::with_rooms(&rooms).scan(memory);
    memory = TaskManager::assign(memory);

    memory = SpawnManager::with_rooms(&rooms).spawn(memory);

    memory
}

pub fn run_creep(creep: &Creep, memory: CreepMemory) -> CreepMemory {
    match memory.worker_type {
        minion::CreepWorkerType::SimpleWorker(ref task) => {
            match task {
                Task::Idle => CreepPurpose::idle(creep, memory.to_owned()),
                Task::Harvest { node, .. } => CreepPurpose::harvest(creep, &node, memory.to_owned()),
                Task::Upgrade { controller, .. } => CreepPurpose::upgrade(creep, controller, memory.to_owned()),
                _ => {
                    todo!("Not yet implemented: {:?}", task);
                }
            }     
        }
    }
}

pub fn run_creeps(creep_memories: HashMap<String, CreepMemory>) -> HashMap<String, CreepMemory> {
    game::creeps().values().map(|creep| {
        (
            creep.name(),
            run_creep(&creep, creep_memories.get(&creep.name()).unwrap_or_default().to_owned())
        )
    }).collect()
}

pub fn game_loop(mut memory: GameMemory) -> GameMemory {
    if memory.ticks_since_managers >= 50 {
        memory = run_managers(memory);
        memory.ticks_since_managers = 0;
    } else {
        memory.ticks_since_managers += 1;
    }

    memory.creep_memories = run_creeps(memory.creep_memories);

    memory
}

// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn memory_loop() {
    // Get our local heap memory and do the actual game logic
    GAME_MEMORY.with(|game_memory_refcell| {
        let mut game_memory = game_memory_refcell.borrow_mut().to_owned();
        game_memory = game_loop(game_memory);

        // Persist to memory refcell after game logic executes
        game_memory_refcell.replace(game_memory);
    });

    // Serialize and save to memory. This is done separately to avoid weirdness.
    GAME_MEMORY.with(|game_memory_refcell| {
        let mut memory_to_save = game_memory_refcell.borrow_mut().to_owned();

        memory_to_save.ticks_since_managers = 9999;

        save_memory(memory_to_save);
    });

    debug!("Game loop finished: {}", game::time());
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

