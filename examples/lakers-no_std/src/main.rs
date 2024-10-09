#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::debug::{self, EXIT_SUCCESS};

#[cfg(not(target_abi = "eabihf"))]
use cortex_m_semihosting::hprintln as info;

#[cfg(target_abi = "eabihf")]
use defmt::info;

use defmt_rtt as _; // global logger

use panic_semihosting as _;

extern crate alloc;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// ================================ paint the stack ===============================
use core::arch::asm;
use core::ptr::addr_of;
use cortex_m::register::msp;

extern "C" {
    // marks the end of the stack, see .map file
    static mut __euninit: u8;
}

// using asm because if I use cortex_m::register::msp::read(), it sometimes crashes
fn get_stack_pointer() -> usize {
    let stack_pointer: *const u8;
    unsafe {
        asm!("mov {}, sp", out(reg) stack_pointer);
    }
    stack_pointer as usize
}

fn get_stack_end() -> usize {
    unsafe { addr_of!(__euninit) as *const u8 as usize }
}

fn paint_stack(pattern: u32) {
    let stack_end = get_stack_end();
    let stack_pointer = get_stack_pointer();
    info!("PAINT_STACK stack end: {:#X}", stack_end);
    info!("PAINT_STACK stack pointer is at: {:#X}", stack_pointer);
    let mut addr = stack_pointer;
    info!(
        "PAINT_STACK will paint a total of {} bytes, from {:#X} to {:#X}",
        (addr - stack_end),
        addr,
        stack_end
    );
    while addr > stack_end {
        unsafe {
            core::ptr::write_volatile(addr as *mut u32, pattern);
        }
        addr -= 4;
    }
    info!(
        // do not remove the ==, it is used in the script to parse the output
        "== PAINT_STACK painted a total of {} bytes, from {:#X} to {:#X} ==",
        (stack_pointer - addr),
        stack_pointer,
        addr
    );
}
// ================================================================================

// sample function that uses some stack space (increase number_of_calls to use more)
fn dummy(number_of_calls: usize) {
    let arr = [0u8; 4];
    if number_of_calls > 0 && arr[0] == 0 {
        info!(">> dummy: stack pointer: {:?}", get_stack_pointer());
        dummy(number_of_calls - 1);
    }
}

#[entry]
fn main() -> ! {
    paint_stack(0xDEAD_BEEF);

    info!("BEGIN test.");

    dummy(1);

    info!("END test.");

    // exit via semihosting call
    debug::exit(EXIT_SUCCESS);

    // the cortex_m_rt `entry` macro requires `main()` to never return
    loop {}
}

use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn strstr(_cs: *const c_char, _ct: *const c_char) -> *mut c_char {
    panic!("strstr handler!");
}
