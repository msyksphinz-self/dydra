extern crate mmap;
use std::env;

pub mod elf_loader;
pub mod emu_env;
pub mod instr_info;
pub mod riscv;
pub mod riscv_decoder;
pub mod riscv_inst_id;
pub mod tcg;
pub mod x86;

use crate::emu_env::EmuEnv;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let emu = EmuEnv::new();
    emu.run(&filename);

    return;
}
