/**************************************************************************************************
* Name : 									  kernel.rs
* Author : 										Avery
* Date : 									  1/31/2023
* Purpose : 					      Processing & managing data
* Version : 									 0.1
**************************************************************************************************/

extern crate alloc;

use bootloader::BootInfo;
use midas::{
    println,
    task::{Task, keyboard, simple_executor::SimpleExecutor, executor::Executor},
    memory::{self, BootInfoFrameAllocator}
};
use x86_64::{structures::paging::{OffsetPageTable, Page}, VirtAddr};

async fn example_number() -> u8 {
    174
}

async fn example_task() {
    let num = example_number().await;
    println!("Example Number: {}", num);
}

pub fn post_boot_sqc(_boot_info: &'static BootInfo, mapper: &mut OffsetPageTable, frame_allocator: &mut BootInfoFrameAllocator, phys_mem_offset: VirtAddr) {
    println!("Booted into kernel.rs::post_boot_sqc()");

/*********************
* Paging
*********************/
    let page = memory::create_page(0, mapper, frame_allocator);
    memory::write_page_vol(page, 0x14, 0x_f021_f077_f065_f04e);

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}