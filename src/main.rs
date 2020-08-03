#![no_std]
#![no_main]
#![feature(asm)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(global_asm)]

mod boot;
mod driver;
mod exception;
mod mmio;
mod print;
mod util;

use core::panic::PanicInfo;

use driver::init_drivers;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    init_drivers();
    kernel_main()
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Invoke a divide by zero error
    println!("Hello World!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match (info.location(), info.message()) {
        (Some(loc), Some(msg)) => {
            println!("<kernel> panicked at '{}', {}", msg, loc);
        }
        _ => println!("kernel panic!"),
    }
    loop {}
}
