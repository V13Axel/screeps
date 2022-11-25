use std::{cell::RefCell, panic, borrow::BorrowMut};

use js_sys::JsString;
use log::*;

use manager::Managers;
use screeps::{RawMemory, game};

use wasm_bindgen::prelude::*;

use crate::{mem::GameMemory, minion::clean_up_dead_creeps};

mod logging;
mod role;
mod mem;
mod util;
mod minion;
mod manager;
mod task;

thread_local! {
    static GAME_MEMORY: RefCell<GameMemory<'static>> = RefCell::new(
        GameMemory::default()
    );
}

/**
*
* This is where the actual magic happens
*
*/
pub fn game_loop(memory: &mut GameMemory) {
    clean_up_dead_creeps(memory);

    Managers::run(memory);

    minion::run_creeps(&mut memory.creeps);
}

/**
* 
*   Setup/memory management stuff
*   -----------------------------
*   What this does is deserialize
*   RawMemory when & if necessary
*   otherwise just gives the loop
*   a mutable borrowed GameMemory
*
*/

// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn memory_loop() {
    // Get our local heap memory and do the actual game logic
    GAME_MEMORY.with(|game_memory_refcell| {
        let mut game_memory: GameMemory = game_memory_refcell.borrow_mut().to_owned();

        game_loop(&mut game_memory);

        // Persist to memory refcell after game logic executes
        game_memory_refcell.replace(game_memory);
    });

    // Serialize and save to memory. This is done separately to avoid weirdness.
    GAME_MEMORY.with(|game_memory_refcell| {
        save_memory(game_memory_refcell.into_inner());
    });

    debug!("Game loop finished: {}", game::time());
}

// add wasm_bindgen to any function you would like to expose for call from js
#[wasm_bindgen]
pub fn setup() {
    logging::setup_logging(logging::Info);
    panic::set_hook(Box::new(panic_handler));
    retrieve_memory();
}

fn panic_handler(info: &panic::PanicInfo) {
    error!("{:?}", info.to_string());
}

// Basically just deserializes memory if necessary
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
                Err(error) => {
                    info!("Error deserializing memory:\n{:?}\n\n ... Falling back to default.", error);

                    GameMemory::default()
                },
            };

            deserialized.needs_deserialized = false;

            debug!("Replacing refcell");
            game_memory_refcell.replace(deserialized);
            debug!("Replaced refcell");
        }
    })
}

// There aren't any real situations I can think of where this would _need_ to happen...
// But if it does, I want to have it handy.
pub fn reset_memory() {
    GAME_MEMORY.with(|game_memory_refcell| {
        game_memory_refcell.replace(GameMemory::default());
    })
}

// Serializes GameMemory and saves it to RawMemory
fn save_memory(game_memory: GameMemory) {
    let stringified = serde_json::to_string(&game_memory);

    debug!("{:?}", &stringified);

    match stringified {
        Ok(stringified) => RawMemory::set(&JsString::from(stringified)),
        Err(error) => info!("Could not serialize memory contents! Error: {:?}", error),
    }
}

