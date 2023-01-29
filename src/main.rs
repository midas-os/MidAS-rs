#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(midas::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod qemu;
mod serial;

use core::panic::PanicInfo;
use midas_os;

static OS_NAME: &str = "Midas";

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

fn test_colors(background: i8) {
    serial_println!("Running colors tests with mode {}", background);
    
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

    change_color!(vga_buffer::Color::White, vga_buffer::Color::Black);
    println!("");
}

fn _start_tests() {
    serial_print!("trivial assertion...");
    assert_eq!(1, 1);
    serial_println!("[ok]");

    serial_print!("Testing colors:\n\n");
    test_colors(0);
    serial_println!("-----------");
    test_colors(1);

    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Welcome to ");
    // delay!(500);
    change_color!(vga_buffer::Color::LightBlue, vga_buffer::Color::Black);
    print!("Mid");
    change_color!(vga_buffer::Color::LightRed, vga_buffer::Color::Black);
    print!("As");
    change_color!(vga_buffer::Color::Yellow, vga_buffer::Color::Black);
    println!("OS");

    #[cfg(test)]
    test_main();
    
    // Infinite loop so the OS doesn't shut down after like 5ms
    loop {}
}

// This function is called whenever the system panics.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    change_color!(vga_buffer::Color::Red, vga_buffer::Color::Black);
    println!("{}", info);
    change_color!(vga_buffer::Color::White, vga_buffer::Color::White);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    midas::test_panic_handler(info)
}