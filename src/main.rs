#![no_std]
#![no_main]

// Use some no_std entry
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut y: f32 = f32::min(2., 1.);
    // Do some loopy stuff to ensure code is not const-evaluated.
    loop {
        y += y * 5.1 + f32::min(2., y);
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
