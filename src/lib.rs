pub mod elf_loader;
pub mod emu_env;
pub mod instr_info;
pub mod riscv;
pub mod riscv_csr;
pub mod riscv_decoder;
pub mod riscv_inst_id;
pub mod tcg;
pub mod x86;

use crate::emu_env::EmuEnv;

pub fn run(filename: String, exp_gpr: &[u64]) -> usize {
    let mut emu = EmuEnv::new();
    emu.run(&filename);
    let gpr_vec = emu.get_gpr();
    for (gpr_val, exp_val) in gpr_vec.iter().zip(exp_gpr.iter()) {
        if gpr_val != exp_val {
            print!("Failed. %08x != %08x\n");
            return 1;
        }
    }
    return 0;
}
