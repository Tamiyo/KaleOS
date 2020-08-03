mod common;
mod gpio;
mod memory;
mod uart0;

use crate::mmio::memory::UART0_BASE;
use crate::mmio::uart0::UART0;

pub static UART0: UART0 = unsafe { UART0::new(UART0_BASE) };
