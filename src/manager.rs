use std::cmp::Ordering;

use log::info;
use screeps::{Room, find, HasTypedId, game, SharedCreepProperties, MaybeHasTypedId, StructureSpawn, Part, RoomPosition, Terrain};

use crate::{mem::{GameMemory, CreepMemory}, task::Task, minion::CreepWorkerType, util};

pub struct TaskManager {
    rooms: Vec<Room>,
}

impl TaskManager {
    pub fn with_rooms(rooms: &Vec<Room>) -> Self {
        TaskManager {
            rooms: rooms.iter().map(|room| room.to_owned()).to_owned().collect()
        }
    }

    pub fn assign(mut game_memory: GameMemory) -> GameMemory {
        let creeps = util::screeps::Screeps::get_idle_screeps(&game_memory);

        for creep in creeps {
            if creep.spawning() {
                info!("Skipping {} due to spawning", creep.name());
                continue;
            }

            let mut creep_memory: CreepMemory = game_memory.creep_memories.get(&creep.name()).unwrap_or_default().to_owned();
            info!("Creep - {:?}", creep_memory);

            let creep_room = &creep.room().unwrap().name().to_string();

            match game_memory.tasks.get(creep_room) {
                Some(room_tasks) => {
                    info!("Room has tasks: {:?}", room_tasks);
                    let mut copied_tasks = room_tasks.to_owned();
                    let mut creep_task = copied_tasks.pop().unwrap_or(Task::Idle);

                    creep_task = match creep_task {
                        Task::Idle => Task::Idle,
                        Task::Harvest { node, mut worked_by, space_limit } => {
                            worked_by.push(creep.try_id().unwrap());

                            Task::Harvest { node, worked_by, space_limit }
                        }
                        Task::Build { site, mut worked_by } => {
                            worked_by.push(creep.try_id().unwrap());

                            Task::Build { 
                                site, 
                                worked_by 
                            }
                        },
                        _ => todo!("Haven't implemented that yet"),
                    };

                    copied_tasks.push(creep_task.to_owned());

                    game_memory.tasks.insert(creep_room.to_string(), copied_tasks);

                    creep_memory.worker_type = CreepWorkerType::SimpleWorker(creep_task.to_owned());

                    game_memory.creep_memories.insert(creep.name(), creep_memory);
                },
                None => {
                    info!("Room has no tasks");
                } 
            };

            // We only get this far if the screep is idle. Let's give them a task!
        }


        game_memory
    }

    pub fn scan(&self, mut game_memory: GameMemory) -> GameMemory {
        for room in self.rooms.iter() {
            game_memory.tasks.insert(room.name().to_string(), self.scan_room(&room));
        }

        game_memory
    }

    pub fn scan_room(&self, room: &Room) -> Vec<Task> {
        let mut room_tasks: Vec<Task> = vec![];
        let spawn = &room.find(find::MY_SPAWNS)[0];

        let mut sources = room.find(find::SOURCES);
        sources.sort_by(|a, b| {
            if spawn.pos().get_range_to(a) > spawn.pos().get_range_to(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        for source in sources.iter() {
            let found: Vec<&Task> = room_tasks.iter().filter(|task| {
                match task {
                    Task::Harvest { node, worked_by: _, space_limit: _ } => node.to_string() == source.id().to_string(),
                    _ => false
                }
            }).collect();

            let position  = source.pos();
            let x = position.x();
            let y = position.y();
            let room_name = room.name();
            let space_limit = 0;

            for check_x in (x-1)..(x+1) {
                for check_y in (y-1)..(y+1) {
                    RoomPosition::new(check_x, check_y, room_name).look().to_vec().iter().for_each(|item| {
                        info!("{:?}", item);
                    });

                    // info!("{:?}", values);
                }
            }

            if found.len() > 0 {
                continue;
            }

            room_tasks.push(Task::Harvest { node: source.id(), worked_by: vec![], space_limit });
        }

        room_tasks
    }
}

pub struct SpawnManager {
    spawners: Vec<StructureSpawn>,
}

impl SpawnManager {
    pub fn with_rooms(rooms: &Vec<Room>) -> Self {
        let spawners: Vec<StructureSpawn> = rooms.iter().map(|room| {
            room.find(find::MY_SPAWNS)[0].to_owned()
        }).collect();

        SpawnManager {
            spawners
        }
    }


    pub fn spawn(&self, mut game_memory: GameMemory) -> GameMemory {
        for spawner in self.spawners.iter() {
            info!("Running spawner {}", spawner.name().to_string());
            let id = spawner.room().unwrap().name();
            let room_tasks: Vec<Task> = game_memory.tasks.get(&id.to_string()).unwrap_or(&vec![]).to_owned();
            let result = self.spawn_if_needed(spawner.to_owned(), room_tasks);

            if result.is_some() {
                let (creep_name, creep_memory) = result.unwrap();
                game_memory.creep_memories.insert(creep_name, creep_memory);
            }
        }

        game_memory
    }

    pub fn spawn_if_needed(&self, spawner: StructureSpawn, _room_tasks: Vec<Task>) -> Option<(String, CreepMemory)> {
        let room_creeps = spawner.room().unwrap().find(find::MY_CREEPS);
        let mut parts: Vec<Part> = vec![];
        let new_name = format!("Worker{}", game::time());

        parts.push(Part::Move);
        parts.push(Part::Carry);
        parts.push(Part::Work);


        if room_creeps.len() < 5 && spawner.spawning().is_none() {
            spawner.spawn_creep(&parts, &new_name);

            return Some((new_name, CreepMemory::default()));
        }

        None
    }
}
