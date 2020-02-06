#![no_std]
#![no_main]

use core::panic::PanicInfo;


//#[cfg(feature = "wasm")]
#[no_mangle]
pub extern "C" fn main() {}

//#[cfg(feature = "wasm")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}