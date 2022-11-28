use std::{collections::HashMap, cmp::Ordering};

use log::{debug, info};
use screeps::{Room, Creep, SharedCreepProperties, find, HasTypedId, HasId};

use crate::{mem::{GameMemory, CreepMemory}, util::{self, screeps::Screeps}, minion::{MinionType, Minions}, task::{upgrade::Upgrade, harvest::Harvest, TaskProps, Action}};

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
        let creeps = Screeps::get_idle_screeps(&game_memory);

        for creep in creeps {
            if creep.spawning() {
                continue;
            }

            Self::assign_creep(
                &creep,
                &mut game_memory.creeps.entry(creep.name()).or_default(),
                &mut game_memory.room_task_queues
            );
        }
}

    fn assign_creep(
        creep: &Creep, 
        memory: &mut CreepMemory, 
        room_task_queues: &mut HashMap<String, HashMap<MinionType, Vec<Action>>>
    ) {
        let creep_room = &creep.room().unwrap().name().to_string();
        let creep_type = &memory.worker_type;

        let tasks_for_creep = room_task_queues
            .entry(creep_room.to_string())
            .or_default()
            .entry(creep_type.to_owned())
            .or_default();

        // info!("Assigning for {:?}", creep_type);
        // info!("{:?}", tasks_for_creep);
        
        if let Some(task) = tasks_for_creep.pop() {
            memory.current_task = task;
        }
    }

    pub fn scan(&self, game_memory: &mut GameMemory) {
        for room in self.rooms.iter() {
            self.scan_room(&room, game_memory);
        }
    }

    pub fn scan_room(&self, room: &Room, game_memory: &mut GameMemory) {
        Self::_room_upgrade_tasks(room, game_memory);
        Self::_source_harvesting_tasks(room, game_memory);
    }

    fn _room_upgrade_tasks(room: &Room, game_memory: &mut GameMemory) {
        let upgrading_creeps = Screeps::get_screeps_doing(Upgrade::for_room(room), game_memory);
        let total_tasks = 5 - upgrading_creeps.len();

        let room_tasks = game_memory.room_task_queues.entry(room.name().to_string()).or_default().entry(MinionType::Upgrader).or_default();
        let needed_tasks = std::cmp::max(0, total_tasks - room_tasks.len());

        if room_tasks.len() > 5 || needed_tasks > 5 {
            info!("room_tasks length is {}, needed tasks is {}. One of those means we should clear the queue.", room_tasks.len(), needed_tasks);
            // We messed up
            room_tasks.clear();
            return;
        }

        info!("Upgrade needs {} tasks: (5 - {}) - {}", needed_tasks, upgrading_creeps.len(), room_tasks.len());

        for _ in 1..needed_tasks {
            room_tasks.push(
                Upgrade::for_room(room)
            )
        }
    }

    fn _source_harvesting_tasks(room: &Room, game_memory: &mut GameMemory) {
        let spawn = &room.find(find::MY_SPAWNS)[0];

        let mut sources = room.find(find::SOURCES);
        let mut max_possible_sources = 0;


        sources.sort_by(|a, b| {
            if spawn.pos().get_range_to(a) > spawn.pos().get_range_to(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        // Sources to harvest
        for source in sources.iter() {
            let spaces_available = util::position::PositionCalculator::spaces_around(&room, source.pos());
            max_possible_sources += spaces_available;

            let working_creeps = Screeps::get_screeps_doing(Action::Harvest(source.id()), game_memory);
            let room_tasks = game_memory
                .room_task_queues
                .entry(room.name().to_string())
                .or_default()
                .entry(MinionType::Harvester)
                .or_default();
            let total_tasks = spaces_available - working_creeps.len();

            let needed_tasks = std::cmp::max(0, total_tasks - room_tasks.len());


            info!("Harvest needs {} tasks: ({} - {}) - {}", needed_tasks, spaces_available, working_creeps.len(), room_tasks.len());

            for _ in 1..needed_tasks {
                room_tasks.push(Action::Harvest(source.id()));
            }
            if room_tasks.len() > max_possible_sources {
                info!("room_tasks length is {}, max_possible is {}. Clearing the harvester queue.", room_tasks.len(), max_possible_sources);
                // We messed up
                room_tasks.clear();
                return;
            }
        }
    }
}

