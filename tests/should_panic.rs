#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(midas::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use midas::qemu::{QemuExitCode, exit_qemu};
use midas::{serial_println, serial_print};

#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    should_fail();
    loop {}
}

#[test_case]
fn test_main() {
    serial_println!("Running should_panic tests...");
    should_fail();
}

#[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }

    exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}