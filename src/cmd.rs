use crate::{change_bg, change_fg, print, println, vga_buffer::Color, clear_screen};
use alloc::{vec::Vec, boxed::Box, string::{String, ToString}, borrow::ToOwned};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref COMMANDS: Mutex<Vec<&'static Command>> = Mutex::new(Vec::new());
}

static mut COMMAND_LINE_ACTIVE: bool = false;
static mut COMMAND_LINE_BUFFER: [u8; 256] = [0; 256];

pub static COMMAND_PREFIX: &str = "midas> ";

#[derive(Clone, Copy)]
struct Command {
    name: &'static str,
    description: &'static str,
    function: fn(&mut String),
}

impl Command {
    pub fn new(name: &'static str, description: &'static str, function: fn(&mut String)) -> Self {
        Command {
            name,
            description,
            function,
        }
    }
}

pub fn is_active() -> bool {
    unsafe { COMMAND_LINE_ACTIVE }
}

fn add_command(command: Command) {
    let command = Box::leak(Box::new(command));
    COMMANDS.lock().push(command);
}

pub fn init() {
    add_command(Command::new("help", "Show this help message", help));
    add_command(Command::new("clear", "Clear the screen", clear));
    add_command(Command::new("echo", "Echoes the input", echo));
    add_command(Command::new("based", "Prints cool stuff", print_based));

    println!("Welcome to the command line interface!");
    print!("Type ");
    
    change_fg!(Color::LightGreen);
    print!("'help'");
    change_fg!(Color::White);

    println!(" to see a list of commands");

    unsafe {
        COMMAND_LINE_ACTIVE = true;
    }

    print!("{}", COMMAND_PREFIX);
}

pub fn uninit() {
    unsafe {
        COMMAND_LINE_ACTIVE = false;
    }
}

pub(crate) fn process_command() {
    let command_bfr = unsafe { core::str::from_utf8_unchecked(&COMMAND_LINE_BUFFER) };

    /****************************************************
    * remove all null terminators from the command buffer
    ****************************************************/
    let command_no_null = command_bfr.split('\0').collect::<Vec<&str>>()[0];

    /**********************************************
    * remove all backspaces from the command buffer
    **********************************************/
    let mut command = String::new();
    for c in command_no_null.chars() {
        if c == '\x08' {
            command.pop();
        } else {
            command.push(c);
        }
    }

    if command.is_empty() {
        return;
    }

    /************************************************
    * print new line so the following command doesn't 
        write on the same line as the input
    *************************************************/
    println!();

    // split the command into arguments and keep it mutable
    let mut args = command.split(' ').collect::<Vec<&str>>();
    
    let mut command_found = false;

    for cmd in COMMANDS.lock().iter() {
        if cmd.name == args[0] {
            (cmd.function)(&mut command.to_owned());
            command_found = true;
            break;
        }
    }

    if !command_found {
        change_fg!(Color::Red);
        
        println!("Command \"{}\" not found", args[0]);
        change_fg!(Color::White);
    }

    unsafe {
        COMMAND_LINE_BUFFER = [0; 256];
    }
}

fn help(_cmd: &mut String) {
    println!("Commands:");

    change_fg!(Color::Yellow);
    println!("The \"help\" command is broken right now. Sorry!");
    change_fg!(Color::White);

    return;

    for cmd in COMMANDS.lock().iter() {
        println!("TEST");
    }
}

fn echo(cmd: &mut String) {
    let args = cmd.split(' ').collect::<Vec<&str>>();

    if args.len() == 1 {
        println!("Usage: echo <text>");
        return;
    }

    let mut text = String::new();

    for arg in args.iter().skip(1) {
        text.push_str(arg);
        text.push(' ');
    }

    println!("{}", text);
}

fn clear(_cmd: &mut String) {
    change_bg!(Color::Black);
    change_fg!(Color::White);
    clear_screen!();
}

fn print_based(_cmd: &mut String) {
    /**************************
    * Based stuff
    * that's the entire command
    **************************/
    change_fg!(Color::LightCyan);
    print!("T");
    change_fg!(Color::Pink);
    print!("R");
    change_fg!(Color::White);
    print!("A");
    change_fg!(Color::Pink);
    print!("N");
    change_fg!(Color::LightCyan);
    print!("S");
    change_fg!(Color::White);
    println!(" Rights!");
}

pub(crate) fn add_char(key: pc_keyboard::DecodedKey) {
    let mut buffer = unsafe { &mut COMMAND_LINE_BUFFER };

    if buffer.iter().position(|&x| x == 0).unwrap() == buffer.len() - 1 {
        return;
    }

    match key {
        pc_keyboard::DecodedKey::Unicode(c) => {
            if buffer.iter().position(|&x| x == 0).unwrap() < buffer.len() - 1 {
                buffer[buffer.iter().position(|&x| x == 0).unwrap()] = c as u8;
            }
        }
        pc_keyboard::DecodedKey::RawKey(key) => {
            match key {
                pc_keyboard::KeyCode::Backspace => {
                    if buffer.iter().position(|&x| x == 0).unwrap() > 0 {
                        backspace();
                        buffer[buffer.iter().position(|&x| x == 0).unwrap() - 1] = 0;
                    }
                },
                _ => {}
            }
        }
    }
}

pub(crate) fn backspace() {
    print!("\u{8}");
}