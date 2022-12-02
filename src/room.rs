use std::fmt::Display;

use log::info;
use screeps::{Room, Look, Terrain, TERRAIN, RoomVisual, CircleStyle, RoomName};
use cli_table::{TableDisplay, Table};

pub struct RoomRectangle {
    coordinates: (usize, usize),
    name: String,
    spaces: [usize; 2500],
    width: usize,
    height: usize,
}

impl Display for RoomRectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: Vec<Vec<usize>> = vec![];
        for row in 0..self.height {
            output.push(self.spaces[(row * self.width)..((row*self.width) + self.width)].to_vec());
        }

        write!(f, "{}", output.table().display().unwrap())
    }
}

impl Default for RoomRectangle {
    fn default() -> Self {
        Self {
            coordinates: (0, 0),
            name: "Unset".into(),
            spaces: [0; 2500],
            width: 50,
            height: 50,
        }
    }
}

impl RoomRectangle {
    pub fn for_room(room: &Room) -> Self {
        let spaces = Self::scan_room(&room);

        let result = Self {
            name: room.name().to_string(),
            spaces,
            ..Self::default()
        };

        result.debug(room.name());

        result
    }

    fn debug(&self, room_name: RoomName) {
        for y in 0..50 {
            for x in 0..50 {
                let mut circle_style = CircleStyle::default();
                let color = match self.spaces[(x + (y * 50))] {
                    0 => "#000000",
                    1 => "#0000FF",
                    2 => "#00FF00",
                    3 => "#00FFFF",
                    4 => "#FF0000",
                    5 => "#FF00FF",
                    6 => "#FFFF00",
                    7 => "#FFFFFF",
                    _ => panic!("UM WHAT")
                };
                circle_style = circle_style.fill(color);
                RoomVisual::new(Some(room_name)).circle(x as f32, y as f32, Some(circle_style));
            }
        }
    }

    fn scan_room(room: &Room) -> [usize; 2500] {
        let mut positions: [usize; 2500] = [0; 2500];
        let mut indexes = vec![];
        for y in 0usize..50 {
            for x in 0usize..50 {
                let mut cell = 0;
                let found = room.look_for_at_xy(TERRAIN, x as u8, y as u8);

                for terrain in found.iter() {
                    match terrain {
                        Terrain::Wall => cell = 1 | cell,
                        Terrain::Plain => cell = 2 | cell,
                        Terrain::Swamp => cell = 4 | cell,
                    }
                }
                let index: usize = x + (y * 50);
                indexes.push(index);
                positions[index] = cell;
            }
        }

        positions
    }
}
