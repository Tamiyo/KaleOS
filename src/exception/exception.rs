use core::fmt;
use cortex_a::{asm, barrier, regs::*};
use register::InMemoryRegister;

global_asm!(include_str!("exception.S"));

#[repr(transparent)]
struct SpsrEL1(InMemoryRegister<u32, SPSR_EL1::Register>);

#[repr(C)]
struct ExceptionContext {
    gpr: [u64; 30],

    lr: u64,

    elr_el1: u64,

    spsr_el1: SpsrEL1,
}

#[no_mangle]
unsafe extern "C" fn current_el0_synchronous(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

fn default_exception_handler(e: &ExceptionContext) {
    panic!("CPU Exception Occured!")
}
