use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(C)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub building_id: String,
    pub floor: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
pub struct NavigationPath {
    pub points: Vec<Location>,
    pub distance: f64,
}