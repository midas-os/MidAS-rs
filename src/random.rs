use crate::{interrupts::get_index};

fn get_cycles() -> u64 {
    let mut result: u64 = 0;

    unsafe {
        core::arch::asm!("rdtsc", out("rax") result);
    }

    result
}

pub fn generate_rnd() -> u64 {
    let cycles: u64 = get_cycles();
    let index: u64 = get_index();

    cycles ^ index
}

pub fn generate_rnd_01() -> f64 {
    let result = generate_rnd_rng(0, 100);

    result as f64 / 100.0
}

pub fn generate_rnd_rng(min: u64, max: u64) -> u64 {
    let cycles: u64 = get_cycles();
    let index: u64 = get_index();

    let result: u64 = cycles ^ index;

    min + (result % (max - min))
}