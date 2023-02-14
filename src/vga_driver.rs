use core::borrow::Borrow;

use alloc::{borrow::ToOwned, string::ToString};
use pc_keyboard::{DecodedKey, KeyCode};
use vga::{colors::{Color16, TextModeColor}, writers::{Graphics640x480x16, GraphicsWriter, Text80x25, TextWriter}, drawing::Point, registers::{ColorPaletteRegisters, CrtcControllerRegisters}};

use crate::{cmd, task::keyboard};

static mut CURRENT_BACKGROUND: Color16 = Color16::Black;
static mut CURRENT_INDEX: usize = 0;

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

pub fn init() {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    unsafe {
        CURRENT_BACKGROUND = BACKGROUND_COLORS[CURRENT_INDEX];
        mode.clear_screen(CURRENT_BACKGROUND);
    }

    let box_size = (500, 300);
    let box_start = calculate_centered_rect(box_size);
    let box_end = (box_start.0 + box_size.0, box_start.1 + box_size.1);

    let box_start_u = (box_start.0 as usize, box_start.1 as usize);
    let box_end_u = (box_end.0 as usize, box_end.1 as usize);

    draw_rect(box_start, box_size, Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_start_u.1 + 20, "MidAS Graphical User Interface (GUI)", Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_start_u.1 + 40, "Version 0.0.1", Color16::White);

    write_str_centered_x(box_start_u, box_end_u, box_end_u.1 - 20, "Press C to change background color", Color16::White);
    write_str_centered_x(box_start_u, box_end_u, box_end_u.1 - 40, "Press X to exit to text", Color16::White);
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

    for (offset, character) in string.chars().enumerate() {
        mode.draw_character(start.0 + offset * 8, start.1, character, color);
    }
}

pub fn draw_centered_rect(size: Point<isize>, color: Color16) {
    let mode = Graphics640x480x16::new();

    let x = (640 - size.0) / 2;
    let y = (480 - size.1) / 2;

    draw_rect((x, y), size, color);
}

pub fn calculate_centered_rect(size: Point<isize>) -> Point<isize> {
    let mode = Graphics640x480x16::new();

    let x = (640 - size.0) / 2;
    let y = (480 - size.1) / 2;

    (x, y)
}

pub fn write_str_centered(bounds_start: Point<usize>, bounds_end: Point<usize>, string: &str, color: Color16) {
    let mode = Graphics640x480x16::new();

    let string_width = string.len() * 8;
    let string_height = 16;

    let x = (bounds_start.0 + bounds_end.0) / 2 - string_width / 2;
    let y = (bounds_start.1 + bounds_end.1) / 2 - string_height / 2;

    write_string((x, y), string, color);
}

pub fn write_str_centered_x(bounds_start: Point<usize>, bounds_end: Point<usize>, y: usize, string: &str, color: Color16) {
    let mode = Graphics640x480x16::new();

    let string_width = string.len() * 8;
    let string_height = 16;

    let x = (bounds_start.0 + bounds_end.0) / 2 - string_width / 2;

    write_string((x, y), string, color);
}

pub fn write_str_centered_y(bounds_start: Point<usize>, bounds_end: Point<usize>, x: usize, string: &str, color: Color16) {
    let mode = Graphics640x480x16::new();

    let string_width = string.len() * 8;
    let string_height = 16;

    let y = (bounds_start.1 + bounds_end.1) / 2 - string_height / 2;

    write_string((x, y), string, color);
}

pub fn register_key(key: DecodedKey) {
    match key {
        DecodedKey::RawKey(c) => {
        },
        DecodedKey::Unicode(c) => {
            let lower = c.to_lowercase().to_string();
            
            if lower == "x" {
                let text_mode = Text80x25::new();
                let color = TextModeColor::new(Color16::White, Color16::Black);
        
                text_mode.set_mode();
                text_mode.clear_screen();
                
                cmd::init();
        
                return;
            }

            if lower == "c" {
                let mode = Graphics640x480x16::new();
                mode.set_mode();

                unsafe {
                    CURRENT_INDEX += 1;
                    if CURRENT_INDEX >= BACKGROUND_COLORS.len() {
                        CURRENT_INDEX = 0;
                    }
                }
                
                init();
            }
        },
    }
}