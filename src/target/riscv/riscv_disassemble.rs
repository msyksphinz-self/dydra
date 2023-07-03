use spike_dasm_wrapper::Disasm;

pub fn disassemble_riscv(inst: u32) -> String {
    let mut disasm = Disasm::new();
    disasm.disassemble(inst)
}
