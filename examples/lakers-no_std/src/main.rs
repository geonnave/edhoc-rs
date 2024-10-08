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

use core::arch::asm;

fn get_sp() -> *const u8 {
    let sp: *const u8;
    unsafe {
        asm!("mov {}, sp", out(reg) sp);
    }
    sp
}

fn paint_ram_from_sp_to_limit(limit: usize) {
    let sp = get_sp() as usize;
    let mut addr = sp;
    info!(
        "PAINT will paint a total of {} bytes, from {:X} to {:X}",
        (sp - limit),
        sp,
        limit
    );
    while addr > limit {
        unsafe {
            core::ptr::write_volatile(addr as *mut u32, 0xDEADBEEF);
        }
        addr -= 4;
    }
    info!(
        "PAINT painted a total of {} bytes, from {:X} to {:X}",
        (sp - addr),
        sp,
        addr
    );
}

static DATA_SIZE: usize = 56;
static BSS_SIZE: usize = 0; // for some reason, in this script, memory painting only works if BSS_SIZE is 0...
static STACK_TOP: usize = 0x2000_0000 + DATA_SIZE + BSS_SIZE + 32;

fn dummy(calls: usize) {
    let arr = [0u8; 4];
    if calls > 0 && arr[0] == 0 {
        info!(">> dummy: stack pointer: {:?}", get_sp());
        dummy(calls - 1);
    }
}

#[entry]
fn main() -> ! {
    let number_of_calls = 4;
    let _just_a_variable_easy_to_find_in_the_memory: u32 = 0xB0BA_B0BA;

    paint_ram_from_sp_to_limit(STACK_TOP);

    info!("BEGIN test.");

    info!("main: stack pointer: {:?}", get_sp());

    dummy(number_of_calls);

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
