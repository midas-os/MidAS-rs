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
use midas::memory::BootInfoFrameAllocator;
use midas::{self, hlt_loop, memory};
use x86_64::structures::paging::Page;
use x86_64::{structures::paging::{Translate, page}, VirtAddr};
use bootloader::{BootInfo, entry_point};

static OS_NAME: &str = "MidAS";
static OS_NAME_FULL: &str = "Midna Avery System";
static VERSION: &str = env!("CARGO_PKG_VERSION"); 

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

/*******************
 * Confirm OS booted
*******************/
    println!("{} ({}) v{}", OS_NAME, OS_NAME_FULL, VERSION);
    println!("Boot successful!");

/**********
 * Based stuff
**********/
    change_fg!(vga_buffer::Color::LightCyan);
    print!("T");
    change_fg!(vga_buffer::Color::Pink);
    print!("R");
    change_fg!(vga_buffer::Color::White);
    print!("A");
    change_fg!(vga_buffer::Color::Pink);
    print!("N");
    change_fg!(vga_buffer::Color::LightCyan);
    print!("S");
    change_fg!(vga_buffer::Color::White);
    println!(" Rights!");
    println!("Yeah, that's right. This OS supports trans people");
    println!("Follow @Steve12618831 on twitter. They're really cool!");

/*********************
* Paging
*********************/

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(20).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    test_main();

    println!("We didn't crash! :D");
    
 /****************************************
* hlt_loop() to keep the OS running
    as long as we need it to.
 ****************************************/
    midas::hlt_loop();
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
    midas::test_panic_handler(info);
    midas::hlt_loop();
}