use screeps::{Room, Path, RoomPosition};
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;

pub enum MovementDistance {
    At,
    Near
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreepPath {
    pub value: String
}

impl CreepPath {
    pub fn determine(room: Room, start: &RoomPosition, end: &RoomPosition, distance: MovementDistance) -> Self {
        let path = room
            .find_path(
                &start, 
                &end, 
                None
            );

        match distance {
            MovementDistance::At => CreepPath::from(path),
            MovementDistance::Near => {
                let mut vecpath = match path {
                    Path::Vectorized(steps) => steps,
                    Path::Serialized(steps) => Room::deserialize_path(&steps),
                };
                vecpath.pop();

                CreepPath::from(Path::Vectorized(vecpath))
            }
        }
    }
}

impl From<Path> for CreepPath {
    fn from(path: Path) -> Self {
        Self {
            value: match path {
                Path::Vectorized(vector) => Room::serialize_path(&vector),
                Path::Serialized(string) => string,
            }
        }
    }
}

impl Into<JsValue> for CreepPath {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.value)
    }
}
