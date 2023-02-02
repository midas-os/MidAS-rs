/**************************************************************************************************
* Name : 								  task/keyboard.rs
* Author : 										Avery
* Date : 									  2/01/2023
* Purpose : 					      Asynchronous Keyboard Input
* Version : 									 0.1
**************************************************************************************************/

use conquer_once::spin::{OnceCell};
use crossbeam_queue::ArrayQueue;
use crate::{print, println, change_fg, vga_buffer::Color, cmd};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
use core::{pin::Pin, task::{Poll, Context}};
use futures_util::{task::AtomicWaker, stream::{Stream, StreamExt}};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        // fast path
        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        change_fg!(Color::Yellow);
        println!("WARNING: scancode queue uninitialized");
        change_fg!(Color::White);
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
        HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                let mut real_key = key;
                
                if cmd::is_active() {
                    if key == DecodedKey::Unicode('\n') {
                        cmd::process_command();
                        real_key = DecodedKey::Unicode('\u{80}');
                    } else if key == DecodedKey::Unicode('\u{08}') {
                        cmd::backspace();
                        continue;
                    } else {
                        cmd::add_char(key);
                    }
                }

                match real_key {
                    DecodedKey::Unicode(character) => {
                        if character == '\u{80}' {
                            print!("\n{}", cmd::COMMAND_PREFIX);
                        } else if character == '\u{08}' {
                            continue;
                        } else {
                            print!("{}", character);
                        }
                    }
                    DecodedKey::RawKey(key) => {
                        let key_u8 = key as u8;

                        /************************************
                        * Check if key is between 0x0 and 0x5
                        ************************************/
                        for i in 0x0..=0x5 {
                            if key_u8 == i {
                                continue;
                            }
                        }

                        /************************************
                        * Check if key is 0x21, 0x40, or 0x41
                        ************************************/
                        if key_u8 == 0x21 || key_u8 == 0x40 || key_u8 == 0x41 {
                            continue;
                        }
                    }
                }
            }
        }
    }
}