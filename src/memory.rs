/**************************************************************************************************
* Name : 									  memory.rs
* Author : 										Avery
* Date : 									  1/29/2023
* Purpose :                       Code for paging & stack allocations
* Version : 									 0.1
**************************************************************************************************/

use x86_64::{
    PhysAddr,
    VirtAddr,
    structures::paging::{PageTable, OffsetPageTable, Size4KiB, page_table::FrameError, FrameAllocator, Page, PageTableFlags as Flags, PhysFrame, Mapper, frame},
    registers::control::Cr3,
};
use bootloader::bootinfo::{MemoryRegionType, MemoryMap};

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }

    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr)
    -> Option<PhysAddr>
{
    translate_addr_inner(addr, physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr)
    -> Option<PhysAddr>
{
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];
    let mut frame = level_4_table_frame;

    for &index in &table_indexes {
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
)
{
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // TODO: replace. this is only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

/***************************************
* functions for easy allocation / paging
***************************************/
pub fn create_page(virt_addr: u64, mapper: &mut OffsetPageTable, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Page {
    let page = Page::containing_address(VirtAddr::new(virt_addr));
    create_example_mapping(page, mapper, frame_allocator);

    page
}

pub fn get_page_ptr(page: Page) -> *mut u64 {
    page.start_address().as_mut_ptr()
}

pub fn write_page(page: Page, offset: isize, data: u8) {
    let ptr: *mut u64 = get_page_ptr(page);
    write_page_ptr(ptr, offset, data)
}

pub fn write_page_ptr(ptr: *mut u64, offset: isize, data: u8) {
    unsafe {
        ptr.offset(offset).write_bytes(data, data as usize);
    }
}

pub fn write_page_vol(page: Page, offset: isize, data: u64) {
    let ptr: *mut u64 = get_page_ptr(page);
    write_page_vol_ptr(ptr, offset, data)
}

pub fn write_page_vol_ptr(ptr: *mut u64, offset: isize, data: u64) {
    unsafe {
        ptr.offset(offset).write_volatile(data);
    }
}