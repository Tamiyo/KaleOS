pub trait DeviceDriver {
    fn display_name(&self) -> &str;

    fn init(&self) -> Result<(), ()> {
        Ok(())
    }
}

pub trait DriverManager {
    fn init_drivers(&self);
}
