/**************************************************************************************************
* Name : 									   main.rs
* Author : 										Avery
* Date : 									  1/28/2023
* Purpose : 					            Setup & tests
* Version : 									 0.2
**************************************************************************************************/

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(midas::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod qemu;
mod serial;
mod kernel;

use core::{panic::PanicInfo, fmt::Write};
use midas::{memory::{BootInfoFrameAllocator, self}, self, allocator, text::WRITER};
use midas::{println,change_color};
use vga::colors::Color16;
use x86_64::{VirtAddr};
use bootloader::{BootInfo, entry_point};

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

fn _start_tests() {
    serial_print!("trivial assertion...");
    assert_eq!(1, 1);
    serial_println!("[ok]");

    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    midas::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    kernel::main(boot_info, &mut mapper, &mut frame_allocator, phys_mem_offset);

    midas::hlt_loop()
}

/****************************************
 * Funtion called whenever the operating
 	system panics
****************************************/
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    change_color!(Color16::Red, Color16::Black);
    println!("{}", info);
    change_color!(Color16::White, Color16::White);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    midas::test_panic_handler(info);
    midas::hlt_loop();
}