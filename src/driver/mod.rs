pub mod common;
mod raspi3;

use crate::driver::common::DriverManager;
use crate::driver::raspi3::RASPI3_DEVICE_MANAGER;

static GLOBAL_DRIVER_MANAGER: GlobalDriverManager = GlobalDriverManager {
    driver_managers: [&crate::driver::RASPI3_DEVICE_MANAGER],
};

struct GlobalDriverManager {
    driver_managers: [&'static (dyn DriverManager + Sync); 1],
}

impl DriverManager for GlobalDriverManager {
    fn init_drivers(&self) {
        for driver_manager in self.driver_managers.iter() {
            driver_manager.init_drivers();
        }
    }
}

pub fn init_drivers() {
    GLOBAL_DRIVER_MANAGER.init_drivers();
}
