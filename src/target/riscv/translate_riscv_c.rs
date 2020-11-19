use std::cell::RefCell;
use std::rc::Rc;

use super::super::super::tcg::tcg::{TCGOp, TCGOpcode, TCGv, TCGLabel};
use super::super::super::instr_info::InstrInfo;
use super::riscv::{CALL_HELPER_IDX, CallFcvtIdx};

use super::riscv::TranslateRiscv;

#[macro_export]
macro_rules! get_nzuimm {
    ($inst:expr) => {
        ((($inst as u64 >> 11) & 0x3) << 4) |
        ((($inst as u64 >>  7) & 0xf) << 6) |
        ((($inst as u64 >>  6) & 0x1) << 2) |
        ((($inst as u64 >>  5) & 0x1) << 3)
    };
}

macro_rules! get_c_rd_addr {
    ($c_reg_addr:expr) => {
        match $c_reg_addr {
            0 => 8,  //  x8,  f8
            1 => 9,  //  x9,  f9
            2 => 10, // x10, f10
            3 => 11, // x11, f11
            4 => 12, // x12, f12
            5 => 13, // x13, f13
            6 => 14, // x14, f14
            7 => 15, // x15, f15
            _ => panic!("Not covered")
        }
    }
}

impl TranslateRiscv {
    pub fn translate_c_addi4spn(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const: u64 = get_nzuimm!(inst.inst as i32);
        let rs1_addr= 2;  // sp
        let rd_addr = get_c_rd_addr!((inst.inst >> 2) & 0x7);

        let mut tcg_lists = vec![];

        if rd_addr == 0 {
            return vec![];
        }

        let source1 = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_64BIT, source1, source1, TCGv::new_imm(imm_const)));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, source1));

        self.tcg_temp_free(source1);
        tcg_lists
    }
}
