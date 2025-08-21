use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub building_id: String,
    pub floor: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationPath {
    pub points: Vec<Location>,
    pub distance: f64,
}

#[no_mangle]
pub extern "C" fn calculate_path(
    start: *const Location,
    end: *const Location,
) -> *mut NavigationPath {
    // Path calculation logic
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn free_navigation_path(path: *mut NavigationPath) {
    unsafe {
        if !path.is_null() {
            Box::from_raw(path);
        }
    }
}