/**************************************************************************************************
* Name : 									  kernel.rs
* Author : 										Avery
* Date : 									  1/31/2023
* Purpose : 					      Processing & managing data
* Version : 									 0.2
**************************************************************************************************/

extern crate alloc;

use bootloader::BootInfo;
use midas::{task::{executor::Executor, keyboard, Task}, cmd, asm, vga_driver};
use crate::{memory::BootInfoFrameAllocator, println};
use x86_64::{structures::paging::OffsetPageTable, VirtAddr};

pub static OS_NAME: &str = "MidAS";
pub static OS_NAME_FULL: &str = "Midna Avery System";
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn main(
    _boot_info: &'static BootInfo,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut BootInfoFrameAllocator,
    phys_mem_offset: VirtAddr,
) {    
    println!("Boot successful!");
    asm::test_asm();
    
    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));

    /*************
    * VGA Graphics
    *************/
    unsafe { vga_driver::init(); }

    /*************
    * Command line
    *************/
    cmd::init();

    /****************************
    * Halt loop so we don't crash
    ****************************/
    executor.run();
}
