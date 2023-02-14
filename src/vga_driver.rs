use vga::{colors::Color16, writers::{Graphics640x480x16, GraphicsWriter}, drawing::Point};

use crate::vga_buffer::Color;

pub fn init() {
    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Blue);

    let box_start: (usize, usize) = (80, 60);
    let box_size: (isize, isize) = (460, 360);
    let box_end: (usize, usize) = (box_start.0 + box_size.0 as usize, box_start.1 + box_size.1 as usize);

    draw_rect(box_start.0 as isize, box_start.1 as isize, box_size.0, box_size.1, Color16::White);
    write_str_centered_x(box_start, box_end, 70, "MidAS Graphical User Interface (GUI)", Color16::White);
    write_str_centered_x(box_start, box_end, 100, "Version 0.0.1", Color16::White);

    write_str_centered(box_start, box_end, "Work In Progress!", Color16::LightRed);
}

pub fn draw_rect(x: isize, y: isize, width: isize, height: isize, color: Color16) {
    let mode = Graphics640x480x16::new();
    mode.set_mode();

    mode.draw_line((x, y), (x, y + height), color);
    mode.draw_line((x, y), (x + width, y), color);
    mode.draw_line((x + width, y), (x + width, y + height), color);
    mode.draw_line((x, y + height), (x + width, y + height), color);
}

pub fn write_string(start: Point<usize>, string: &str, color: Color16) {
    let mode = Graphics640x480x16::new();

    for (offset, character) in string.chars().enumerate() {
        mode.draw_character(start.0 + offset * 8, start.1, character, color);
    }
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