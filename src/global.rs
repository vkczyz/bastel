use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Global {
    pub title: String,
    pub window_size: (u32, u32),
    pub view_size: (u32, u32),
    pub view_origin: (u32, u32),
    pub entity_map: HashMap<String, u32>,
    pub signals: HashMap<String, bool>,
}

impl Global {
    pub fn new(title: String, window_size: (u32, u32)) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
            Global {
            title,
            window_size,
            view_size: window_size,
            view_origin: (0, 0),
            entity_map: HashMap::new(),
            signals: HashMap::new(),
            }
        ))
    }
}