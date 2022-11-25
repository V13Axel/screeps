use screeps::{Room, Path, Position, RoomPosition};
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreepPath {
    pub value: String
}

impl CreepPath {
    pub fn determine(room: Room, start: &RoomPosition, end: &RoomPosition) -> Self {
        CreepPath::from(room
            .find_path(
                &start, 
                &end, 
                None
            )
        )
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
