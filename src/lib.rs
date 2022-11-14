use std::{collections::HashMap, cell::RefCell, panic};

use js_sys::JsString;
use log::*;
use screeps::{
    find, game, prelude::*, Creep, Part, ResourceType, ReturnCode, RoomObjectProperties, StructureObject, ConstructionSite, JsHashMap, RawObjectId, StructureType, StructureSpawn, Source, ObjectId, RawMemory, StructureController, Find, Room, Structure,
};

use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::{from_value, to_value};

use goal::CreepGoal;
use role::{CreepRole, CreepPurpose};
use wasm_bindgen::prelude::*;

mod logging;
mod role;
mod goal;

thread_local! {
    static GAME_MEMORY: RefCell<GameMemory> = RefCell::new(
        GameMemory::default()
    );
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct GameMemory {
    needs_deserialized: bool,
    creep_memories: HashMap<ObjectId<Creep>, CreepMemory>,
    room_memories: HashMap<String, RoomMemory>,
    structure_memories: HashMap<ObjectId<Structure>, StructureMemory>,
}

impl GameMemory {
    pub fn default() -> Self {
        GameMemory { 
            needs_deserialized: true,
            creep_memories: HashMap::new(),
            room_memories: HashMap::new(),
            structure_memories: HashMap::new() 
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct RoomMemory {
    controller_level: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
enum StructureMemory {
    Spawner(i32),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct CreepMemory {
    worker_type: CreepWorkerType,
}

impl CreepMemory {
    pub fn default(room: Room) -> CreepMemory {
        CreepMemory {
            worker_type: CreepWorkerType::SimpleWorker(
                SimpleJob::MoveToSpawn(
                    room.find(find::MY_SPAWNS)[0].id()
                )
            )
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
enum CreepWorkerType {
    SimpleWorker(SimpleJob),
}


#[derive(Clone, Serialize, Deserialize, Debug)]
enum SimpleJob {
    ApproachSource(ObjectId<Source>),
    HarvestSource(ObjectId<Source>),
    MoveToController(ObjectId<StructureController>),
    UpgradeController(ObjectId<StructureController>),
    MoveToSpawn(ObjectId<StructureSpawn>),
    // TransferToSpawn(ObjectId<StructureSpawn>),
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

pub fn save_memory() {
    GAME_MEMORY.with(|game_memory_refcell| {
        let game_memory = game_memory_refcell.borrow_mut();
        let stringified = serde_json::to_string(&game_memory.to_owned());

        match stringified {
            Ok(stringified) => RawMemory::set(&JsString::from(stringified)),
            Err(error) => info!("Could not serialize memory contents! Error: {:?}", error),
        }
    })
}

// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    GAME_MEMORY.with(|game_memory_refcell| {
        let game_memory = game_memory_refcell.borrow_mut();

        info!("Game memory: \n{:?}", game_memory);
        let structures = game::structures();
        let creeps = game::creeps();

        run_structures(&structures);
        run_creeps(&creeps);
    });

    // Serialize and save to memory.
    save_memory();
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

pub fn get_room_of<T>(object: &dyn RoomObjectProperties) -> Room {
    object.room().unwrap()
}

pub fn run_creep(creep: Creep) {
    info!("{:?}", creep.name());
    let creep_room_spawn: &StructureSpawn = &creep.room().to_owned().unwrap().find(find::MY_SPAWNS)[0];
    let memory = from_value(creep.memory()).unwrap_or(
        CreepMemory::default(
            get_room_of::<Creep>(&creep)
        )
    );

    // Break out memory values
    let CreepMemory { worker_type } = memory;

    let job = match worker_type {
        CreepWorkerType::SimpleWorker(job) => job
    };

    let keep_job = match job {
        SimpleJob::ApproachSource(target) => { info!("{:?}", target); true }, // CreepPurpose::move_to(&creep, &target),
        SimpleJob::HarvestSource(target) => { info!("{:?}", target); true },
        SimpleJob::MoveToController(target) => { info!("{:?}", target); true },
        SimpleJob::UpgradeController(target) => { info!("{:?}", target); true },
        SimpleJob::MoveToSpawn(target)=> { info!("{:?}", target); true },
        // TransferToSpawn(ObjectId<StructureSpawn>),
    };
}
