use std::collections::HashMap;

use log::info;
use screeps::{StructureSpawn, Room, find, Part, game};

use crate::{mem::{GameMemory, CreepMemory}, minion::MinionType, task::Task};

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
        let room_creeps = spawner.room()
            .unwrap()
            .find(find::MY_CREEPS);
        let creeps_needed = _room_tasks.iter().fold(0, |total, tasks| {
            info!("{:?}", tasks);

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
