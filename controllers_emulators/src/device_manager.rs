use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::device::{Device, self};

pub struct DeviceManager {
    current_dev: u64,
    devices: HashMap<u64, Arc<Mutex<dyn Device>>>
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceManager {
    fn new() -> Self {
        Self {
            current_dev: 0,
            devices: HashMap::<u64, Arc<Mutex<dyn Device>>>::default()
        }
    }

    pub fn alloc(&mut self, dev: Arc<Mutex<dyn Device>>) -> u64 {
        self.current_dev += 1;

        self.devices.insert(self.current_dev, dev);

        self.current_dev
    }

    pub fn fetch(&self, hash: u64) -> Option<Arc<Mutex<dyn Device>>> {
        match self.devices.get(&hash) {
            Some(dev) => Some(dev.clone()),
            None => None
        }
    }
}

