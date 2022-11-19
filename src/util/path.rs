use js_sys::Array;
use log::info;
use screeps::{Position, RoomPosition};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreepPath {
    pub steps: Vec<Position>
}

impl From<Vec<Position>> for CreepPath {
    fn from(steps: Vec<Position>) -> Self {
        CreepPath {
            steps 
        }
    }
}

impl From<Array> for CreepPath {
    fn from(steps: Array) -> Self {
        let mut positions = vec![];
        for step in steps.values() {
            let room_position = RoomPosition::from(step.unwrap());
            let position_version = Position::from(room_position);

            positions.push(position_version);
        }

        Self {
            steps: positions
        }
    }
}
