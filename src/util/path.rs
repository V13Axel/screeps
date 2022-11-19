use js_sys::Array;
use log::info;
use screeps::Position;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreepPath {
    steps: Vec<Position>
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
        let positions = vec![];
        for step in steps.values() {
            info!("{:?}", step);
        }

        Self {
            steps: positions
        }
    }
}
