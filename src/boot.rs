global_asm!(include_str!("boot.S"));

pub unsafe fn zero_bss() {
    extern "C" {
        static mut __bss_start: usize;
        static mut __bss_end: usize;
    }

    let mut start = &mut __bss_start as *mut usize;
    let end = &mut __bss_end;

    while start < end {
        core::ptr::write_volatile(start as *mut usize, 0);
        start = start.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() -> ! {
    zero_bss();
    crate::kernel_init()
}
