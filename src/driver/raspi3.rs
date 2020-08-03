use crate::driver::common::DeviceDriver;
use crate::driver::common::DriverManager;

use crate::println;

pub static RASPI3_DEVICE_MANAGER: Raspi3DriverManager = Raspi3DriverManager {
    drivers: [&crate::mmio::UART0],
};

pub struct Raspi3DriverManager {
    drivers: [&'static (dyn DeviceDriver + Sync); 1],
}

impl DriverManager for Raspi3DriverManager {
    fn init_drivers(&self) {
        for driver in self.drivers.iter() {
            match driver.init() {
                Ok(()) => (),
                Err(()) => {
                    println!("Error loading driver: {}", driver.display_name());
                    loop {}
                }
            }
        }
    }
}
