use std::cell::RefCell;
use std::rc::Rc;

use super::super::super::tcg::tcg::{TCGOp, TCGOpcode, TCGv, TCGLabel};
use super::super::super::instr_info::InstrInfo;
use super::riscv::{CALL_HELPER_IDX, CallFcvtIdx};

use super::super::super::get_rs1_addr;
use super::super::super::get_rs2_addr;
use super::super::super::get_rs3_addr;
use super::super::super::get_rd_addr;

use super::riscv::TranslateRiscv;

impl TranslateRiscv {
    pub fn translate_fld(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        

        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));

        let label = Rc::new(RefCell::new(TCGLabel::new()));

        let tcg_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_FLOAT_LOAD64_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);

        let zero = Box::new(TCGv::new_reg(0 as u64));        
        let dummy_addr = Box::new(TCGv::new_imm(0));

        let result_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label));
        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_set_label = TCGOp::new_label(Rc::clone(&label));

        vec![tcg_call_op, result_cmp_op, exit_tb, tcg_set_label]

        // Self::translate_float_rri(TCGOpcode::LOAD_FLOAT_64BIT, inst)
    }
    pub fn translate_flw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        

        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));

        let label = Rc::new(RefCell::new(TCGLabel::new()));

        let tcg_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_FLOAT_LOAD32_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);

        let zero = Box::new(TCGv::new_reg(0 as u64));        
        let dummy_addr = Box::new(TCGv::new_imm(0));

        let result_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label));
        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_set_label = TCGOp::new_label(Rc::clone(&label));

        vec![tcg_call_op, result_cmp_op, exit_tb, tcg_set_label]

        // Self::translate_float_rri(TCGOpcode::LOAD_FLOAT_32BIT, inst)
    }
    pub fn translate_fsd(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        

        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));

        let label = Rc::new(RefCell::new(TCGLabel::new()));

        let tcg_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_FLOAT_STORE64_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);

        let zero = Box::new(TCGv::new_reg(0 as u64));        
        let dummy_addr = Box::new(TCGv::new_imm(0));

        let result_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label));
        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_set_label = TCGOp::new_label(Rc::clone(&label));

        vec![tcg_call_op, result_cmp_op, exit_tb, tcg_set_label]

        // Self::translate_store(TCGOpcode::STORE_FLOAT_64BIT, inst)
    }
    pub fn translate_fsw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        

        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));

        let label = Rc::new(RefCell::new(TCGLabel::new()));

        let tcg_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_FLOAT_STORE32_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);

        let zero = Box::new(TCGv::new_reg(0 as u64));        
        let dummy_addr = Box::new(TCGv::new_imm(0));

        let result_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label));
        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_set_label = TCGOp::new_label(Rc::clone(&label));

        vec![tcg_call_op, result_cmp_op, exit_tb, tcg_set_label]

        // Self::translate_store(TCGOpcode::STORE_FLOAT_32BIT, inst)
    }

    pub fn translate_fadd_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fadd_d =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FADD_D_IDX as usize, *rd, *rs1, *rs2);
        vec![fadd_d]
    }

    pub fn translate_fsub_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fsub_d =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FSUB_D_IDX as usize, *rd, *rs1, *rs2);
        vec![fsub_d]
    }

    pub fn translate_fmul_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fmul_d =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMUL_D_IDX as usize, *rd, *rs1, *rs2);
        vec![fmul_d]
    }

    pub fn translate_fmadd_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rs3 = Box::new(TCGv::new_reg(get_rs3_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FMADD_D_IDX as usize, *rd, *rs1, *rs2, *rs3);        vec![fop]
    }

    pub fn translate_fmsub_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rs3 = Box::new(TCGv::new_reg(get_rs3_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FMSUB_D_IDX as usize, *rd, *rs1, *rs2, *rs3);        vec![fop]
    }

    pub fn translate_fnmsub_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rs3 = Box::new(TCGv::new_reg(get_rs3_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FNMSUB_D_IDX as usize, *rd, *rs1, *rs2, *rs3);        vec![fop]
    }

    pub fn translate_fnmadd_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rs3 = Box::new(TCGv::new_reg(get_rs3_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FNMADD_D_IDX as usize, *rd, *rs1, *rs2, *rs3);        vec![fop]
    }

    pub fn translate_fdiv_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fdiv_d =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FDIV_D_IDX as usize, *rd, *rs1, *rs2);
        vec![fdiv_d]
    }

    pub fn translate_fsqrt_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fdiv_d =
            TCGOp::new_helper_call_arg2(CALL_HELPER_IDX::CALL_FSQRT_D_IDX as usize, *rd, *rs1);
        vec![fdiv_d]
    }

    pub fn translate_fmv_x_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let mov_x_d = TCGOp::new_2op(TCGOpcode::MOVE_TO_INT_FROM_FLOAT, *rd, *rs1);
        vec![mov_x_d]
    }


    pub fn translate_fmv_d_x(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let mov_x_d = TCGOp::new_2op(TCGOpcode::MOVE_TO_FLOAT_FROM_INT, *rd, *rs1);
        vec![mov_x_d]
    }

    pub fn translate_feq_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FEQ_D_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }
    pub fn translate_flt_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FLT_D_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }
    pub fn translate_fle_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FLE_D_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fclass_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg2(CALL_HELPER_IDX::CALL_FCLASS_D_IDX as usize, *rd, *rs1);
        vec![op]
    }

    pub fn translate_fmax_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMAX_D_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fmin_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMIN_D_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnj_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_3op(TCGOpcode::SGNJ_64BIT, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnjn_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_3op(TCGOpcode::SGNJN_64BIT, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnjx_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_3op(TCGOpcode::SGNJX_64BIT, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fadd_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fadd_s =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FADD_S_IDX as usize, *rd, *rs1, *rs2);
        vec![fadd_s]
    }

    pub fn translate_fsub_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fsub_s =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FSUB_S_IDX as usize, *rd, *rs1, *rs2);
        vec![fsub_s]
    }

    pub fn translate_fmul_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fmul_s =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMUL_S_IDX as usize, *rd, *rs1, *rs2);
        vec![fmul_s]
    }

    pub fn translate_fmadd_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rs3 = Box::new(TCGv::new_reg(get_rs3_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FMADD_S_IDX as usize, *rd, *rs1, *rs2, *rs3);        
        vec![fop]
    }

    pub fn translate_fmsub_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rs3 = Box::new(TCGv::new_reg(get_rs3_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FMSUB_S_IDX as usize, *rd, *rs1, *rs2, *rs3);       
        vec![fop]
    }

    pub fn translate_fnmsub_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rs3 = Box::new(TCGv::new_reg(get_rs3_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FNMSUB_S_IDX as usize, *rd, *rs1, *rs2, *rs3);
        vec![fop]
    }

    pub fn translate_fnmadd_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rs3 = Box::new(TCGv::new_reg(get_rs3_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FNMADD_S_IDX as usize, *rd, *rs1, *rs2, *rs3);
        vec![fop]
    }

    pub fn translate_fdiv_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fdiv_s =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FDIV_S_IDX as usize, *rd, *rs1, *rs2);
        vec![fdiv_s]
    }

    pub fn translate_fsqrt_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let fdiv_s =
            TCGOp::new_helper_call_arg2(CALL_HELPER_IDX::CALL_FSQRT_S_IDX as usize, *rd, *rs1);
        vec![fdiv_s]
    }

    pub fn translate_fmv_x_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let mov_x_s = TCGOp::new_2op(TCGOpcode::MOVE_TO_INT_FROM_FLOAT, *rd, *rs1);
        vec![mov_x_s]
    }


    pub fn translate_fmv_s_x(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let mov_x_s = TCGOp::new_2op(TCGOpcode::MOVE_TO_FLOAT_FROM_INT, *rd, *rs1);
        vec![mov_x_s]
    }

    pub fn translate_feq_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FEQ_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }
    pub fn translate_flt_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FLT_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }
    pub fn translate_fle_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FLE_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fclass_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg2(CALL_HELPER_IDX::CALL_FCLASS_S_IDX as usize, *rd, *rs1);
        vec![op]
    }

    pub fn translate_fmv_x_w(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let mov_x_d = TCGOp::new_2op(TCGOpcode::MOVE_TO_INT_FROM_FLOAT_32BIT, *rd, *rs1);
        vec![mov_x_d]
    }


    pub fn translate_fmv_w_x(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let mov_x_d = TCGOp::new_2op(TCGOpcode::MOVE_TO_FLOAT_FROM_INT_32BIT, *rd, *rs1);
        vec![mov_x_d]
    }

    pub fn translate_fmax_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMAX_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fmin_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMIN_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnj_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FSGNJ_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnjn_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FSGNJN_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnjx_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1 = Box::new(TCGv::new_reg(get_rs1_addr!(inst.inst) as u64));
        let rs2 = Box::new(TCGv::new_reg(get_rs2_addr!(inst.inst)as u64));
        let rd = Box::new(TCGv::new_reg(get_rd_addr!(inst.inst) as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FSGNJX_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fcvt_w_s (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::W_S as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx, rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_wu_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::WU_S as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_s_w (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::S_W as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_s_wu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::S_WU as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_s_d (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::S_D as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_d_s (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::D_S as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_w_d (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::W_D as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_wu_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::WU_D as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_d_w (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::D_W as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_d_wu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::D_WU as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_l_s (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::L_S as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_lu_s(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::LU_S as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_s_l (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::S_L as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_s_lu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::S_LU as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_l_d (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::L_D as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_lu_d(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::LU_D as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_d_l (&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::D_L as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }

    pub fn translate_fcvt_d_lu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> { 
        let mut tcg_lists = vec![];
        let rs1 = TCGv::new_reg(get_rs1_addr!(inst.inst) as u64);
        let rd  = TCGv::new_reg(get_rd_addr!(inst.inst) as u64);
        let fcvt_helper_idx = TCGv::new_imm(CallFcvtIdx::D_LU as u64);
        tcg_lists.push(TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FCVT_IDX as usize, fcvt_helper_idx,  rd, rs1));
        tcg_lists 
    }



}
