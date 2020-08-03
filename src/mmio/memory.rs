pub const GPIO_OFFSET: usize = 0x0020_0000;
pub const UART0_OFFSET: usize = 0x0020_1000;

pub const BASE: usize = 0x3F00_0000;
pub const GPIO_BASE: usize = BASE + GPIO_OFFSET;
pub const UART0_BASE: usize = BASE + UART0_OFFSET;