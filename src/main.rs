use clap::{App,Arg};
extern crate mmap;
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
        Arg::new("elf-file")
        .about("ELF file for emulation")
        .value_name("ELF_FILE")
        .short('e')
        .long("elf-file")
        .required(true),
    )
    .arg(
        Arg::new("debug")
        .about("Debug mode")
        .short('d')
        .long("debug")
        .required(false)
    )
    .arg(
        Arg::new("dump-gpr")
        .about("Dump Integer Register by Each Block")
        .long("dump-gpr")
        .required(false)
    )
    .arg(
        Arg::new("dump-fpr")
        .about("DUmp Floating Point Register by Each Block")
        .long("dump-fpr")
        .required(false)
    )
    .arg(
        Arg::new("dump-tcg")
        .about("Dump Translated TCG for each Block")
        .long("dump-tcg")
        .required(false)
    )
    .arg(
        Arg::new("step")
        .about("TCG Translation, step execution")
        .long("step")
        .short('s')
        .required(false)
    )
    .get_matches();

    let elf_file = matches.values_of("elf-file").unwrap().next().unwrap().to_string();
    let step = matches.is_present("step");
    let debug = matches.is_present("debug");
    let dump_gpr = matches.is_present("dump-gpr");
    let dump_fpr = matches.is_present("dump-fpr");
    let dump_tcg = matches.is_present("dump-tcg");
    let debug = if dump_gpr || dump_fpr || dump_tcg { true } else { debug };

    let mut emu = EmuEnv::new();
    emu.run(&elf_file, debug, dump_gpr, dump_fpr, dump_tcg, step);

    println!("Result: MEM[0x1000] = {:08x}", emu.get_mem(0x1000));

    return;
}
