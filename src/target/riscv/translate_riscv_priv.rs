use super::super::super::tcg::tcg::{TCGOp, TCGOpcode, TCGv};
use super::super::super::instr_info::InstrInfo;
use super::riscv::CALL_HELPER_IDX;

use super::super::super::get_rs1_addr;
use super::super::super::get_rd_addr;

use super::riscv::TranslateRiscv;

impl TranslateRiscv {
    pub fn translate_csrrw(&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRW_IDX as usize, *rd, *rs1, *csr);
        (false, vec![csr_op])
    }
    pub fn translate_csrrs(&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRS_IDX as usize, *rd, *rs1, *csr);

        (false, vec![csr_op])
    }
    pub fn translate_csrrc(&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRC_IDX as usize, *rd, *rs1, *csr);
        (false, vec![csr_op])
    }
    pub fn translate_csrrwi(&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rs1_imm: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_imm(rs1_imm as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRWI_IDX as usize, *rd, *rs1, *csr);

        (false, vec![csr_op])
    }
    pub fn translate_csrrsi(&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rs1_imm: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_imm(rs1_imm as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRSI_IDX as usize, *rd, *rs1, *csr);
        (false, vec![csr_op])
    }
    pub fn translate_csrrci(&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rs1_imm: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_imm(rs1_imm as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRCI_IDX as usize, *rd, *rs1, *csr);
        (false, vec![csr_op])
    }

    pub fn translate_fence(&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        (false, vec![])
    }
    pub fn translate_fence_i(&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        (false, vec![exit_tb])
    }
    pub fn translate_sfence_vma(&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let op = TCGOp::new_helper_call_arg0(CALL_HELPER_IDX::CALL_SFENCE_VMA_IDX as usize);
        (false, vec![op])
    }
    pub fn translate_mret(&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let mret_op = TCGOp::new_helper_call_arg0(CALL_HELPER_IDX::CALL_MRET_IDX as usize);
        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        (false, vec![mret_op, exit_tb])
    }

    pub fn translate_ecall(&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let ecall_op = TCGOp::new_helper_call_arg0(CALL_HELPER_IDX::CALL_ECALL_IDX as usize);
        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        (true, vec![ecall_op, exit_tb])
    }

    pub fn translate_sret(&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let mret_op = TCGOp::new_helper_call_arg0(CALL_HELPER_IDX::CALL_SRET_IDX as usize);
        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        (false, vec![mret_op, exit_tb])
    }

}
