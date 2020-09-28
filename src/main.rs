use clap::{App,Arg};
extern crate mmap;
use std::env;
extern crate clap;

pub mod elf_loader;
pub mod emu_env;
pub mod instr_info;
pub mod target;
pub mod tcg;
pub mod op_helper;

use crate::emu_env::EmuEnv;

fn main() {
    let matches = App::new("Hydra")
    .version("0.0.1")
    .author("msyksphinz")
    .about("Binary Translated Instruction Set Emulator")
    .arg(
        Arg::new("elf_file")
        .about("ELF file for emulation")
        .value_name("ELF_FILE")
        .short('b')
        .long("bin_file")
        .required(true),
    )
    .arg(
        Arg::new("debug")
        .about("Debug mode")
        .short('d')
        .long("debug")
        .required(false)
    )
    .get_matches();

    let elf_file = matches.values_of("elf_file").unwrap().next().unwrap().to_string();
    let debug = matches.is_present("debug");
    println!("Debug Mode specified : {}\n", debug);

    let mut emu = EmuEnv::new();
    emu.run(&elf_file, debug);

    println!("Result: MEM[0x1000] = {:08x}", emu.get_mem(0x1000));

    return;
}
