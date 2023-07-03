use crate::clap::Parser;
use emu_env::MachineEnum;

extern crate clap;
extern crate mmap;

pub mod elf_loader;
pub mod emu_env;
pub mod instr_info;
pub mod op_helper;
pub mod op_helper_fcvt;
pub mod op_helper_fp_d;
pub mod op_helper_fp_s;
pub mod op_helper_mem;
pub mod target;
pub mod tcg;

use crate::emu_env::ArgConfig;
use crate::emu_env::EmuEnv;

fn main() {
    let mut cfg = ArgConfig::parse();
    if cfg.dump_gpr || cfg.dump_fpr || cfg.dump_tcg {
        cfg.debug = true;
    }

    let mut emu = EmuEnv::new(cfg);
    emu.run();

    println!("Result: MEM[0x1000] = {:08x}", emu.get_mem(0x1000));

    return;
}
