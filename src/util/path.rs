use screeps::Position;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreepPath {
    steps: Vec<Position>
}

impl CreepPath {
    pub fn from(steps: Vec<Position>) -> Self {
        CreepPath {
            steps 
        }
    }
}
