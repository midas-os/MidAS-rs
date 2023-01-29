#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

static OS_NAME: &str = "Midas";

fn test_color(background: i8) {
    for i in 0..16 {

        // Test Foreground
        if background == 0 {
            change_color!(vga_buffer::Color::from_u32(i),
                vga_buffer::Color::Black);

            println!("Foreground Color Test!");
            continue;
        }

        // Test Background
        change_color!(vga_buffer::Color::White,
            vga_buffer::Color::from_u32(i));

        println!("Background Color Test! (Ignore the weird bugs)");   
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to {}!", OS_NAME);
    
    // Tests
    println!("Test Numbers:");
    println!("Natural: {}", 10);
    println!("Decimal: {}", 3.141592653589);

    test_color(0);
    print!("--------");
    test_color(1);
        
    // Infinite loop so the OS doesn't shut down after like 5ms
    loop {}
}

// This function is called whenever the system panics.

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    change_color!(vga_buffer::Color::Red, vga_buffer::Color::Black);
    println!("{}", info);
    change_color!(vga_buffer::Color::White, vga_buffer::Color::White);
    loop {}
}