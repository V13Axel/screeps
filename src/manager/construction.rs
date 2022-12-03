use log::info;
use screeps::{Room, find, StructureProperties, StructureType, Structure, StructureObject, ConstructionSite};

use crate::{mem::{GameMemory, RoomMemory}, room::RoomRectangle};

pub struct ConstructionManager {
    room: Room,
}

impl ConstructionManager {
    pub fn with_room(room: &Room) -> Self {
        Self {
            room: room.to_owned()
        }
    }

    pub fn scan(&self, memory: &mut RoomMemory) {
        // let room_grid = RoomRectangle::for_room(&self.room);
        let controller_level = self.room.controller().unwrap().level();
        if controller_level > 1 {
            self.choose_extension_locations(controller_level.into());
        }

        if controller_level > 2 {
            // self.choose_road_locations();
        }
    }

    fn choose_extension_locations(&self, controller_level: usize) {
        let levels = [
            0,
            5,
            10,
            20,
            30,
            40,
            50,
            60
        ];
        let desired_extension_count = levels.get(controller_level - 1).unwrap();
        let extensions = self.room.find(find::STRUCTURES);
        let only_extensions: Vec<&StructureObject> = extensions.iter().filter(|structure| structure.structure_type() == StructureType::Extension).collect();
        let construction_sites: Vec<ConstructionSite> = self.room.find(find::CONSTRUCTION_SITES);
        let extension_construction_sites: Vec<&ConstructionSite> = construction_sites.iter().filter(|site| site.structure_type() == StructureType::Extension).collect();

        let extensions_to_build = desired_extension_count - only_extensions.len() - extension_construction_sites.len();

        // info!("Found {} extensions and {} constructionsites. Will build {} extensions", only_extensions.len(), extension_construction_sites.len(), extensions_to_build);

        for _ in 1..(extensions_to_build+1) {

        }
    }

    fn choose_road_locations(&self) {
        // todo!()
    }
}
