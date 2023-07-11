#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

mod std;
mod interrupts;
mod gdt;
use core::panic::PanicInfo;

fn init() {
    interrupts::init_idt();
    gdt::init();
}


#[no_mangle]
extern "C" fn kmain() -> ! {
    init();
    println!("EPIK! IT WORKS");

    loop {}
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}