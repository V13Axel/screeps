use std::{cmp::Ordering, collections::HashMap};

use log::{info, debug};
use screeps::{Room, find, HasTypedId, game, SharedCreepProperties, MaybeHasTypedId, StructureSpawn, Part, Creep, Position, RoomPosition, Terrain, LookResult};

use crate::{mem::{GameMemory, CreepMemory}, task::Task, minion::CreepWorkerType, util::{self, console::clear_console}};

pub fn run_managers(memory: &mut GameMemory) {
    let tick_since_last = game::time() - memory.last_managers_tick;
    debug!("Last managers tick: {:?}\nCurrent tick: {:?}\nDifference: {:?}", memory.last_managers_tick, game::time(), tick_since_last);

    // Ok so this is bit of a debugging hack.
    // Basically, we want to clear the console
    // a single tick before managers get handled.
    if tick_since_last == 49 {
        clear_console();
    }

    // Only want to run managers if it's been 20 ticks.
    if tick_since_last < 20 {
        return;
    }

    handle_managers(memory);

    info!("setting last_managers_tick");
    memory.last_managers_tick = game::time();
}

fn handle_managers(memory: &mut GameMemory) {
    let rooms: Vec<Room> = game::rooms()
        .values()
        .collect();

    TaskManager::with_rooms(&rooms).scan(memory);
    TaskManager::assign(memory);

    SpawnManager::with_rooms(&rooms).spawn(memory);
}

pub struct TaskManager {
    rooms: Vec<Room>,
}

impl TaskManager {
    pub fn with_rooms(rooms: &Vec<Room>) -> Self {
        TaskManager {
            rooms: rooms.iter()
                .map(|room| room.to_owned())
                .to_owned()
                .collect()
        }
    }

    pub fn assign(game_memory: &mut GameMemory) {
        let creeps = util::screeps::Screeps::get_idle_screeps(&game_memory);

        for creep in creeps {
            if creep.spawning() {
                continue;
            }

            Self::assign_creep(
                &creep,
                &mut game_memory.creeps.entry(creep.name()).or_default(),
                &mut game_memory.tasks
            );
        }
}

    fn assign_creep(creep: &Creep, memory: &mut CreepMemory, tasks: &mut HashMap<String, Vec<Task>>) {
        info!("Creep - {:?}", memory);

        let creep_room = &creep.room().unwrap().name().to_string();

        match tasks.get(creep_room) {
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

                tasks.insert(creep_room.to_string(), copied_tasks);

                memory.current_task = creep_task.to_owned();
                memory.worker_type = CreepWorkerType::SimpleWorker;

            },
            None => {
                info!("Room has no tasks");
            } 
        };
    }

    pub fn scan(&self, game_memory: &mut GameMemory) {
        for room in self.rooms.iter() {
            let tasks = game_memory.tasks.entry(
                room.name().to_string()
            ).or_default();

            self.scan_room(&room, tasks);
        }
    }

    pub fn scan_room(&self, room: &Room, tasks: &mut Vec<Task>) {
        // todo: Probably ought to have tasks for refilling spawns
        let spawn = &room.find(find::MY_SPAWNS)[0];

        let mut sources = room.find(find::SOURCES);

        sources.sort_by(|a, b| {
            if spawn.pos().get_range_to(a) > spawn.pos().get_range_to(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        // Sources to harvest
        for source in sources.iter() {
            let found: Vec<&Task> = tasks.iter().filter(|task| {
                match task {
                    Task::Harvest { node, worked_by: _, space_limit: _ } => node.to_string() == source.id().to_string(),
                    _ => false
                }
            }).collect();


            if found.len() > 0 {
                continue;
            }

            let source_position = source.pos();
            let x = source_position.x();
            let y = source_position.y();
            let mut space_limit = 0;

            info!("Around {},{}", x, y);

            for xpos in (x-1)..(x+2) {
                for ypos in (y-1)..(y+2) {
                    info!("{},{}", xpos, ypos);
                    if ypos == y && xpos == x {continue};

                    let has_wall = room.look_at(&room.get_position_at(xpos, ypos));
                    if has_wall.len() > 0 {
                        for item in &has_wall {
                            match item {
                                LookResult::Terrain(kind) => match kind {
                                    Terrain::Wall => {space_limit+=1},
                                    _ => {}
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }

            tasks.push(Task::Harvest { node: source.id(), worked_by: vec![], space_limit });
        }
        
        // // Controller to upgrade
        // let upgrade_tasks = room_tasks.iter().filter(|task| {
        //     match task {
        //         Task::Upgrade { .. } => true,
        //         _ => false,
        //     }
        // }).collect::<Vec<&Task>>();
        //
        // info!("{:?}", upgrade_tasks);
        //
        // if upgrade_tasks.len() < 1 {
        //     room_tasks.push(Task::Upgrade { controller: room.controller().unwrap().id(), worked_by: vec![] });
        // }
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


    pub fn spawn(&self, game_memory: &mut GameMemory) {
        for spawner in self.spawners.iter() {
            info!("Running spawner {}", spawner.name().to_string());
            let id = spawner.room().unwrap().name();
            let room_tasks: Vec<Task> = game_memory.tasks.get(&id.to_string()).unwrap_or(&vec![]).to_owned();
            let result = self.spawn_if_needed(spawner.to_owned(), room_tasks);

            if result.is_some() {
                let (creep_name, creep_memory) = result.unwrap();
                game_memory.creeps.insert(creep_name, creep_memory);
            }
        }
    }

    pub fn spawn_if_needed(&self, spawner: StructureSpawn, _room_tasks: Vec<Task>) -> Option<(String, CreepMemory)> {
        let room_creeps = spawner.room().unwrap().find(find::MY_CREEPS);
        let mut parts: Vec<Part> = vec![];
        let new_name = format!("Worker{}", game::time());

        parts.push(Part::Move);
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
