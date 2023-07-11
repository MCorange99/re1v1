#![no_main]
#![no_std]

mod std;
use core::panic::PanicInfo;


#[no_mangle]
extern "C" fn kmain() -> ! {
    println!("EPIK! IT WORKS");

    loop {}
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}