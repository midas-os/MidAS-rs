/**************************************************************************************************
* Name : 									vga_driver.rs
* Author : 										Avery
* Date : 									  2/14/2023
* Purpose :                           VGA Driver for Graphics Mode
* Version : 									 0.1
**************************************************************************************************/


use alloc::{string::ToString, format, vec::Vec};
use midas_vga::graphics::{calculate_centered_rect, draw_rect, write_str_centered_x, draw_centered_rect};
use pc_keyboard::{DecodedKey, KeyCode};
use spin::Mutex;
use vga::{colors::{Color16}, writers::{Graphics640x480x16, GraphicsWriter, Text80x25, TextWriter}, drawing::Point};
use lazy_static::lazy_static;

use crate::{cmd::{self}, os_info};

lazy_static! {
    static ref PAGES: Mutex<Vec<Page>> = Mutex::new(Vec::new());
}

static mut CURRENT_BACKGROUND: Color16 = Color16::Black;
static mut CURRENT_INDEX: isize = 0;

static mut CURRENT_PAGE: usize = 0;

pub static BACKGROUND_COLORS: [Color16; 16] = [
    Color16::Black,
    Color16::Blue,
    Color16::Green,
    Color16::Cyan,
    Color16::Red,
    Color16::Magenta,
    Color16::Brown,
    Color16::LightGrey,
    Color16::DarkGrey,
    Color16::LightBlue,
    Color16::LightGreen,
    Color16::LightCyan,
    Color16::LightRed,
    Color16::Magenta,
    Color16::Yellow,
    Color16::White,
];

#[derive(Clone, Copy)]
pub struct Page {
    function: fn(),
}

impl Page {
    pub fn new(function: fn()) -> Self {
        Self {
            function,
        }
    }
}

pub fn add_page(page: Page) {
    PAGES.lock().push(page);
}

fn main_page() {
    let box_size = (550, 300);
    let box_start = calculate_centered_rect(box_size);
    let box_end = (box_start.0 + box_size.0, box_start.1 + box_size.1);

    let box_start_u = (box_start.0 as usize, box_start.1 as usize);
    let box_end_u = (box_end.0 as usize, box_end.1 as usize);

    draw_rect(box_start, box_size, Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_start_u.1 + 20, "MidAS Graphical User Interface (GUI)", Color16::White);

    write_str_centered_x(box_start_u, box_end_u, box_end_u.1 - 20, "Use the Left and Right Arrow keys to change the background color", Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_end_u.1 - 40, "Use the Up and Down Arrow keys to change the current page", Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_end_u.1 - 60, "Press X to exit to text", Color16::White);
}

fn device_info_page() {
    let box_size = (550, 300);
    let box_start = calculate_centered_rect(box_size);
    let box_end = (box_start.0 + box_size.0, box_start.1 + box_size.1);

    let box_start_u = (box_start.0 as usize, box_start.1 as usize);
    let box_end_u = (box_end.0 as usize, box_end.1 as usize);

    draw_centered_rect((550, 400), Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_start_u.1 + 20, "Device Information", Color16::White);

    let device_name = format!("Device Name: {}", cmd::DEVICE_NAME.lock().as_str());
    let architecture = format!("Architecture: x86_64");

    let os_version = format!("OS Version: {}", os_info::VERSION);
    let kernel_version = format!("Kernel Version: {}", os_info::KERNEL_VERSION);
    let gui_version = format!("GUI Version: {}", os_info::GUI_VERSION);

    write_str_centered_x(box_start_u, box_end_u, box_start_u.1 + 60, device_name.as_str(), Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_start_u.1 + 80, architecture.as_str(), Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_start_u.1 + 100, os_version.as_str(), Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_start_u.1 + 120, kernel_version.as_str(), Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_start_u.1 + 140, gui_version.as_str(), Color16::White);

    write_str_centered_x(box_start_u, box_end_u, box_end_u.1 - 20, "Use the Left and Right Arrow keys to change the background color", Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_end_u.1 - 40, "Use the Up and Down Arrow keys to change the current page", Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_end_u.1 - 60, "Press X to exit to text", Color16::White);
}

pub unsafe fn init() {
    add_page(Page::new(main_page));
    add_page(Page::new(device_info_page));
    CURRENT_PAGE = 0;
}

pub fn start() {
    unsafe {
        CURRENT_BACKGROUND = BACKGROUND_COLORS[CURRENT_INDEX as usize];

        let mode = Graphics640x480x16::new();
        mode.set_mode();
        mode.clear_screen(CURRENT_BACKGROUND);
    }

    load_current_page();
}

pub fn load_current_page() {
    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(unsafe { CURRENT_BACKGROUND });

    let page = PAGES.lock()[unsafe { CURRENT_PAGE }].function;
    page();
}

pub fn register_key(key: DecodedKey) {
    match key {
        DecodedKey::RawKey(c) => {
            match c {
                KeyCode::ArrowLeft => {
                    unsafe {
                        CURRENT_INDEX -= 1;
                        if CURRENT_INDEX < 0 {
                            CURRENT_INDEX = (BACKGROUND_COLORS.len() - 1) as isize;
                        }
                    }
                    
                    start();
                },
                KeyCode::ArrowRight => {
                    unsafe {
                        CURRENT_INDEX += 1;
                        if CURRENT_INDEX >= BACKGROUND_COLORS.len() as isize {
                            CURRENT_INDEX = 0;
                        }
                    }
                    
                    start();
                },
                KeyCode::ArrowUp => {
                    unsafe {
                        if CURRENT_PAGE == 0 {
                            CURRENT_PAGE = PAGES.lock().len();
                        }
                        CURRENT_PAGE -= 1;
                    }
                    
                    load_current_page();
                },
                KeyCode::ArrowDown => {
                    unsafe {
                        CURRENT_PAGE += 1;
                        if CURRENT_PAGE >= PAGES.lock().len() {
                            CURRENT_PAGE = 0;
                        }
                    }
                    
                    load_current_page();
                },
                _ => {}
            }
        },
        DecodedKey::Unicode(c) => {
            let lower = c.to_lowercase().to_string();
            
            if lower == "x" {
                let text_mode = Text80x25::new();

                text_mode.set_mode();
                text_mode.clear_screen();
                
                cmd::show_intro();
        
                return;
            }

            if lower == "c" {
                let mode = Graphics640x480x16::new();
                mode.set_mode();

                unsafe {
                    CURRENT_INDEX += 1;
                    if CURRENT_INDEX >= BACKGROUND_COLORS.len() as isize {
                        CURRENT_INDEX = 0;
                    }
                }
                
                start();
            }
        },
    }
}