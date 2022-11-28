use std::collections::HashMap;

use log::info;
use screeps::{StructureSpawn, Room, find, Part, game};

use crate::{mem::{GameMemory, CreepMemory}, minion::MinionType, task::Action};

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
            let id = spawner.room()
                .unwrap()
                .name();
            let room_tasks = game_memory.room_task_queues
                .entry(id.to_string())
                .or_default();

            self.spawn_if_needed(spawner.to_owned(), room_tasks, &mut game_memory.creeps);
        }
    }

    pub fn spawn_if_needed(&self, spawner: StructureSpawn, _room_tasks: &mut HashMap<MinionType, Vec<Action>>, creep_memories: &mut HashMap<String, CreepMemory>) {
        // if spawner.spawning().is_some() || spawner.store().get_used_capacity(Some(ResourceType::Energy)) < 300 {
        //     info!("Can't spawn right now, energy too low or already spawning");
        //
        //     return;
        // }

        for (minion_type, tasks) in _room_tasks.iter() {
            for task in tasks.iter() {
                self.spawn_it(minion_type, spawner, task, creep_memories);

                return;
            }
        }
    }

    fn spawn_it(&self, minion_type: &MinionType, spawner: StructureSpawn, task: &Action, creep_memories: &mut HashMap<String, CreepMemory>) {
        let mut parts: Vec<Part> = vec![];
        let new_name = format!("{}{}", minion_type.to_string(), game::time());

        parts.push(Part::Move);
        // parts.push(Part::Move);
        parts.push(Part::Carry);
        parts.push(Part::Work);

        creep_memories.insert(new_name.to_owned(), CreepMemory {
            worker_type: minion_type.to_owned(),
            ..Default::default()
        });

        spawner.spawn_creep(&parts, &new_name);
    }
}
