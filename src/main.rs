extern crate mmap;
use std::env;

pub mod elf_loader;
pub mod emu_env;
pub mod instr_info;
pub mod target;
pub mod tcg;

use crate::emu_env::EmuEnv;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut emu = EmuEnv::new();
    emu.run(&filename);

    println!("Result: MEM[0x1000] = {:08x}", emu.get_mem(0x1000));

    return;
}
