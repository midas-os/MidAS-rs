extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use bootloader::BootInfo;
use midas::{println, allocator};
use midas::memory::{self, BootInfoFrameAllocator};
use x86_64::structures::paging::OffsetPageTable;
use x86_64::{VirtAddr, structures::paging::Page};

pub fn post_boot_sqc(boot_info: &'static BootInfo, mapper: &mut OffsetPageTable, frame_allocator: &mut BootInfoFrameAllocator, phys_mem_offset: VirtAddr) {
    println!("Booted into kernel.rs::post_boot_sqc()");

/*********************
* Paging
*********************/
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, mapper, frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(0x14).write_volatile(0x_f021_f077_f065_f04e)};

/****************************************
* hlt_loop() to keep the OS running
    as long as we need it to.
****************************************/
    midas::hlt_loop();
}