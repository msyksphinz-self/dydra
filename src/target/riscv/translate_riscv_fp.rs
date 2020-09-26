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
        Self::translate_store(TCGOpcode::STORE_FLOAT_64BIT, inst)
    }
    pub fn translate_fsw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_store(TCGOpcode::STORE_FLOAT_32BIT, inst)
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


    pub fn translate_fmv_d_x(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let mov_x_d = TCGOp::new_2op(TCGOpcode::MOVE_TO_FLOAT_FROM_INT, *rd, *rs1);
        vec![mov_x_d]
    }

    pub fn translate_feq_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FEQ_D_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }
    pub fn translate_flt_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FLT_D_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }
    pub fn translate_fle_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FLE_D_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fclass_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg2(CALL_HELPER_IDX::CALL_FCLASS_D_IDX as usize, *rd, *rs1);
        vec![op]
    }

    pub fn translate_fmax_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMAX_D_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fmin_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMIN_D_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnj_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_3op(TCGOpcode::SGNJ_64BIT, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnjn_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_3op(TCGOpcode::SGNJN_64BIT, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnjx_d(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_3op(TCGOpcode::SGNJX_64BIT, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fadd_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fadd_s =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FADD_S_IDX as usize, *rd, *rs1, *rs2);
        vec![fadd_s]
    }

    pub fn translate_fsub_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fsub_s =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FSUB_S_IDX as usize, *rd, *rs1, *rs2);
        vec![fsub_s]
    }

    pub fn translate_fmul_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fmul_s =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMUL_S_IDX as usize, *rd, *rs1, *rs2);
        vec![fmul_s]
    }

    pub fn translate_fmadd_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rs3_addr: usize = get_rs3_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rs3 = Box::new(TCGv::new_reg(rs3_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FMADD_S_IDX as usize,
            *rd,
            *rs1,
            *rs2,
            *rs3,
        );
        vec![fop]
    }

    pub fn translate_fmsub_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rs3_addr: usize = get_rs3_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rs3 = Box::new(TCGv::new_reg(rs3_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FMSUB_S_IDX as usize,
            *rd,
            *rs1,
            *rs2,
            *rs3,
        );
        vec![fop]
    }

    pub fn translate_fnmsub_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rs3_addr: usize = get_rs3_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rs3 = Box::new(TCGv::new_reg(rs3_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FNMSUB_S_IDX as usize,
            *rd,
            *rs1,
            *rs2,
            *rs3,
        );
        vec![fop]
    }

    pub fn translate_fnmadd_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rs3_addr: usize = get_rs3_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rs3 = Box::new(TCGv::new_reg(rs3_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fop = TCGOp::new_helper_call_arg4(
            CALL_HELPER_IDX::CALL_FNMADD_S_IDX as usize,
            *rd,
            *rs1,
            *rs2,
            *rs3,
        );
        vec![fop]
    }

    pub fn translate_fdiv_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fdiv_s =
            TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FDIV_S_IDX as usize, *rd, *rs1, *rs2);
        vec![fdiv_s]
    }

    pub fn translate_fsqrt_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let fdiv_s =
            TCGOp::new_helper_call_arg2(CALL_HELPER_IDX::CALL_FSQRT_S_IDX as usize, *rd, *rs1);
        vec![fdiv_s]
    }

    pub fn translate_fmv_x_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let mov_x_s = TCGOp::new_2op(TCGOpcode::MOVE_TO_INT_FROM_FLOAT, *rd, *rs1);
        vec![mov_x_s]
    }


    pub fn translate_fmv_s_x(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let mov_x_s = TCGOp::new_2op(TCGOpcode::MOVE_TO_FLOAT_FROM_INT, *rd, *rs1);
        vec![mov_x_s]
    }

    pub fn translate_feq_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FEQ_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }
    pub fn translate_flt_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FLT_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }
    pub fn translate_fle_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FLE_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fclass_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg2(CALL_HELPER_IDX::CALL_FCLASS_S_IDX as usize, *rd, *rs1);
        vec![op]
    }

    pub fn translate_fmv_x_w(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let mov_x_d = TCGOp::new_2op(TCGOpcode::MOVE_TO_INT_FROM_FLOAT_32BIT, *rd, *rs1);
        vec![mov_x_d]
    }


    pub fn translate_fmv_w_x(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let mov_x_d = TCGOp::new_2op(TCGOpcode::MOVE_TO_FLOAT_FROM_INT_32BIT, *rd, *rs1);
        vec![mov_x_d]
    }

    pub fn translate_fmax_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMAX_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fmin_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FMIN_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnj_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FSGNJ_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnjn_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FSGNJN_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

    pub fn translate_fsgnjx_s(inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_FSGNJX_S_IDX as usize, *rd, *rs1, *rs2);
        vec![op]
    }

}
