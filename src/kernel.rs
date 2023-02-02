/**************************************************************************************************
* Name : 									  kernel.rs
* Author : 										Avery
* Date : 									  1/31/2023
* Purpose : 					      Processing & managing data
* Version : 									 0.2
**************************************************************************************************/

extern crate alloc;

use bootloader::BootInfo;
use midas::{
    change_bg, change_color, change_fg,
    memory::{self, BootInfoFrameAllocator},
    cmd,
    print, println,
    task::{executor::Executor, keyboard, Task},
    vga_buffer,
};
use x86_64::{
    structures::paging::{OffsetPageTable, Page},
    VirtAddr,
};

async fn async_string() -> &'static str {
    "Hello from async_string()!"
}

async fn example_task() {
    let future = async_string();
    let s = future.await;

    println!("{}", s);
}

pub fn post_boot_sqc(
    _boot_info: &'static BootInfo,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut BootInfoFrameAllocator,
    phys_mem_offset: VirtAddr,
) {
    println!("Booted into kernel.rs::post_boot_sqc()");

    let mut executor = Executor::new();
    //executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));

    /*************
    * Command line
    *************/
    cmd::init();

    /****************************
    * Halt loop so we don't crash
    ****************************/
    executor.run();
}
