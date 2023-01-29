/**************************************************************************************************
* Name : 									   main.rs
* Author : 										Avery
* Date : 									  1/28/2023
* Purpose : 					   Driver for operating system code
* Version : 									 0.1
**************************************************************************************************/

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(midas::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod qemu;
mod serial;

use core::panic::PanicInfo;
use midas;

static OS_NAME: &str = "MidAS";
static OS_NAME_FULL: &str = "Midna Avery System";

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

<<<<<<< HEAD
=======
fn test_colors(background: i8) {
    serial_println!("Running colors tests with mode {}", background);
    
    for i in 0..16 {

    /****************************************
     * Test Foreground                      
     ****************************************/
        if background == 0 {
            change_fg!(vga_buffer::Color::from_u32(i));

            println!("Foreground Color Test!");
            continue;
        }

     /****************************************
     * Test Background                      
     ****************************************/
        change_bg!(vga_buffer::Color::from_u32(i));

       println!("Background Color Test! (Ignore the weird bugs)");   
    }

    change_color!(vga_buffer::Color::White, vga_buffer::Color::Black);
    println!("");
}

>>>>>>> d30e5590acc222bed472dd51b62cba42e6884d11
fn _start_tests() {
    serial_print!("trivial assertion...");
    assert_eq!(1, 1);
    serial_println!("[ok]");

    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    midas::init();
    
    let usr = "Admin";
    print!("Hey, ");
    change_fg!(vga_buffer::Color::LightGreen);
    print!("{}", usr);
    change_fg!(vga_buffer::Color::White);
    println!(".");
    
    print!("Welcome to ");
    // delay!(500);
    change_fg!(vga_buffer::Color::LightBlue);
    print!("Mid");
    change_fg!(vga_buffer::Color::LightRed);
    print!("A");
    change_fg!(vga_buffer::Color::Yellow);
    println!("S");
    change_fg!(vga_buffer::Color::White);

    x86_64::instructions::interrupts::int3();
    
    #[cfg(test)]
    test_main();

/****************************************
* Command line without any input handling                      
****************************************/
    change_fg!(vga_buffer::Color::LightGreen);
    print!("{}", usr);
    change_fg!(vga_buffer::Color::White);
    print!("@");
    change_fg!(vga_buffer::Color::LightRed);
    print!("qemu");
    change_fg!(vga_buffer::Color::White);
    print!("> ");
       
 /****************************************
 * Infinite loop, to keep the operating
 	system from stopping after 5ms
 ****************************************/
    loop {}
}

/****************************************
 * Funtion called whenever the operating
 	system panics
****************************************/
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
