use std::{collections::HashMap, cmp::Ordering};

use log::info;
use screeps::{Room, Creep, find, HasTypedId, MaybeHasTypedId};

use crate::{mem::{GameMemory, CreepMemory}, util, minion::MinionType, task::Task};

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
        let creep_type = &memory.worker_type;

        let tasks_for_creep = room_task_queues
            .entry(creep_room.to_string())
            .or_default()
            .entry(creep_type.to_owned())
            .or_default(); 

        info!("{:?}", tasks_for_creep);
    }

    pub fn scan(&self, game_memory: &mut GameMemory) {
        for room in self.rooms.iter() {
            let room_task_queues = game_memory.room_task_queues.entry(
                room.name().to_string()
            ).or_default();

            self.scan_room(&room, room_task_queues);
        }
    }

    pub fn scan_room(&self, room: &Room, room_task_queues: &mut HashMap<MinionType, Vec<Task>>) {
        Self::_room_upgrade_task(room, room_task_queues);
        Self::_source_harvesting_tasks(room, room_task_queues);
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

