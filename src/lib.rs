use std::{collections::HashMap, cell::RefCell, panic};

use js_sys::JsString;
use log::*;
use screeps::{
    find, game, prelude::*, Creep, Part, RoomObjectProperties, StructureObject, JsHashMap, RawObjectId, StructureType, StructureSpawn, Source, ObjectId, RawMemory, StructureController, Room, Structure,
};

use serde::{Serialize, Deserialize};

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
    Controller(ControllerMemory),
    Empty,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct ControllerMemory {
    controller_level: usize
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct CreepMemory {
    worker_type: CreepWorkerType,
}

impl CreepMemory {
    pub fn default(room: Room) -> CreepMemory {
        CreepMemory {
            worker_type: CreepWorkerType::SimpleWorker(
                SimpleJob::ApproachSpawn(
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

        let structures = game::structures();

        new_structure_memories = run_structures(structures, structure_memories);
        new_creep_memories = run_creeps(creep_memories);
    });

    // Serialize and save to memory.
    save_memory(GameMemory {
        creep_memories: new_creep_memories,
        structure_memories: new_structure_memories,
        room_memories: HashMap::new(),
        needs_deserialized: false,
    });
}

fn run_structures(structures: JsHashMap<RawObjectId, StructureObject>, structure_memories: HashMap<ObjectId<Structure>,StructureMemory>) -> HashMap<ObjectId<Structure>,StructureMemory> {
    let mut values: HashMap<ObjectId<Structure>, StructureMemory> = HashMap::new();

    for structure in structures.values() {
        let structure_id = structure.as_structure().id();
        let structure_memory = structure_memories
            .get(&structure_id)
            .unwrap_or(&StructureMemory::Spawner(1))
            .to_owned();

        info!("{:?}", structure_memory);

        values.insert(structure_id, run_structure(structure, structure_memory));
    };

    values
}

// Structure memory isn't used _yet_, but implemented here to force me into using it later
fn run_structure(structure: StructureObject, _structure_memory: StructureMemory) -> StructureMemory {
    match structure.structure_type() {
        StructureType::Spawn => run_spawn(structure.try_into().unwrap()),
        StructureType::Controller => run_controller(structure.try_into().unwrap()),
        st => {
            warn!("Not yet implemented type: {:?}", st);

            StructureMemory::Empty
        } 
    }
}

fn run_controller(controller: StructureController) -> StructureMemory {
    // let room = controller.room().to_owned().unwrap();
    // let mut room_memory: RoomMemory = from_value::<RoomMemory>(
    //     room.memory()
    // ).unwrap_or(RoomMemory{
    //     controller_level: 1,
    // });

    // if room_memory.controller_level < controller.level().into() {
    //     room_memory.controller_level = controller.level().into();
    //     info!("Controller upgraded!");
    // }

    StructureMemory::Controller(ControllerMemory{
        controller_level: controller.level().into()
    })
}

fn run_spawn(spawn: StructureSpawn) -> StructureMemory {
    let creeps = game::creeps();
    if creeps.values().count() < 5 {
        let creep_name = format!("{}-{}", String::from("Creep"), game::time());
        spawn.spawn_creep(&vec![
            Part::Carry,
            Part::Move,
            Part::Work,
        ], &creep_name);
    }

    StructureMemory::Spawner(1)
}

fn run_creeps(creep_memories: HashMap<ObjectId<Creep>, CreepMemory>) -> HashMap<ObjectId<Creep>, CreepMemory> {
    let mut memories = HashMap::new();
    game::creeps().values().for_each(|creep| {
        if creep.spawning() {
            return;
        }

        let room = get_room_of::<Creep>(&creep);
        let creep_memory = match creep_memories.get(&creep.try_id().unwrap()) {
            Some(memory) => memory.to_owned(),
            None => CreepMemory::default(room)
        };

        let new_memory = run_creep(creep.to_owned(), creep_memory);

        info!("Creep: {:?}", creep.name());
        info!("Memory: {:?}", new_memory);

        memories.insert(creep.try_id().unwrap(), new_memory);
    });

    memories
}

fn get_room_of<T>(object: &dyn RoomObjectProperties) -> Room {
    object.room().unwrap()
}

 fn run_creep(creep: Creep, memory: CreepMemory) -> CreepMemory {
    let creep_room_spawn: &StructureSpawn = &creep.room().to_owned().unwrap().find(find::MY_SPAWNS)[0];

    // Break out memory values
    let CreepMemory { mut worker_type } = memory;

    let job = match worker_type {
        CreepWorkerType::SimpleWorker(job) => job
    };

    let keep_job = match job {
        SimpleJob::ApproachSource(target) => { 
            info!("Approach {:?}", target);
            CreepPurpose::move_near(&creep, target.resolve().unwrap().pos())
        }, 
        SimpleJob::HarvestSource(target) => { info!("Harvest {:?}", target); true },
        SimpleJob::ApproachController(target) => { info!("Approach {:?}", target); true },
        SimpleJob::UpgradeController(target) => { info!("Upgrade {:?}", target); true },
        SimpleJob::ApproachSpawn(target)=> { 
            info!("Approach {:?}", target); 
            CreepPurpose::move_near(&creep, target.resolve().unwrap().pos())
        },
        SimpleJob::TransferToSpawn(_target) => false,
    };

    let new_job: SimpleJob;
    if !keep_job {
        new_job = match job {
            SimpleJob::ApproachSource(target) => {
                SimpleJob::HarvestSource(target)
            }, 
            SimpleJob::HarvestSource(_target) => {
                SimpleJob::ApproachSpawn(creep_room_spawn.id())
            },
            SimpleJob::ApproachController(target) => {
                SimpleJob::UpgradeController(target)
            },
            SimpleJob::UpgradeController(_target) => {
                SimpleJob::ApproachSource(creep.room().to_owned().unwrap().find(find::SOURCES)[0].id())
            },
            SimpleJob::ApproachSpawn(target)=> {
                SimpleJob::TransferToSpawn(target)
            },
            SimpleJob::TransferToSpawn(_target) => {
                SimpleJob::HarvestSource(creep.room().to_owned().unwrap().find(find::SOURCES)[0].id())
            }
        };

        worker_type = CreepWorkerType::SimpleWorker(new_job);
    }


    CreepMemory {
        worker_type
    }
}
