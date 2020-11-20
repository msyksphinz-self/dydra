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

#[macro_export]
macro_rules! get_nzimm {
    ($inst:expr) => {
        extend_sign(((($inst as u64 >> 12) &  0x1) << 5) |
                    ((($inst as u64 >>  2) & 0x1f) << 0), 5)
    };
}


macro_rules! get_c_reg_addr {
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

macro_rules! get_nzimm_addi16sp {
    ($inst: expr) => {
        extend_sign(((((($inst >>12) & 0x1) << 5) |
                      ((($inst >> 3) & 0x3) << 3) |
                      ((($inst >> 5) & 0x1) << 2) |
                      ((($inst >> 2) & 0x1) << 1) |
                      ((($inst >> 6) & 0x1) << 0)) << 4) as u64, 9)
    }
}


#[inline]
fn extend_sign(data: u64, msb: usize) -> i64
{
  let mask = 1 << msb; // mask can be pre-computed if b is fixed
  let data = data & ((1 << (msb + 1)) - 1);  // (Skip this if bits in x above position b are already zero.)
  
  ((data ^ mask).wrapping_sub(mask)) as i64
}


impl TranslateRiscv {
    pub fn translate_c_addi4spn(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const: u64 = get_nzuimm!(inst.inst as i32);
        let rs1_addr= 2;  // sp
        let rd_addr = get_c_reg_addr!((inst.inst >> 2) & 0x7);

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

    pub fn translate_c_fld  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        vec![] 
    }
    pub fn translate_c_lw   (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let imm = extend_sign((((((inst.inst >> 10) & 0x7) << 3) |
                                (((inst.inst >>  6) & 0x1) << 2) |
                                (((inst.inst >>  5) & 0x1) << 6))) as u64, 6);

        self.translate_raw_load(get_c_reg_addr!((inst.inst >> 7) & 0x7), imm as u64, get_c_reg_addr!((inst.inst >> 2) & 0x7), inst, TCGOpcode::LOAD_32BIT, CALL_HELPER_IDX::CALL_LOAD32_IDX)
    }

    pub fn translate_c_flw  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_ld   (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let imm = extend_sign((((((inst.inst >> 10) & 0x7) << 3) |
                                (((inst.inst >>  5) & 0x3) << 6))) as u64, 6);

        self.translate_raw_load(get_c_reg_addr!((inst.inst >> 7) & 0x7), imm as u64, get_c_reg_addr!((inst.inst >> 2) & 0x7), inst, TCGOpcode::LOAD_64BIT, CALL_HELPER_IDX::CALL_LOAD32_IDX)
    }
    pub fn translate_c_fsd  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_sw   (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let imm = extend_sign((((((inst.inst >> 10) & 0x7) << 3) |
                                (((inst.inst >>  6) & 0x1) << 2) |
                                (((inst.inst >>  5) & 0x1) << 6))) as u64, 6);

        self.translate_raw_store(get_c_reg_addr!((inst.inst >> 7) & 0x7), imm as u64, get_c_reg_addr!((inst.inst >> 2) & 0x7), inst, TCGOpcode::STORE_32BIT, CALL_HELPER_IDX::CALL_STORE32_IDX)
    }

    pub fn translate_c_fsw  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_sd   (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let imm = extend_sign((((((inst.inst >> 10) & 0x7) << 3) |
                                (((inst.inst >>  5) & 0x3) << 6))) as u64, 6);

        self.translate_raw_store(get_c_reg_addr!((inst.inst >> 7) & 0x7), imm as u64, get_c_reg_addr!((inst.inst >> 2) & 0x7), inst, TCGOpcode::STORE_64BIT, CALL_HELPER_IDX::CALL_STORE32_IDX)
    }
    pub fn translate_c_nop  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_addi (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let imm_const = get_nzimm!(inst.inst as i32);
        let rs1_addr = get_rd_addr!(inst.inst);
        let rd_addr  = get_rd_addr!(inst.inst);

        let mut tcg_lists = vec![];

        let source1 = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_64BIT, source1, source1, TCGv::new_imm(imm_const as u64)));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, source1));

        self.tcg_temp_free(source1);
        tcg_lists
    }
    pub fn translate_c_jal  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_addiw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_li   (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }


    pub fn translate_c_addi16sp(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const = get_nzimm_addi16sp!(inst.inst as i32);
        let rs1_addr= 2;  // sp
        let rd_addr = 2;  // sp

        let mut tcg_lists = vec![];

        let source1 = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_64BIT, source1, source1, TCGv::new_imm(imm_const as u64)));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, source1));

        self.tcg_temp_free(source1);
        tcg_lists
    }

    pub fn translate_c_lui   (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_srli  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_srli64(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_srai  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_srai64(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_andi  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_sub   (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_xor   (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_or    (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_and   (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_subw  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_addw  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_j     (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_beqz  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_bnez  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_slli  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_fldsp (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_lwsp  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_flwsp (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_ldsp  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_jr    (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_mv    (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_ebreak(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_jalr  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_add   (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_fsdsp (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_swsp  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_fswsp (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }
    pub fn translate_c_sdsp  (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { vec![] }


}
