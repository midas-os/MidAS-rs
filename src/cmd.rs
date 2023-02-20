/**************************************************************************************************
* Name : 									    cmd.rs
* Author : 									Avery & Midna
* Date : 									  2/02/2023
* Purpose :                       Command line interface for MidAS
* Version : 									 0.1
**************************************************************************************************/

use crate::{change_bg, change_fg, print, println, clear_screen, os_info::{self, OS_NAME}, task::{self, keyboard}, application::Application, vga_driver, asm, random};
use vga::colors::Color16;
use alloc::{vec::Vec, boxed::Box, string::{String, ToString}};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref COMMANDS: Mutex<Vec<&'static Command>> = Mutex::new(Vec::new());
}

static mut COMMAND_LINE_ACTIVE: bool = false;
static mut COMMAND_LINE_BUFFER: [u8; 512] = [0; 512];
static mut CURRENT_INDEX: u16 = 0;

lazy_static! {
    pub static ref DEVICE_NAME: Mutex<String> = Mutex::new("qemu".to_string());
}

pub fn get_command_prefix() -> String {
    let mut prefix = String::new();
    prefix.push_str(DEVICE_NAME.lock().as_str());
    prefix.push_str("@midas> ");

    // unsafe { DEVICE_NAME.force_unlock(); }
    
    prefix
} 

#[derive(Clone, Copy)]
pub struct Command {
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

pub fn add_command(command: Command) {
    let command = Box::leak(Box::new(command));
    COMMANDS.lock().push(command);
}

pub fn show_intro(is_command: bool) {
    clear_screen!();

    /**********************
    * print welcome message
    * ASCII Art from https://patorjk.com/software/taag/#p=display&f=Roman
    *********************/
    print!("\nWelcome to \n");
    print_colored(r#"
ooo        ooooo  o8o        .o8        .o.        .oooooo..o 
`88.       .888'  `"'       "888       .888.      d8P'    `Y8 
 888b     d'888  oooo   .oooo888      .8"888.     Y88bo.      
 8 Y88. .P  888  `888  d88' `888     .8' `888.     `"Y8888o.  
 8  `888'   888   888  888   888    .88ooo8888.        `"Y88b 
 8    Y     888   888  888   888   .8'     `888.  oo     .d8P 
o8o        o888o o888o `Y8bod88P" o88o     o8888o 8""88888P'

"#, Color16::Yellow);

    print!("Type ");

    print_colored("\"help\"", Color16::LightGreen);

    println!(" to see a list of commands");

    unsafe {
        COMMAND_LINE_ACTIVE = true;
        keyboard::INPUT_TARGET = keyboard::InputTarget::Terminal;
    }

    if is_command {
        return;
    }
    
    print!("{}", get_command_prefix());
    
} 

pub fn init() {
    /***************************************************
    * add commands to command list so they can be called
    ***************************************************/
    add_command(Command::new("help", "Show this help message", help));
    add_command(Command::new("midas", "Shows the MidAS initialization screen.", cmd_show_intro));
    add_command(Command::new("clear", "Clear the screen", clear));
    add_command(Command::new("echo", "Echoes the input", echo));
    add_command(Command::new("based", "Prints cool stuff", print_based));
    add_command(Command::new("version", "Shows current Version", version_info));
    add_command(Command::new("rdvc", "Lets you change the name of the current device", rename_device));
    add_command(Command::new("credits", "Shows who worked on the OS!", credits));
    add_command(Command::new("vga", "Enables VGA Graphics Mode", vga_graphics));
    add_command(Command::new("tfrst", "Triple Fault Reset (MAY CORRUPT HARDWARE)", triple_fault_reset));
    add_command(Command::new("rnd", "Generates a random number", generate_rnd));
    add_command(Command::new("rndrg", "Generates a random number in a range", generate_rnd_range));
    
    show_intro(false);
}

pub fn uninit() {
    unsafe {
        COMMAND_LINE_ACTIVE = false;
        task::keyboard::INPUT_TARGET = task::keyboard::InputTarget::None;
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
            // cmd.function(&'static mut args);
            let mut cmd_str = String::new();
            for arg in args.iter().skip(1) {
                cmd_str.push_str(arg);
                cmd_str.push(' ');
            }

            (cmd.function)(&mut cmd_str);
            command_found = true;
            break;
        }
    }

    if !command_found {
        change_fg!(Color16::Red);
        
        println!("Command \"{}\" not found", args[0]);
        change_fg!(Color16::White);
    }

    /******************************
    * reset the command line buffer
    ******************************/
    unsafe {
        COMMAND_LINE_BUFFER = [0; 512];
        CURRENT_INDEX = 0;
    }
}

fn help(_cmd: &mut String) {
    unsafe {
        COMMANDS.force_unlock();
    }
    
    println!("Commands:");

    for cmd in COMMANDS.lock().iter() {
        println!("{} - {}", cmd.name, cmd.description);
    }
}

fn cmd_show_intro(_cmd: &mut String) {
    show_intro(true);
}

fn generate_rnd(_cmd: &mut String) {
    println!("Random number (0, 1): {}", random::generate_rnd_01());
    println!("Random number: {}", random::generate_rnd());
}

fn generate_rnd_range(cmd: &mut String) {
    let args = cmd.split(' ').collect::<Vec<&str>>();

    if args.len() <= 2 {
        println!("Usage: rndrg <min> <max>");
        return;
    }

    let min = args[0].parse::<u64>().unwrap();
    let max = args[1].parse::<u64>().unwrap();

    println!("Random number in range ({}, {}): {}", min, max, random::generate_rnd_rng(min, max));
}

fn rename_device(cmd: &mut String) {
    let args = cmd.split(' ').collect::<Vec<&str>>();

    if args.len() == 1 {
        println!("Usage: rename_device <text>");
        return;
    }

    println!("Renaming device to \"{}\"", args[0]);
    DEVICE_NAME.lock().clear();
    DEVICE_NAME.lock().push_str(args[0]);
}

fn vga_graphics(_cmd: &mut String) {
    unsafe {
        COMMAND_LINE_ACTIVE = false;
        keyboard::INPUT_TARGET = keyboard::InputTarget::GraphicMode;
    }
    
    vga_driver::start();
}

fn print_colored(message: &str, color: Color16) {
    change_fg!(color);
    print!("{}", message);
    change_fg!(Color16::White);
}

fn triple_fault_reset(_cmd: &mut String) {
    println!("Triple fault, resetting...");
    asm::triple_fault();
}

fn credits(_cmd: &mut String) {
    /************************************************************************************
     * Added credits because without the people here, it wouldn't even have been possible
        for me to even get a basic vga_buffer running.
        - Avery
    ************************************************************************************/
    print_colored("\nMid", Color16::Magenta);
    print_colored("A", Color16::LightCyan);
    print_colored("S", Color16::Yellow);


    println!(" was created by:");
    print_colored("A", Color16::Yellow);
    print_colored("very", Color16::LightCyan);

    println!(" - @MindlessSea on GitHub");

    print_colored("Mid", Color16::Yellow);
    print_colored("na", Color16::Magenta);

    println!(" - @Midnight-Midna on GitHub");

    println!("\nSpecial thanks to:");

    /************
    * RustOS team
    ************/
    print_colored("The RustOS Team", Color16::LightRed);

    println!(" - @rust-osdev on GitHub");
    print!("for Developing RustOS libraries\n\n");

    /******************
    * Phillip Oppermann
    ******************/
    print_colored("Phillip Oppermann", Color16::Blue);

    println!(" - @phil-opp on GitHub");
    print!("for Developing the blog series \"Writing an OS in Rust\"\n\n");

    /**********
    * Jai/Aenri
    **********/
    print_colored("Jai/Aenri", Color16::Pink);

    println!(" - @jdadonut on GitHub");
    println!("for helping Avery out with fixing bugs");
    print!("(she made an OS called ");

    print_colored("\"veil\"", Color16::Magenta);

    println!(" go check it out!)\n");

    /**********
    * Rust Team
    **********/
    print_colored("The Rust Team", Color16::LightRed);

    println!(" - @rust-lang on GitHub");
    println!("for developing Rust\n");

    /*******
    * OSDev 
    *******/
    print_colored("The OSDev community", Color16::LightBlue);

    println!(" - https://wiki.osdev.org");
    print!("for being an awesome way to learn OS Development!\n\n");

    // TODO: Update credits
    
}

fn run(cmd: &mut String) {
    let args = cmd.split(' ').collect::<Vec<&str>>();

    if args.len() == 1 {
        println!("Usage: run <file>");
        return;
    }

    if args[0] == "elden_ring" {
        println!("Yes, it can.");
        return;
    }
}

fn echo(cmd: &mut String) {
    let args = cmd.split(' ').collect::<Vec<&str>>();

    if args.len() == 1 {
        println!("Usage: echo <text>");
        return;
    }

    let mut text = String::new();

    for arg in args.iter() {
        text.push_str(arg);
        text.push(' ');
    }

    println!("{}", text);
}

fn clear(_cmd: &mut String) {
    change_bg!(Color16::Black);
    change_fg!(Color16::White);

    // reset the cursor position
    unsafe {
        CURRENT_INDEX = 0;
    }

    clear_screen!();
}

fn print_based(_cmd: &mut String) {
    /**************************
    * Based stuff
    * that's the entire command
    **************************/
    change_fg!(Color16::LightCyan);
    print!("T");
    change_fg!(Color16::Pink);
    print!("R");
    change_fg!(Color16::White);
    print!("A");
    change_fg!(Color16::Pink);
    print!("N");
    change_fg!(Color16::LightCyan);
    print!("S");
        
    change_fg!(Color16::White);
    print!(" Rights are ");

    change_fg!(Color16::LightCyan);
    print!("H");
    change_fg!(Color16::Pink);
    print!("U");
    change_fg!(Color16::White);
    print!("M");
    change_fg!(Color16::Pink);
    print!("A");
    change_fg!(Color16::LightCyan);
    print!("N");
    change_fg!(Color16::White);
    print!(" Rights");
    println!();
}

fn version_info(_cmd: &mut String) {
    print_colored(r#"
ooo        ooooo  o8o        .o8        .o.        .oooooo..o 
`88.       .888'  `"'       "888       .888.      d8P'    `Y8 
 888b     d'888  oooo   .oooo888      .8"888.     Y88bo.      
 8 Y88. .P  888  `888  d88' `888     .8' `888.     `"Y8888o.  
 8  `888'   888   888  888   888    .88ooo8888.        `"Y88b 
 8    Y     888   888  888   888   .8'     `888.  oo     .d8P 
o8o        o888o o888o `Y8bod88P" o88o     o8888o 8""88888P'  
"#, Color16::Yellow);    

    print_colored(r#"
              .oooo.         .o  
             d8P'`Y8b      o888  
oooo    ooo 888    888      888  
 `88.  .8'  888    888      888  
  `88..8'   888    888      888  
   `888'    `88b  d88' .o.  888  
    `8'      `Y8bd8P'  Y8P o888o 
    "#, Color16::LightBlue);}

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

    unsafe {
        CURRENT_INDEX = buffer.iter().position(|&x| x == 0).unwrap() as u16;
    }
}

pub(crate) fn backspace() {
    if unsafe { CURRENT_INDEX == 0 } {
        return;
    }

    unsafe {
        let mut buffer = &mut COMMAND_LINE_BUFFER;
        buffer[CURRENT_INDEX as usize] = 0;
        buffer[CURRENT_INDEX as usize - 1] = 0;
        CURRENT_INDEX -= 1;
    }

    print!("\u{08}");
}
