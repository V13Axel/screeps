use screeps::{RoomPosition, Room, LookResult, Terrain};

pub struct PositionCalculator;

impl PositionCalculator {
    pub fn spaces_around(room: &Room, position: RoomPosition) -> usize {
        let x = position.x();
        let y = position.y();
        let mut space_limit = 8;

        for xpos in (x-1)..(x+2) {
            for ypos in (y-1)..(y+2) {
                if ypos == y && xpos == x {continue};

                let has_wall = room.look_at(&room.get_position_at(xpos, ypos));
                if has_wall.len() > 0 {
                    for item in &has_wall {
                        match item {
                            LookResult::Terrain(kind) => match kind {
                                Terrain::Wall => {space_limit-=1},
                                _ => {}
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        space_limit
    } 
}
