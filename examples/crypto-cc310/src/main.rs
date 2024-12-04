#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::debug::{self, EXIT_SUCCESS};
use defmt::info;
use defmt_rtt as _;
use panic_semihosting as _;

pub use nrf52840_pac as pac;

const SHA256_DIGEST_LEN: usize = 32;

const MESSAGE_LEN: usize = 128;
#[no_mangle]
#[link_section = ".data"]
// static MESSAGE: [u8; MESSAGE_LEN] = [0xfa; MESSAGE_LEN];
static MESSAGE: [u8; 3] = *b"abc";

#[entry]
fn main() -> ! {
    info!("Running.");

    info!("Message: {:?}", MESSAGE);

    let p = pac::Peripherals::take().unwrap();
    let cc_hash = p.cc_hash;
    let cc_host_rgf = p.cc_host_rgf;
    let cc_ctl = p.cc_ctl;
    let cc_misc = p.cc_misc;
    let cc_din = p.cc_din;

    // Enable CRYPTOCELL and the necessary clocks
    p.cryptocell.enable().write(|w| w.enable().set_bit());

    cc_misc.hash_clk().write(|w| w.enable().set_bit());
    cc_misc.dma_clk().write(|w| w.enable().set_bit());

    info!("CRYPTOCELL enabled.");

    // Wait for the HASH engine to be idle
    while cc_ctl.hash_busy().read().bits() != 0 {}

    // Clear all pending interrupts
    cc_host_rgf.icr().write(|w| unsafe { w.bits(0xFFFFFFFF) });

    // Configure HASH as cryptographic flow
    cc_ctl.crypto_ctl().write(|w| w.mode().hash_active());

    // Set the HASH mode to SHA256
    cc_hash.hash_control().write(|w| w.mode().sha256());

    // Enable automatic HW padding
    cc_hash.hash_pad().write(|w| w.enable().set_bit());
    cc_hash.hash_pad_auto().write(|w| w.hwpad().enable());

    // Write the initial values for SHA256
    cc_hash.hash_h(7).write(|w| unsafe { w.bits(0x5BE0CD19) });
    cc_hash.hash_h(6).write(|w| unsafe { w.bits(0x1F83D9AB) });
    cc_hash.hash_h(5).write(|w| unsafe { w.bits(0x9B05688C) });
    cc_hash.hash_h(4).write(|w| unsafe { w.bits(0x510E527F) });
    cc_hash.hash_h(3).write(|w| unsafe { w.bits(0xA54FF53A) });
    cc_hash.hash_h(2).write(|w| unsafe { w.bits(0x3C6EF372) });
    cc_hash.hash_h(1).write(|w| unsafe { w.bits(0xBB67AE85) });
    cc_hash.hash_h(0).write(|w| unsafe { w.bits(0x6A09E667) });

    // Set the input source for DMA
    cc_din
        .src_mem_addr()
        .write(|w| unsafe { w.addr().bits(MESSAGE.as_ptr() as u32) });
    cc_din
        .src_mem_size()
        .write(|w| unsafe { w.bits(MESSAGE.len() as u32) });

    while cc_host_rgf.irr().read().mem_to_din_int().bit_is_clear() {}

    while cc_ctl.hash_busy().read().status().is_busy() {}

    // Read the resulting hash from HASH_H registers
    let mut hash_regs = [0u32; 8];
    for (i, reg) in hash_regs.iter_mut().enumerate() {
        *reg = cc_hash.hash_h(i).read().bits();
    }

    let hash_bytes = convert_array(&hash_regs);
    info!("Hash: {:02x}", hash_bytes);

    info!("Done.");

    // exit via semihosting call
    debug::exit(EXIT_SUCCESS);
    loop {}
}

fn convert_array(input: &[u32]) -> [u8; SHA256_DIGEST_LEN] {
    assert!(input.len() == SHA256_DIGEST_LEN / 4);

    let mut output = [0x00u8; SHA256_DIGEST_LEN];
    for i in 0..SHA256_DIGEST_LEN / 4 {
        output[4 * i..4 * i + 4].copy_from_slice(&input[i].to_be_bytes());
    }
    output
}
