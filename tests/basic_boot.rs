#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(midas::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[test_case]
fn test_println() {
    midas::println!("test_println output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    midas::test_panic_handler(info)
}