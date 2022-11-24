use std::{cmp::Ordering, collections::HashMap};

use log::info;
use screeps::{Room, find, HasTypedId, game, SharedCreepProperties, MaybeHasTypedId, StructureSpawn, Part, Creep, Terrain, LookResult};

use crate::{mem::{GameMemory, CreepMemory}, task::Task, minion::MinionType, util};

pub fn run_managers(memory: &mut GameMemory) {
    // Only want to run managers if it's been 20 ticks.
    if game::time() - memory.last_managers_tick < 20 {
        return;
    }

    handle_managers(memory);

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
                &mut game_memory.creeps.entry(creep.try_id().unwrap()).or_default(),
                &mut game_memory.room_task_queues
            );
        }
}

    fn assign_creep(
        creep: &Creep, 
        memory: &mut CreepMemory, 
        room_task_queues: &mut HashMap<String, HashMap<MinionType, Vec<Task>>>
    ) {
        let creep_room = &creep.room().unwrap().name().to_string();

        let room_tasks_by_minion_type = room_task_queues.entry(creep_room.to_string()).or_default(); 

        info!("{:?}", room_tasks_by_minion_type);
    }

    pub fn scan(&self, game_memory: &mut GameMemory) {
        for room in self.rooms.iter() {
            let room_task_queues = game_memory.room_task_queues.entry(
                room.name().to_string()
            ).or_default();

            self.scan_room(&room, room_task_queues);
        }
    }

    fn _room_upgrade_task(room: &Room, room_task_queues: &mut HashMap<MinionType, Vec<Task>>) {
        if !room_task_queues.contains_key(&MinionType::Upgrader) {
            // Controller to upgrade
            room_task_queues.insert(
                MinionType::Upgrader,
                vec![Task::Upgrade { controller: room.controller().unwrap().id(), worked_by: vec![] }]
            );
        }
    }

    pub fn scan_room(&self, room: &Room, room_task_queues: &mut HashMap<MinionType, Vec<Task>>) {
        Self::_room_upgrade_task(room, room_task_queues);

    }

    fn _source_harvesting_tasks(room: &Room, room_task_queues: &mut HashMap<MinionType, Vec<Task>>) {
        // todo: Probably ought to have room_task_queues for refilling spawns
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
            let space_limit = util::position::PositionCalculator::spaces_around(&room, source.pos());

            room_task_queues.entry(MinionType::Harvester)
                .or_default()
                .push(
                    Task::Harvest { 
                        node: source.id(), 
                        worked_by: vec![], 
                        space_limit 
                    }
                );
        }
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
            let room_tasks = game_memory.room_task_queues.entry(id.to_string()).or_default();
            self.spawn_if_needed(spawner.to_owned(), room_tasks);
        }
    }

    pub fn spawn_if_needed(&self, spawner: StructureSpawn, _room_tasks: &mut HashMap<MinionType, Vec<Task>>) -> Option<(String, CreepMemory)> {
        let room_creeps = spawner.room().unwrap().find(find::MY_CREEPS);
        let creeps_needed = _room_tasks.iter().fold(0, |total, tasks| {
            total
        });
        let mut parts: Vec<Part> = vec![];
        let new_name = format!("Worker{}", game::time());

        parts.push(Part::Move);
        // parts.push(Part::Move);
        parts.push(Part::Carry);
        parts.push(Part::Work);


        if room_creeps.len() < creeps_needed && spawner.spawning().is_none() {
            info!("Need {} creeps, have {}. Spawning.", creeps_needed, room_creeps.len());
            let result = spawner.spawn_creep(&parts, &new_name);
            info!("Spawn result: {:?}", result);

            return Some((new_name, CreepMemory::default()));
        } else {
            info!("Need {} creeps, have {}. Not spawning.", creeps_needed, room_creeps.len());
        }

        None
    }
}
