use x86_64::structures::idt::InterruptStackFrame;

use crate::println;


pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {

    println!("EXEPTION: BREAKPOINT >\n{:#?}", stack_frame);

}