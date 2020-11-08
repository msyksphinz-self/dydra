pub mod elf_loader;
pub mod emu_env;
pub mod instr_info;
pub mod target;
pub mod tcg;
pub mod op_helper;
pub mod op_helper_fp_d;
pub mod op_helper_fp_s;
pub mod op_helper_mem;
pub mod op_helper_fcvt;

use crate::emu_env::{EmuEnv, ArgConfig};

pub fn run(filename: String, step: bool, exp_gpr: &[u64; 32]) -> usize {
    let arg_config = ArgConfig {
        step    : step,
        debug   : false,
        dump_gpr: false,
        dump_fpr: false,
        dump_tcg: false,
        mmu_debug: false,
        dump_guest: false,
        dump_host: false,
    };

    let mut emu = EmuEnv::new(arg_config);
    emu.run(&filename);
    let gpr_vec = emu.get_gpr();
    for (gpr_val, exp_val) in gpr_vec.iter().zip(exp_gpr.iter()) {
        if gpr_val != exp_val {
            print!("Failed: {:016x} != {:016x}\n", *gpr_val, *exp_val);
            return 1;
        }
    }
    return 0;
}

pub fn run_riscv_test(filename: String, opt_step: bool) -> u64 {
    let arg_config = ArgConfig {
        step    : opt_step,
        debug   : false,
        dump_gpr: false,
        dump_fpr: false,
        dump_tcg: false,
        mmu_debug : false,
        dump_guest: false,
        dump_host: false,
    };

    let mut emu = EmuEnv::new(arg_config);
    emu.run(&filename);
    return emu.get_mem(0x1000) as u64;
}