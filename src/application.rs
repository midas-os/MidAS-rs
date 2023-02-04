use pc_keyboard::DecodedKey;
use crate::task::keyboard::{self};

// Application class with a name and a running state
pub struct Application {
    pub name: &'static str,
    pub running: bool,
    pub can_run: bool,
}

impl Application {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            running: false,
            can_run: true,
        }
    }

    pub const fn new_unrunnable(name: &'static str) -> Self {
        Self {
            name,
            running: false,
            can_run: false,
        }
    }

    pub fn init(self) {
        unsafe {
            keyboard::INPUT_TARGET = keyboard::InputTarget::Application;
            keyboard::APPLICATION = self;
        }
    }

    pub fn run(&mut self) {
        if !self.can_run {
            return;
        }

        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn redirect_input(&mut self, key: DecodedKey) {
        unimplemented!("Function has to be implemented by target app!");
    }
}