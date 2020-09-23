use super::super::super::tcg::tcg::{TCGOp, TCGOpcode, TCGv};
use super::super::super::instr_info::InstrInfo;
use super::riscv::CALL_HELPER_IDX;

use super::super::super::get_rs1_addr;
use super::super::super::get_rs2_addr;
use super::super::super::get_rs3_addr;
use super::super::super::get_rd_addr;

use super::riscv::TranslateRiscv;

impl TranslateRiscv {
    pub fn translate_fld(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_float_rri(TCGOpcode::LOAD_FLOAT_64BIT, inst)
    }
    pub fn translate_flw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_float_rri(TCGOpcode::LOAD_FLOAT_32BIT, inst)
    }
    pub fn translate_fsd(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_float_rri(TCGOpcode::STORE_FLOAT_64BIT, inst)
    }
    pub fn translate_fsw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_float_rri(TCGOpcode::STORE_FLOAT_32BIT, inst)
    }

    pub fn translate_fadd_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fadd_d =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FADD_D_IDX as usize, *rd, *rs1, *rs2);
        vec![fadd_d]
    }

    pub fn translate_fsub_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fsub_d =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FSUB_D_IDX as usize, *rd, *rs1, *rs2);
        vec![fsub_d]
    }

    pub fn translate_fmul_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fmul_d =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMUL_D_IDX as usize, *rd, *rs1, *rs2);
        vec![fmul_d]
    }

    pub fn translate_fmadd_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rs3_addr: usize = get_rs3_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rs3 = Box::new(TCGv::new_reg(rs3_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FMADD_D_IDX as usize,
            *rd,
            *rs1,
            *rs2,
            *rs3,
        );
        vec![fop]
    }

    pub fn translate_fmsub_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rs3_addr: usize = get_rs3_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rs3 = Box::new(TCGv::new_reg(rs3_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FMSUB_D_IDX as usize,
            *rd,
            *rs1,
            *rs2,
            *rs3,
        );
        vec![fop]
    }

    pub fn translate_fnmsub_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rs3_addr: usize = get_rs3_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rs3 = Box::new(TCGv::new_reg(rs3_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FNMSUB_D_IDX as usize,
            *rd,
            *rs1,
            *rs2,
            *rs3,
        );
        vec![fop]
    }

    pub fn translate_fnmadd_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rs3_addr: usize = get_rs3_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rs3 = Box::new(TCGv::new_reg(rs3_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FNMADD_D_IDX as usize,
            *rd,
            *rs1,
            *rs2,
            *rs3,
        );
        vec![fop]
    }

    pub fn translate_fdiv_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fdiv_d =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FDIV_D_IDX as usize, *rd, *rs1, *rs2);
        vec![fdiv_d]
    }

    pub fn translate_fsqrt_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fdiv_d =
            TCGOp::new_helper_call_arg2(CALL_HELPER_IDX::CALL_FSQRT_D_IDX as usize, *rd, *rs1);
        vec![fdiv_d]
    }

    pub fn translate_fmv_x_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let mov_x_d = TCGOp::new_2op(TCGOpcode::MOVE_TO_INT_FROM_FLOAT, *rd, *rs1);
        vec![mov_x_d]
    }
}
