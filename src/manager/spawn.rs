use std::collections::HashMap;

use log::info;
use screeps::{StructureSpawn, Room, find, Part, game, ENERGY, ResourceType};

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
            let id = spawner.room()
                .unwrap()
                .name();
            let room_tasks = game_memory.room_task_queues
                .entry(id.to_string())
                .or_default();

            self.spawn_if_needed(spawner.to_owned(), room_tasks);
        }
    }

    pub fn spawn_if_needed(&self, spawner: StructureSpawn, _room_tasks: &mut HashMap<MinionType, Vec<Task>>) {
        if spawner.spawning().is_some() || spawner.store().get_used_capacity(Some(ResourceType::Energy)) < 300 {
            println!("Can't spawn right now, energy too low or already spawning");

            return;
        }

        let room_creeps = spawner.room()
            .unwrap()
            .find(find::MY_CREEPS);
        let mut to_spawn: Option<(MinionType, Task)> = None;

        'outer: for (minion_type, tasks) in _room_tasks.iter() {
            for task in tasks.iter() {
                match task {
                    Task::Upgrade { controller, worked_by } => {
                        if worked_by.len() < 2 {
                            to_spawn = Some(
                                (minion_type.to_owned(), task.to_owned())
                            );

                            Self::spawn_it(minion_type, spawner);

                            break 'outer;
                        }
                    },
                    _ => {
                        println!("A different task");
                    }
                }
            }
        }

        if let Some((minion_type, task)) = to_spawn {
        }
    }

    fn spawn_it(minion_type: &MinionType, spawner: StructureSpawn) {
        let mut parts: Vec<Part> = vec![];
        let new_name = format!("{}{}", minion_type.to_string(), game::time());

        parts.push(Part::Move);
        // parts.push(Part::Move);
        parts.push(Part::Carry);
        parts.push(Part::Work);

        spawner.spawn_creep(&parts, &new_name);
    }
}
