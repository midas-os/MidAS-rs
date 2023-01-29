#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to MAverROS!");
    
    // Tests
    println!("Test Numbers:");
    println!("Natural: {}", 10);
    println!("Decimal: {}", 3.141592653589);

    // Infinite loop so the OS doesn't shut down after like 5ms
    loop {}
}

// This function is called whenever the system panics.

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}