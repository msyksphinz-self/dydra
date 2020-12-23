use std::cell::RefCell;
use std::rc::Rc;

use super::super::super::tcg::tcg::{TCGOp, TCGOpcode, TCGv, TCGLabel};
use super::super::super::instr_info::InstrInfo;
use super::riscv::{CALL_HELPER_IDX};

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


#[macro_export]
macro_rules! get_nzimm_lui {
    ($inst:expr) => {
        extend_sign(((($inst as u64 >> 12) &  0x1) << 17) |
                    ((($inst as u64 >>  2) & 0x1f) << 12), 17)
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
    pub fn translate_c_addi4spn(&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm_const: u64 = get_nzuimm!(inst.inst as i32);
        let rs1_addr= 2;  // sp
        let rd_addr = get_c_reg_addr!((inst.inst >> 2) & 0x7);

        let mut tcg_lists = vec![];

        if rd_addr == 0 {
            return (false, vec![]);
        }

        let source1 = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_64BIT, source1, source1, TCGv::new_imm(imm_const)));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, source1));

        self.tcg_temp_free(source1);
        (false, tcg_lists)
    }

    pub fn translate_c_fld  (&mut self, __inst: &InstrInfo) -> (bool, Vec<TCGOp>) { (false, vec![]) }
    pub fn translate_c_lw   (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm = extend_sign((((((inst.inst >> 10) & 0x7) << 3) |
                                (((inst.inst >>  6) & 0x1) << 2) |
                                (((inst.inst >>  5) & 0x1) << 6))) as u64, 6);

        self.translate_raw_load(get_c_reg_addr!((inst.inst >> 7) & 0x7), imm as u64, get_c_reg_addr!((inst.inst >> 2) & 0x7), inst, TCGOpcode::LOAD_32BIT, CALL_HELPER_IDX::CALL_LOAD32_IDX)
    }

    pub fn translate_c_flw  (&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) { (false, vec![]) }
    pub fn translate_c_ld   (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm = extend_sign((((((inst.inst >> 10) & 0x7) << 3) |
                                (((inst.inst >>  5) & 0x3) << 6))) as u64, 6);

        self.translate_raw_load(get_c_reg_addr!((inst.inst >> 7) & 0x7), imm as u64, get_c_reg_addr!((inst.inst >> 2) & 0x7), inst, TCGOpcode::LOAD_64BIT, CALL_HELPER_IDX::CALL_LOAD64_IDX)
    }
    pub fn translate_c_fsd  (&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) { (false, vec![]) }
    pub fn translate_c_sw   (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm = extend_sign((((((inst.inst >> 10) & 0x7) << 3) |
                                (((inst.inst >>  6) & 0x1) << 2) |
                                (((inst.inst >>  5) & 0x1) << 6))) as u64, 6);

        self.translate_raw_store(get_c_reg_addr!((inst.inst >> 7) & 0x7), imm as u64, get_c_reg_addr!((inst.inst >> 2) & 0x7), inst, TCGOpcode::STORE_32BIT, CALL_HELPER_IDX::CALL_STORE32_IDX)
    }

    pub fn translate_c_fsw  (&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) { (false, vec![]) }
    pub fn translate_c_sd   (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm = extend_sign((((((inst.inst >> 10) & 0x7) << 3) |
                                (((inst.inst >>  5) & 0x3) << 6))) as u64, 6);

        self.translate_raw_store(get_c_reg_addr!((inst.inst >> 7) & 0x7), imm as u64, get_c_reg_addr!((inst.inst >> 2) & 0x7), inst, TCGOpcode::STORE_64BIT, CALL_HELPER_IDX::CALL_STORE64_IDX)
    }
    pub fn translate_c_nop  (&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) { (false, vec![]) }
    pub fn translate_c_addi (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm_const = get_nzimm!(inst.inst as i32);
        let rs1_addr = get_rd_addr!(inst.inst);
        let rd_addr  = get_rd_addr!(inst.inst);

        let mut tcg_lists = vec![];

        let source1 = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_64BIT, source1, source1, TCGv::new_imm(imm_const as u64)));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, source1));

        self.tcg_temp_free(source1);
        (false, tcg_lists)
    }
    pub fn translate_c_jal (&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        (true, vec![])
    }


    pub fn translate_c_addiw (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm_const = get_nzimm!(inst.inst as i32);
        let rd_addr = get_rd_addr!(inst.inst);

        let mut tcg_lists = vec![];

        let source1 = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rd_addr));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_32BIT, source1, source1, TCGv::new_imm(imm_const as u64)));
        tcg_lists.push(TCGOp::new_2op(TCGOpcode::SIGN_EXT_32_64, source1, source1));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, source1));
        self.tcg_temp_free(source1);

        (false, tcg_lists)
    }


    pub fn translate_c_li (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm_const = get_nzimm!(inst.inst as i32);
        let rd_addr  = get_rd_addr!(inst.inst);
        let mut tcg_lists = vec![];

        let dest_tmp = self.tcg_temp_new();
        tcg_lists.push(TCGOp::tcg_get_gpr(dest_tmp, 0));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_64BIT, dest_tmp, dest_tmp, TCGv::new_imm(imm_const as u64)));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, dest_tmp));

        self.tcg_temp_free(dest_tmp);
        (false, tcg_lists)
    }


    pub fn translate_c_addi16sp (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm_const = get_nzimm_addi16sp!(inst.inst as i32);
        let rs1_addr= 2;  // sp
        let rd_addr = 2;  // sp

        let mut tcg_lists = vec![];

        let source1 = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_64BIT, source1, source1, TCGv::new_imm(imm_const as u64)));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, source1));

        self.tcg_temp_free(source1);
        (false, tcg_lists)
    }

    pub fn translate_c_lui   (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm_const = get_nzimm_lui!(inst.inst as i32);
        let rd_addr  = get_rd_addr!(inst.inst);
        let mut tcg_lists = vec![];

        let dest_tmp = self.tcg_temp_new();
        tcg_lists.push(TCGOp::tcg_get_gpr(dest_tmp, 0));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_64BIT, dest_tmp, dest_tmp, TCGv::new_imm(imm_const as u64)));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, dest_tmp));

        self.tcg_temp_free(dest_tmp);
        (false, tcg_lists)
    }

    pub fn translate_c_srli  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let shamt    = get_nzimm!(inst.inst);
        let rd_addr  = get_c_reg_addr!((inst.inst >> 7) & 0x7);

        let mut tcg_list = vec![];

        let source1 = self.tcg_temp_new();
        tcg_list.push(TCGOp::tcg_get_gpr(source1, rd_addr));
        tcg_list.push(TCGOp::new_3op(TCGOpcode::SRL_64BIT, source1, source1, TCGv::new_imm(shamt as u64)));
        tcg_list.push(TCGOp::tcg_set_gpr(rd_addr, source1));
        self.tcg_temp_free(source1);

        (false, tcg_list)
    }


    pub fn translate_c_srli64 (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let shamt    = get_nzimm!(inst.inst);
        let rd_addr  = get_c_reg_addr!((inst.inst >> 7) & 0x7);

        let mut tcg_list = vec![];

        let source1 = self.tcg_temp_new();
        tcg_list.push(TCGOp::tcg_get_gpr(source1, rd_addr));
        tcg_list.push(TCGOp::new_3op(TCGOpcode::SRL_64BIT, source1, source1, TCGv::new_imm(shamt as u64)));
        tcg_list.push(TCGOp::tcg_set_gpr(rd_addr, source1));
        self.tcg_temp_free(source1);

        (false, tcg_list)
    }

    pub fn translate_c_srai  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let shamt    = get_nzimm!(inst.inst);
        let rd_addr  = get_c_reg_addr!((inst.inst >> 7) & 0x7);

        let mut tcg_list = vec![];

        let source1 = self.tcg_temp_new();
        tcg_list.push(TCGOp::tcg_get_gpr(source1, rd_addr));
        tcg_list.push(TCGOp::new_3op(TCGOpcode::SRA_64BIT, source1, source1, TCGv::new_imm(shamt as u64)));
        tcg_list.push(TCGOp::tcg_set_gpr(rd_addr, source1));
        self.tcg_temp_free(source1);

        (false, tcg_list)
    }


    pub fn translate_c_srai64 (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let shamt    = get_nzimm!(inst.inst);
        let rd_addr  = get_c_reg_addr!((inst.inst >> 7) & 0x7);

        let mut tcg_list = vec![];

        let source1 = self.tcg_temp_new();
        tcg_list.push(TCGOp::tcg_get_gpr(source1, rd_addr));
        tcg_list.push(TCGOp::new_3op(TCGOpcode::SRA_64BIT, source1, source1, TCGv::new_imm(shamt as u64)));
        tcg_list.push(TCGOp::tcg_set_gpr(rd_addr, source1));
        self.tcg_temp_free(source1);

        (false, tcg_list)
    }


    pub fn translate_c_andi  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm_const = get_nzimm!(inst.inst as i32);
        let rd_addr  = get_c_reg_addr!((inst.inst >> 7) & 0x7);

        let mut tcg_lists = vec![];

        let source1 = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rd_addr));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::AND_64BIT, source1, source1, TCGv::new_imm(imm_const as u64)));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, source1));

        self.tcg_temp_free(source1);
        (false, tcg_lists)
    }


    pub fn translate_c_sub   (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rd_addr   = get_c_reg_addr!((inst.inst >> 7) & 0x7);
        let rs2_addr  = get_c_reg_addr!((inst.inst >> 2) & 0x7);

        let mut tcg_lists = vec![];

        let rd_tmp = self.tcg_temp_new();
        let rs2_tmp = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(rd_tmp, rd_addr));
        tcg_lists.push(TCGOp::tcg_get_gpr(rs2_tmp, rs2_addr));

        tcg_lists.push(TCGOp::new_3op(TCGOpcode::SUB_64BIT, rd_tmp, rd_tmp, rs2_tmp));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, rd_tmp));

        self.tcg_temp_free(rd_tmp);
        self.tcg_temp_free(rs2_tmp);

        (false, tcg_lists)
    }


    pub fn translate_c_xor   (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rd_addr   = get_c_reg_addr!((inst.inst >> 7) & 0x7);
        let rs2_addr  = get_c_reg_addr!((inst.inst >> 2) & 0x7);

        let mut tcg_lists = vec![];

        let rd_tmp = self.tcg_temp_new();
        let rs2_tmp = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(rd_tmp, rd_addr));
        tcg_lists.push(TCGOp::tcg_get_gpr(rs2_tmp, rs2_addr));

        tcg_lists.push(TCGOp::new_3op(TCGOpcode::XOR_64BIT, rd_tmp, rd_tmp, rs2_tmp));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, rd_tmp));

        self.tcg_temp_free(rd_tmp);
        self.tcg_temp_free(rs2_tmp);

        (false, tcg_lists)
    }

    pub fn translate_c_or    (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rd_addr   = get_c_reg_addr!((inst.inst >> 7) & 0x7);
        let rs2_addr  = get_c_reg_addr!((inst.inst >> 2) & 0x7);

        let mut tcg_lists = vec![];

        let rd_tmp = self.tcg_temp_new();
        let rs2_tmp = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(rd_tmp, rd_addr));
        tcg_lists.push(TCGOp::tcg_get_gpr(rs2_tmp, rs2_addr));

        tcg_lists.push(TCGOp::new_3op(TCGOpcode::OR_64BIT, rd_tmp, rd_tmp, rs2_tmp));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, rd_tmp));

        self.tcg_temp_free(rd_tmp);
        self.tcg_temp_free(rs2_tmp);

        (false, tcg_lists)
    }
    pub fn translate_c_and   (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rd_addr   = get_c_reg_addr!((inst.inst >> 7) & 0x7);
        let rs2_addr  = get_c_reg_addr!((inst.inst >> 2) & 0x7);

        let mut tcg_lists = vec![];

        let rd_tmp = self.tcg_temp_new();
        let rs2_tmp = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(rd_tmp, rd_addr));
        tcg_lists.push(TCGOp::tcg_get_gpr(rs2_tmp, rs2_addr));

        tcg_lists.push(TCGOp::new_3op(TCGOpcode::AND_64BIT, rd_tmp, rd_tmp, rs2_tmp));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, rd_tmp));

        self.tcg_temp_free(rd_tmp);
        self.tcg_temp_free(rs2_tmp);

        (false, tcg_lists)
    }


    pub fn translate_c_subw  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rd_addr   = get_c_reg_addr!((inst.inst >> 7) & 0x7);
        let rs2_addr  = get_c_reg_addr!((inst.inst >> 2) & 0x7);

        let mut tcg_lists = vec![];

        let rd_tmp = self.tcg_temp_new();
        let rs2_tmp = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(rd_tmp, rd_addr));
        tcg_lists.push(TCGOp::tcg_get_gpr(rs2_tmp, rs2_addr));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::SUB_32BIT, rd_tmp, rd_tmp, rs2_tmp));
        tcg_lists.push(TCGOp::new_2op(TCGOpcode::SIGN_EXT_32_64, rd_tmp, rd_tmp));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, rd_tmp));

        self.tcg_temp_free(rd_tmp);
        self.tcg_temp_free(rs2_tmp);

        (false, tcg_lists)
    }


    pub fn translate_c_addw  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rd_addr   = get_c_reg_addr!((inst.inst >> 7) & 0x7);
        let rs2_addr  = get_c_reg_addr!((inst.inst >> 2) & 0x7);

        let mut tcg_lists = vec![];

        let rd_tmp = self.tcg_temp_new();
        let rs2_tmp = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(rd_tmp, rd_addr));
        tcg_lists.push(TCGOp::tcg_get_gpr(rs2_tmp, rs2_addr));
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_32BIT, rd_tmp, rd_tmp, rs2_tmp));
        tcg_lists.push(TCGOp::new_2op(TCGOpcode::SIGN_EXT_32_64, rd_tmp, rd_tmp));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, rd_tmp));

        self.tcg_temp_free(rd_tmp);
        self.tcg_temp_free(rs2_tmp);

        (false, tcg_lists)
    }
    pub fn translate_c_j (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let jmp_const = (((inst.inst >> 12) & 0x1) << 11) |
                        (((inst.inst >> 11) & 0x1) <<  4) |
                        (((inst.inst >>  9) & 0x3) <<  8) |
                        (((inst.inst >>  8) & 0x1) << 10) |
                        (((inst.inst >>  7) & 0x1) <<  6) |
                        (((inst.inst >>  6) & 0x1) <<  7) |
                        (((inst.inst >>  3) & 0x7) <<  1) |
                        (((inst.inst >>  2) & 0x1) <<  5);
        let jmp_const = extend_sign(jmp_const as u64, 11);

        let mut tcg_lists = vec![];

        let dest_temp = self.tcg_temp_new();
        tcg_lists.push(TCGOp::new_2op(TCGOpcode::JMPIM, dest_temp, TCGv::new_imm(inst.addr.wrapping_add(jmp_const as u64))));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));
        self.tcg_temp_free(dest_temp);

        (true, tcg_lists)
    }
    pub fn translate_c_beqz  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rs1_addr: usize = get_c_reg_addr!((inst.inst >> 7) & 0x7) as usize;

        let target = (((inst.inst >> 12) & 0x1) << 8) |
                     (((inst.inst >> 10) & 0x3) << 3) |
                     (((inst.inst >>  5) & 0x3) << 6) |
                     (((inst.inst >>  3) & 0x3) << 1) |
                     (((inst.inst >>  2) & 0x1) << 5);
        let target = extend_sign (target as u64, 8);
        let target = inst.addr.wrapping_add(target as u64);
        let label = Rc::new(RefCell::new(TCGLabel::new()));

        let mut tcg_lists = vec![];

        let rs1 = self.tcg_temp_new();
        let zero = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(rs1, rs1_addr as u32));
        tcg_lists.push(TCGOp::tcg_get_gpr(zero, 0 as u32));

        tcg_lists.push(TCGOp::new_4op(TCGOpcode::EQ_64BIT, rs1, zero, TCGv::new_imm(target as u64), Rc::clone(&label)));
        tcg_lists.push(TCGOp::new_goto_tb(TCGv::new_imm(inst.addr + 2)));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));

        tcg_lists.push(TCGOp::new_label(Rc::clone(&label)));
        tcg_lists.push(TCGOp::new_goto_tb(TCGv::new_imm(target  as u64)));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));

        self.tcg_temp_free(rs1);
        self.tcg_temp_free(zero);

        (false, tcg_lists)
    }
    pub fn translate_c_bnez  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rs1_addr: usize = get_c_reg_addr!((inst.inst >> 7) & 0x7) as usize;
        let target = (((inst.inst >> 12) & 0x1) << 8) |
                     (((inst.inst >> 10) & 0x3) << 3) |
                     (((inst.inst >>  5) & 0x3) << 6) |
                     (((inst.inst >>  3) & 0x3) << 1) |
                     (((inst.inst >>  2) & 0x1) << 5);
        let target = extend_sign (target as u64, 8);
        let target = inst.addr.wrapping_add(target as u64);
        let label = Rc::new(RefCell::new(TCGLabel::new()));

        let mut tcg_lists = vec![];

        let rs1 = self.tcg_temp_new();
        let zero = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(rs1, rs1_addr as u32));
        tcg_lists.push(TCGOp::tcg_get_gpr(zero, 0 as u32));

        tcg_lists.push(TCGOp::new_4op(TCGOpcode::NE_64BIT, rs1, zero, TCGv::new_imm(target as u64), Rc::clone(&label)));
        tcg_lists.push(TCGOp::new_goto_tb(TCGv::new_imm(inst.addr + 2)));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));

        tcg_lists.push(TCGOp::new_label(Rc::clone(&label)));
        tcg_lists.push(TCGOp::new_goto_tb(TCGv::new_imm(target  as u64)));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));

        self.tcg_temp_free(rs1);
        self.tcg_temp_free(zero);

        (false, tcg_lists)
    }


    pub fn translate_c_slli  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let shamt    = get_nzimm!(inst.inst);
        let rd_addr  = get_rd_addr!(inst.inst);

        let mut tcg_list = vec![];

        let source1 = self.tcg_temp_new();
        tcg_list.push(TCGOp::tcg_get_gpr(source1, rd_addr));
        tcg_list.push(TCGOp::new_3op(TCGOpcode::SLL_64BIT, source1, source1, TCGv::new_imm(shamt as u64)));
        tcg_list.push(TCGOp::tcg_set_gpr(rd_addr, source1));
        self.tcg_temp_free(source1);

        (false, tcg_list)
    }
    pub fn translate_c_fldsp (&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) { (false, vec![]) }
    pub fn translate_c_lwsp  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm = (((inst.inst >> 12) & 0x1) << 5) |
                      (((inst.inst >>  4) & 0x7) << 2) |
                      (((inst.inst >>  2) & 0x3) << 6);

        self.translate_raw_load(2,
                                imm as u64,
                                get_rd_addr!(inst.inst),
                                inst, TCGOpcode::LOAD_32BIT, CALL_HELPER_IDX::CALL_LOAD32_IDX)
    }
    pub fn translate_c_flwsp (&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) { (false, vec![]) }
    pub fn translate_c_ldsp  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm = (((inst.inst >> 12) & 0x1) << 5) |
                      (((inst.inst >>  4) & 0x7) << 2) |
                      (((inst.inst >>  2) & 0x3) << 6);

        self.translate_raw_load(2,
                                imm as u64,
                                get_rd_addr!(inst.inst),
                                inst, TCGOpcode::LOAD_64BIT, CALL_HELPER_IDX::CALL_LOAD64_IDX)
    }
    pub fn translate_c_jr    (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rs1_addr = get_rd_addr!(inst.inst);   // src1 is 11-7 bitfield

        let mut tcg_lists = vec![];

        let source1 = self.tcg_temp_new();
        let dest = self.tcg_temp_new();
        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_lists.push(TCGOp::tcg_get_gpr(dest, 0));

        tcg_lists.push(TCGOp::new_3op(TCGOpcode::JMPR, dest, source1, TCGv::new_imm(0)));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));

        self.tcg_temp_free(source1);
        self.tcg_temp_free(dest);

        (true, tcg_lists)
    }
    pub fn translate_c_mv    (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rd_addr   = (inst.inst >> 7) & 0x1f;
        let rs2_addr  = (inst.inst >> 2) & 0x1f;

        let mut tcg_lists = vec![];

        let zero_tmp = self.tcg_temp_new();
        let val_tmp = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(zero_tmp, 0));
        tcg_lists.push(TCGOp::tcg_get_gpr(val_tmp, rs2_addr));

        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_64BIT, val_tmp, val_tmp, zero_tmp));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, val_tmp));

        self.tcg_temp_free(val_tmp);
        self.tcg_temp_free(zero_tmp);

        (false, tcg_lists)
    }


    pub fn translate_c_ebreak (&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) { (false, vec![]) }
    pub fn translate_c_jalr  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rs1_addr = get_rd_addr!(inst.inst);   // src1 is 11-7 bitfield

        let mut tcg_lists = vec![];

        let source1 = self.tcg_temp_new();
        let dest = self.tcg_temp_new();
        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rs1_addr));

        let zero = self.tcg_temp_new();
        tcg_lists.push(TCGOp::tcg_get_gpr(zero, 0));
        let next_pc = TCGv::new_imm((inst.addr as u64).wrapping_add(2));
        tcg_lists.push(TCGOp::new_2op(TCGOpcode::MOV_IMM_64BIT, dest, next_pc));
        self.tcg_temp_free(zero);
        tcg_lists.push(TCGOp::tcg_set_gpr(1, dest));

        tcg_lists.push(TCGOp::new_3op(TCGOpcode::JMPR, dest, source1, TCGv::new_imm(0)));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));

        self.tcg_temp_free(source1);
        self.tcg_temp_free(dest);

        (true, tcg_lists)
    }

    pub fn translate_c_add   (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let rd_addr   = (inst.inst >> 7) & 0x1f;
        let rs2_addr  = (inst.inst >> 2) & 0x1f;

        let mut tcg_lists = vec![];

        let rd_tmp = self.tcg_temp_new();
        let rs2_tmp = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(rd_tmp, rd_addr));
        tcg_lists.push(TCGOp::tcg_get_gpr(rs2_tmp, rs2_addr));

        tcg_lists.push(TCGOp::new_3op(TCGOpcode::ADD_64BIT, rd_tmp, rd_tmp, rs2_tmp));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, rd_tmp));

        self.tcg_temp_free(rd_tmp);
        self.tcg_temp_free(rs2_tmp);

        (false, tcg_lists)
    }
    pub fn translate_c_fsdsp (&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) { (false, vec![]) }
    pub fn translate_c_swsp  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm = (((inst.inst >> 9) & 0xf) << 2) |
                      (((inst.inst >> 7) & 0x3) << 6);

        self.translate_raw_store(2,
                          imm as u64,
                          (inst.inst >> 2) & 0x1f,
                                 inst, TCGOpcode::STORE_32BIT, CALL_HELPER_IDX::CALL_STORE32_IDX)
    }
    pub fn translate_c_fswsp (&mut self, _inst: &InstrInfo) -> (bool, Vec<TCGOp>) { (false, vec![]) }
    pub fn translate_c_sdsp  (&mut self, inst: &InstrInfo) -> (bool, Vec<TCGOp>) {
        let imm = (((inst.inst >> 9) & 0xf) << 2) |
                      (((inst.inst >> 7) & 0x3) << 6);

        self.translate_raw_store(2,
                          imm as u64,
                          (inst.inst >> 2) & 0x1f,
                          inst, TCGOpcode::STORE_64BIT, CALL_HELPER_IDX::CALL_STORE64_IDX)
    }


}
