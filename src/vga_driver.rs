/**************************************************************************************************
* Name : 									vga_driver.rs
* Author : 										Avery
* Date : 									  2/14/2023
* Purpose :                           VGA Driver for Graphics Mode
* Version : 									 0.1
**************************************************************************************************/


use alloc::{string::ToString, format, vec::Vec};
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
    add_page(Page::new(my_page));
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

pub fn draw_rect(position: Point<isize>, size: Point<isize>, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    mode.draw_line((position.0, position.1), (position.0, position.1 + size.1), color);
    mode.draw_line((position.0, position.1), (position.0 + size.0, position.1), color);
    mode.draw_line((position.0 + size.0, position.1), (position.0 + size.0, position.1 + size.1), color);
    mode.draw_line((position.0, position.1 + size.1), (position.0 + size.0, position.1 + size.1), color);
}

pub fn write_string(start: Point<usize>, string: &str, color: Color16) {
    let mode = Graphics640x480x16::new();
    let mut y_offset = 0;
    let mut x_offset = 0;

    for (offset, character) in string.chars().enumerate() {
        if character == '\n' {
            x_offset = 0;
            y_offset += 1;
            continue;
        }

        x_offset += 1;
        mode.draw_character(start.0 + x_offset * 8, start.1 + (y_offset * 10), character, color);
    }
}

pub fn draw_centered_rect(size: Point<isize>, color: Color16) {
    let x = (640 - size.0) / 2;
    let y = (480 - size.1) / 2;

    draw_rect((x, y), size, color);
}

pub fn calculate_centered_rect(size: Point<isize>) -> Point<isize> {
    let x = (640 - size.0) / 2;
    let y = (480 - size.1) / 2;

    (x, y)
}

pub fn write_str_centered(bounds_start: Point<usize>, bounds_end: Point<usize>, string: &str, color: Color16) {
    let lines = string.split('\n');

    for (offset, line) in lines.enumerate() {
        let string_width = line.len() * 8;
        let string_height = 16;

        let x = (bounds_start.0 + bounds_end.0) / 2 - string_width / 2;
        let y = (bounds_start.1 + bounds_end.1) / 2 - string_height / 2;

        write_string((x, y + (offset * 10)), line, color);
    }
}

pub fn write_str_centered_x(bounds_start: Point<usize>, bounds_end: Point<usize>, y: usize, string: &str, color: Color16) {
    // split string into new lines
    let lines = string.split('\n');

    for (offset, line) in lines.enumerate() {
        let string_width = line.len() * 8;

        let x = (bounds_start.0 + bounds_end.0) / 2 - string_width / 2;

        write_string((x, y + (offset * 10)), line, color);
    }
}

pub fn write_str_centered_y(bounds_start: Point<usize>, bounds_end: Point<usize>, x: usize, string: &str, color: Color16) {
    let string_height = 16;

    let y = (bounds_start.1 + bounds_end.1) / 2 - string_height / 2;

    write_string((x, y), string, color);
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