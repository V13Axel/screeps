use js_sys::Array;

use screeps::{Position, Room};
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreepPath {
    pub value: String
}

impl From<Vec<Position>> for CreepPath {
    fn from(steps: Vec<Position>) -> Self {
        let positions = Array::new();
        for position in steps.iter() {
            positions.push(&to_value(position).unwrap());
        }

        CreepPath {
            value: Room::serialize_path(&positions).into()
        }
    }
}

impl From<Array> for CreepPath {
    fn from(steps: Array) -> Self {
        Self {
            value: Room::serialize_path(&steps).into()
        }
    }
}

impl Into<JsValue> for CreepPath {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.value)
    }
}
