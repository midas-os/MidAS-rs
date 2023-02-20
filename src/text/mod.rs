/**************************************************************************************************
* Name : 								     text/mod.rs
* Author : 										Avery
* Date : 									  2/20/2023
* Purpose :                              Text Writer for VGA
* Version : 									 0.1
**************************************************************************************************/

use core::fmt::{self, Write};

use alloc::{string::{ToString, String}, format, vec::Vec};
use midas_vga::text::{write_string, write_char};
use vga::{colors::{TextModeColor, Color16}, writers::{Text80x25, TextWriter, ScreenCharacter}};
use spin::Mutex;
use lazy_static::lazy_static;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { 
        position: 0,
        line: 0,
        color: TextModeColor::new(Color16::White, Color16::Black),
        mode: Writer::get_mode(),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub struct Writer {
    position: usize,
    line: usize,
    color: TextModeColor,
    mode: Text80x25,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub const fn get_mode() -> Text80x25 {
        Text80x25::new()
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let screen_character = ScreenCharacter::new(byte, self.color);
                self.buffer.chars[self.line][self.position].write(screen_character);
                self.position += 1;
            }
        }

        self.update_cursor();
    }

    pub fn write_string(&mut self, message: &str) {
        for byte in message.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // backspace
                b'\x08' => self.backspace(),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn new_line(&mut self) {
        if self.line >= BUFFER_HEIGHT - 1 {
            // scroll up
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        } else {
            self.line += 1;
        }
        self.position = 0;

        self.update_cursor();
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenCharacter::new(b' ', self.color);
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn backspace(&mut self) {
        if self.position == 0 {
            return;
        }

        self.position -= 1;
        write_char(ScreenCharacter::new(b' ', self.color), self.position, self.line);
        self.update_cursor();
    }

    pub fn change_color(&mut self, fg: Color16, bg: Color16) {
        self.change_foreground(fg);
        self.change_background(bg);
    }

    pub fn change_foreground(&mut self, color: Color16) {
        self.color.set_foreground(color);
    }

    pub fn change_background(&mut self, color: Color16) {
        self.color.set_background(color);
    }

    pub fn clear_screen(&mut self) {
        let mode = Self::get_mode();
        mode.clear_screen();

        self.position = 0;
        self.line = 0;
    }

    pub fn update_cursor(&mut self) {
        let mode = Self::get_mode();
        mode.set_cursor_position(self.position, self.line);
    }

}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::text::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! change_fg {
    ($fg:expr) => {
        $crate::text::_change_fg($fg);
    };
}

#[macro_export]
macro_rules! change_bg {
    ($bg:expr) => {
        $crate::text::_change_bg($bg);
    };
}

#[macro_export]
macro_rules! change_color {
    ($fg:expr, $bg:expr) => {
        $crate::text::_change_color($fg, $bg);
    };
}

pub fn _change_fg(fg: Color16) {
    WRITER.lock().change_foreground(fg);
}

pub fn _change_bg(bg: Color16) {
    WRITER.lock().change_background(bg);
}

pub fn _change_color(fg: Color16, bg: Color16) {
    WRITER.lock().change_color(fg, bg);
}

#[macro_export]
macro_rules! clear_screen {
    () => {
        $crate::text::_clear_screen();
    };
}

pub fn _clear_screen() {
    WRITER.lock().clear_screen();
}

#[macro_export]
macro_rules! update_cursor {
    () => {
        $crate::text::_update_cursor();
    };
}

pub fn _update_cursor() {
    WRITER.lock().update_cursor();
}