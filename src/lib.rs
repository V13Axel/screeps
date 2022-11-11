use std::cell::RefCell;
use std::collections::HashMap;

use log::*;
use screeps::{
    find, game, prelude::*, Creep, Part, ResourceType, ReturnCode, RoomObjectProperties, StructureObject, ConstructionSite,
};
use goal::CreepGoal;
use role::CreepRole;
use wasm_bindgen::prelude::*;

mod logging;
mod role;
mod goal;

// add wasm_bindgen to any function you would like to expose for call from js
#[wasm_bindgen]
pub fn setup() {
    logging::setup_logging(logging::Info);
}

// this is one way to persist data between ticks within Rust's memory, as opposed to
// keeping state in memory on game objects - but will be lost on global resets!
thread_local! {
    static CREEP_TARGETS: RefCell<HashMap<String, CreepGoal>> = RefCell::new(HashMap::new());
}


// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    debug!("loop starting! CPU: {}", game::cpu::get_used());
    // mutably borrow the creep_targets refcell, which is holding our creep target locks
    // in the wasm heap
    CREEP_TARGETS.with(|creep_targets_refcell| {
        let mut creep_targets = creep_targets_refcell.borrow_mut();
        debug!("running creeps");
        // same type conversion (and type assumption) as the spawn loop
        for creep in game::creeps().values() {
            run_creep(&creep, &mut creep_targets);
        }
    });

    debug!("running spawns");
    // Game::spawns returns a `js_sys::Object`, which is a light reference to an
    // object of any kind which is held on the javascript heap.
    //
    // Object::values returns a `js_sys::Array`, which contains the member spawn objects
    // representing all the spawns you control.
    //
    // They are returned as wasm_bindgen::JsValue references, which we can safely
    // assume are StructureSpawn objects as returned from js without checking first
    let mut additional = 0;
    for spawn in game::spawns().values() {
        debug!("running spawn {}", String::from(spawn.name()));

        let body = [Part::Move, Part::Move, Part::Carry, Part::Work];
        if spawn.room().unwrap().energy_available() >= body.iter().map(|p| p.cost()).sum() {
            // create a unique name, spawn.
            let name_base = game::time();
            let name = format!("{}-{}", name_base, additional);
            // note that this bot has a fatal flaw; spawning a creep
            // creates Memory.creeps[creep_name] which will build up forever;
            // these memory entries should be prevented (todo doc link on how) or cleaned up
            let res = spawn.spawn_creep(&body, &name);

            // todo once fixed in branch this should be ReturnCode::Ok instead of this i8 grumble grumble
            if res != ReturnCode::Ok {
                warn!("couldn't spawn: {:?}", res);
            } else {
                additional += 1;
            }
        }
    }


    info!("done! cpu: {}", game::cpu::get_used())
}

fn run_creep(creep: &Creep, creep_targets: &mut HashMap<String, CreepGoal>) {
    if creep.spawning() {
        return;
    }
    let name = creep.name();
    debug!("running creep {}", name);

    let target = creep_targets.remove(&name);
    match target {
        Some(creep_target) => {
            let keep_target = match &creep_target {
                CreepGoal::Upgrade(controller_id) => {
                    CreepRole::upgrade(creep, controller_id)
                }
                CreepGoal::Harvest(source_id) => {
                    CreepRole::harvest(creep, source_id)
                }
                CreepGoal::Construct(_site_id) => {
                    false
                }
            };

            if keep_target {
                creep_targets.insert(name, creep_target);
            }
        }
        None => {
            // no target, let's find one depending on if we have energy
            let room = creep.room().expect("couldn't resolve creep room");

            if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {

                for site in room.find(find::CONSTRUCTION_SITES).iter() {
                    info!("{:?}", site.try_id());
                    creep_targets.insert(name, CreepGoal::Construct(site.try_id().unwrap()));
                    return;
                }

                for structure in room.find(find::STRUCTURES).iter() {
                    if let StructureObject::StructureController(controller) = structure {
                        creep_targets.insert(name, CreepGoal::Upgrade(controller.id()));
                        break;
                    }
                }
            } else if let Some(source) = room.find(find::SOURCES_ACTIVE).get(0) {
                creep_targets.insert(name, CreepGoal::Harvest(source.id()));
            }
        }
    }
}
