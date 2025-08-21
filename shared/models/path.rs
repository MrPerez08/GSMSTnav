use serde::{Deserialize, Serialize};
use super::Location;

#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
pub struct NavigationPath {
    pub points: Vec<Location>,
    pub distance: f64,
}