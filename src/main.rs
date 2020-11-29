use clap::{App,Arg};
use emu_env::MachineEnum;

extern crate mmap;
extern crate clap;

pub mod elf_loader;
pub mod emu_env;
pub mod instr_info;
pub mod target;
pub mod tcg;
pub mod op_helper;
pub mod op_helper_fp_s;
pub mod op_helper_fp_d;
pub mod op_helper_mem;
pub mod op_helper_fcvt;

use crate::emu_env::EmuEnv;
use crate::emu_env::ArgConfig;

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
        Arg::new("machine")
        .about("specify machine")
        .value_name("Machine Name")
        .long("machine")
        .required(true)
    )
    .arg(
        Arg::new("debug")
        .about("Debug mode")
        .short('d')
        .long("debug")
        .required(false)
    )
    .arg(
        Arg::new("mmu-debug")
        .about("MMU debug log output")
        .short('m')
        .long("mmu")
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
    .arg(
        Arg::new("dump-guest")
        .about("Dump Guest Instruction")
        .long("dump-guest")
        .required(false)
    )
    .arg(
        Arg::new("dump-host")
        .about("Dump Host Instruction")
        .long("dump-host")
        .required(false)
    )
    .get_matches();

    let arg_config_step     = matches.is_present("step");
    let arg_config_dump_gpr = matches.is_present("dump-gpr");
    let arg_config_dump_fpr = matches.is_present("dump-fpr");
    let arg_config_dump_tcg = matches.is_present("dump-tcg");
    let arg_config_mmu_debug = matches.is_present("mmu-debug");
    let arg_config_dump_guest = matches.is_present("dump-guest");
    let arg_config_dump_host = matches.is_present("dump-host");
    let arg_config_debug    = matches.is_present("debug") || arg_config_dump_gpr || arg_config_dump_fpr || arg_config_dump_tcg;
    let arg_config_machine_string = matches.values_of("machine").unwrap().next().unwrap().to_string();
    let arg_config_machine = match &*arg_config_machine_string {
        "virt" => MachineEnum::RiscvVirt,
        "sifive_u" => MachineEnum::RiscvSiFiveU,
        _ => panic!("-machine not specified"),
    };
    let arg_config = ArgConfig {
        step    : arg_config_step,
        debug   : arg_config_debug,
        dump_gpr: arg_config_dump_gpr,
        dump_fpr: arg_config_dump_fpr,
        dump_tcg: arg_config_dump_tcg,
        mmu_debug: arg_config_mmu_debug,
        dump_guest: arg_config_dump_guest,
        dump_host: arg_config_dump_host,
        machine: arg_config_machine,
    };

    let elf_file = matches.values_of("elf-file").unwrap().next().unwrap().to_string();

    let mut emu = EmuEnv::new(arg_config);
    emu.run(&elf_file);

    println!("Result: MEM[0x1000] = {:08x}", emu.get_mem(0x1000));

    return;
}
