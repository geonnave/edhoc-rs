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

// sample function that uses some stack space (increase number_of_calls to use more)
fn dummy(number_of_calls: usize) {
    let arr = [0u8; 5];
    if number_of_calls > 0 && arr[0] == 0 {
        // get stack pointer
        let stack_pointer: *const u8;
        unsafe { core::arch::asm!("mrs {}, msp", out(reg) stack_pointer) };
        info!(">> dummy: stack pointer: {:#X}", stack_pointer as usize);
        dummy(number_of_calls - 1);
    }
}

extern "C" {
    static mut __sheap: u8;
    static mut _stack_start: u8;
}
#[cortex_m_rt::pre_init]
unsafe fn pre_init() {
    let mut addr;
    // get heap start
    extern "C" {
        static mut __sheap: u8;
    }
    let heap_start = core::ptr::addr_of!(__sheap) as *mut u8 as usize;

    // get stack pointer
    let stack_pointer: *const u8;
    core::arch::asm!("mrs {}, msp", out(reg) stack_pointer);
    let stack_pointer = stack_pointer as usize - 4;

    // paint the stack
    addr = heap_start;
    while addr < stack_pointer {
        unsafe {
            core::ptr::write_volatile(addr as *mut u32, 0xDEAD_BEEF);
        }
        addr += 4;
    }
}

#[entry]
fn main() -> ! {
    info!("__sheap: {:#X}", unsafe {
        core::ptr::addr_of!(__sheap) as *const _ as usize
    });
    info!("_stack_start: {:#X}", unsafe {
        core::ptr::addr_of!(_stack_start) as *const _ as usize
    });

    info!("BEGIN test.");

    dummy(2);

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
