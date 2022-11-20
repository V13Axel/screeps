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

pub fn run_managers(memory: &mut GameMemory) {
    let rooms: Vec<Room> = game::rooms()
        .values()
        .collect();

    TaskManager::with_rooms(&rooms).scan(memory);
    TaskManager::assign(memory);

    SpawnManager::with_rooms(&rooms).spawn(memory);
}

pub fn run_creep(creep: &Creep, memory: &mut CreepMemory) {
    debug!("Running {:?}", creep.name());
    if memory.current_path.is_some() {
        let path = memory.current_path.to_owned().unwrap();
        memory.current_path = match creep.move_by_path(&JsValue::from_str(&path.value)) {
            screeps::ReturnCode::Ok => Some(path),
            _ => None
        }
    }

    let worker_type = memory.worker_type.to_owned();

    match worker_type {
        minion::CreepWorkerType::SimpleWorker(task) => {
            match task {
                Task::Harvest { node, .. } => CreepPurpose::harvest(creep, &node, memory),
                Task::Upgrade { controller, .. } => CreepPurpose::upgrade(creep, &controller, memory),
                Task::Deposit { dest, .. } => CreepPurpose::deposit(creep, &dest.resolve().unwrap(), memory),
                Task::Build { site, .. } => CreepPurpose::build(creep, &site.resolve().unwrap(), memory),
                _ => {
                    // Basically ... If it's not one of the above, we'll just skip it
                    CreepPurpose::idle(creep, memory)
                }
            }     
        }
    }
}

pub fn run_creeps(memories: &mut HashMap<String, CreepMemory>) {
    for creep in game::creeps().values() {
        let name = creep.name();
        let memory = memories.entry(name).or_default();
        run_creep(&creep, memory);
    }
}

pub fn game_loop(memory: &mut GameMemory) {
    if memory.ticks_since_managers >= 50 {
        run_managers(memory);

        memory.ticks_since_managers = 0;
    } else {
        memory.ticks_since_managers += 1;
    }

    if memory.ticks_since_managers == 49 {
        clear_console();
    }

    run_creeps(&mut memory.creep_memories);
}

// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn memory_loop() {
    // Get our local heap memory and do the actual game logic
    GAME_MEMORY.with(|game_memory_refcell| {
        let mut game_memory = game_memory_refcell.borrow_mut().to_owned();
        game_loop(&mut game_memory);

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
    logging::setup_logging(logging::Info);
    panic::set_hook(Box::new(panic_handler));
    retrieve_memory();
}

pub fn retrieve_memory() {
    GAME_MEMORY.with(|game_memory_refcell| {
        if game_memory_refcell.borrow().needs_deserialized {
            info!("Global reset detected, memory deserialized");

            let memory_string = match RawMemory::get().as_string() {
                Some(memory) => {
                    debug!("Retrieved memory as: {}", memory);

                    memory
                },
                None => String::from("{}"), 
            };

            let mut deserialized: GameMemory = match serde_json::from_str(&memory_string) {
                Ok(deserialized) => {
                    debug!("Successfully deserialized memory to: {:?}", deserialized);

                    deserialized
                },
                Err(_) => GameMemory::default(),
            };

            deserialized.needs_deserialized = false;

            debug!("Replacing refcell");
            game_memory_refcell.replace(deserialized);
            debug!("Replaced refcell");
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

    debug!("{:?}", &stringified);

    match stringified {
        Ok(stringified) => RawMemory::set(&JsString::from(stringified)),
        Err(error) => info!("Could not serialize memory contents! Error: {:?}", error),
    }
}

// What a crazy hack.
fn clear_console() {
    console::log_1(&JsString::from("<script>angular.element(document.getElementsByClassName('fa fa-trash ng-scope')[0].parentNode).scope().Console.clear()</script>"));
}
