use super::super::super::tcg::tcg::{TCGLabel, TCGOp, TCGOpcode, TCGv};
use std::cell::RefCell;
use std::rc::Rc;

use super::super::super::instr_info::InstrInfo;
use super::riscv_inst_id::RiscvInstId;

#[derive(PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)]
pub enum ExceptCode {
    InstAddrMisalign = 0,
    InstAccessFault = 1,
    IllegalInst = 2,
    Breakpoint = 3,
    LoadAddrMisalign = 4,
    LoadAccessFault = 5,
    StoreAddrMisalign = 6,
    StoreAccessFault = 7,
    EcallFromUMode = 8,
    EcallFromSMode = 9,
    EcallFromHMode = 10,
    EcallFromMMode = 11,
    InstPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
}

#[allow(non_camel_case_types)]
pub enum CALL_HELPER_IDX {
    CALL_CSRRW_IDX = 0,
    CALL_CSRRS_IDX = 1,
    CALL_CSRRC_IDX = 2,
    CALL_CSRRWI_IDX = 3,
    CALL_CSRRSI_IDX = 4,
    CALL_CSRRCI_IDX = 5,
    CALL_MRET_IDX = 6,
    CALL_ECALL_IDX = 7,
    CALL_FADD_D_IDX = 8,
    CALL_FSUB_D_IDX = 9,
    CALL_FMUL_D_IDX = 10,
    CALL_FDIV_D_IDX = 11,
    CALL_FMADD_D_IDX = 12,
    CALL_FMSUB_D_IDX = 13,
    CALL_FNMSUB_D_IDX = 14,
    CALL_FNMADD_D_IDX = 15,
    CALL_FSQRT_D_IDX = 16,
    CALL_FEQ_D_IDX = 17,
    CALL_FLT_D_IDX = 18,
    CALL_FLE_D_IDX = 19,
    CALL_FCLASS_D_IDX = 20,
    CALL_FADD_S_IDX = 21,
    CALL_FSUB_S_IDX = 22,
    CALL_FMUL_S_IDX = 23,
    CALL_FDIV_S_IDX = 24,
    CALL_FMADD_S_IDX = 25,
    CALL_FMSUB_S_IDX = 26,
    CALL_FNMSUB_S_IDX = 27,
    CALL_FNMADD_S_IDX = 28,
    CALL_FSQRT_S_IDX = 29,
    CALL_FEQ_S_IDX = 30,
    CALL_FLT_S_IDX = 31,
    CALL_FLE_S_IDX = 32,
    CALL_FCLASS_S_IDX = 33,
    CALL_FMAX_D_IDX = 34,
    CALL_FMIN_D_IDX = 35,
    CALL_FMAX_S_IDX = 36,
    CALL_FMIN_S_IDX = 37,
    CALL_FSGNJ_S_IDX = 38,
    CALL_FSGNJN_S_IDX = 39,
    CALL_FSGNJX_S_IDX = 40,
}

#[macro_export]
macro_rules! get_rs1_addr {
    ($inst:expr) => {
        ($inst >> 15) & 0x1f
    };
}

#[macro_export]
macro_rules! get_rs2_addr {
    ($inst:expr) => {
        ($inst >> 20) & 0x1f
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! get_rs3_addr {
    ($inst:expr) => {
        ($inst >> 27) & 0x1f
    };
}

#[macro_export]
macro_rules! get_rd_addr {
    ($inst:expr) => {
        ($inst >> 7) & 0x1f
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! get_imm12 {
    ($inst:expr) => {
        ($inst >> 20) as u64
    };
}

#[macro_export]
macro_rules! get_sb_field {
    ($inst:expr) => {
        ((($inst as u64 >> 7) & 0x01) << 11)
            | ((($inst as u64 >> 8) & 0x0f) << 1)
            | ((($inst as u64 >> 25) & 0x3f) << 5)
            | ((($inst as u64 >> 31) & 0x01) << 12) as u64
    };
}

#[macro_export]
macro_rules! extract_j_field {
    ($inst:expr) => {
        ((((($inst >> 21) & 0x3ff) << 1)
            | ((($inst >> 20) & 0x001) << 11)
            | ((($inst >> 12) & 0x0ff) << 12)
            | ((($inst >> 31) & 0x001) << 20)) as i32) as u64
    };
}

macro_rules! get_s_imm_field {
    ($inst:expr) => {
        ((((($inst as u64 >> 25) & 0x7f) << 5) | ($inst as u64 >> 7 & 0x1f)) as i32) as u64
    };
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum PrivMode {
    User,
    Supervisor,
    Hypervisor,
    Machine,
}

impl PrivMode {
    pub fn from_u8(n: u8) -> PrivMode {
        match n {
            0 => PrivMode::User,
            1 => PrivMode::Supervisor,
            2 => PrivMode::Hypervisor,
            3 => PrivMode::Machine,
            _ => PrivMode::Machine,
        }
    }
}

pub struct TranslateRiscv;

impl TranslateRiscv {
    pub fn translate(id: RiscvInstId, inst: &InstrInfo) -> Vec<TCGOp> {
        return match id {
            RiscvInstId::ADDI => TranslateRiscv::translate_addi(inst),
            RiscvInstId::ADD => TranslateRiscv::translate_add(inst),
            RiscvInstId::SUB => TranslateRiscv::translate_sub(inst),
            RiscvInstId::AND => TranslateRiscv::translate_and(inst),
            RiscvInstId::OR => TranslateRiscv::translate_or(inst),
            RiscvInstId::XOR => TranslateRiscv::translate_xor(inst),
            RiscvInstId::ANDI => TranslateRiscv::translate_andi(inst),
            RiscvInstId::ORI => TranslateRiscv::translate_ori(inst),
            RiscvInstId::XORI => TranslateRiscv::translate_xori(inst),
            RiscvInstId::ADDW => TranslateRiscv::translate_addw(inst),
            RiscvInstId::SUBW => TranslateRiscv::translate_subw(inst),

            RiscvInstId::ADDIW => TranslateRiscv::translate_addiw(inst),

            RiscvInstId::LUI => TranslateRiscv::translate_lui(inst),
            RiscvInstId::AUIPC => TranslateRiscv::translate_auipc(inst),

            RiscvInstId::BEQ => TranslateRiscv::translate_beq(inst),
            RiscvInstId::BNE => TranslateRiscv::translate_bne(inst),
            RiscvInstId::BLT => TranslateRiscv::translate_blt(inst),
            RiscvInstId::BGE => TranslateRiscv::translate_bge(inst),
            RiscvInstId::BLTU => TranslateRiscv::translate_bltu(inst),
            RiscvInstId::BGEU => TranslateRiscv::translate_bgeu(inst),

            RiscvInstId::LD => TranslateRiscv::translate_ld(inst),
            RiscvInstId::LW => TranslateRiscv::translate_lw(inst),
            RiscvInstId::LH => TranslateRiscv::translate_lh(inst),
            RiscvInstId::LB => TranslateRiscv::translate_lb(inst),
            RiscvInstId::LWU => TranslateRiscv::translate_lwu(inst),
            RiscvInstId::LHU => TranslateRiscv::translate_lhu(inst),
            RiscvInstId::LBU => TranslateRiscv::translate_lbu(inst),
            RiscvInstId::SD => TranslateRiscv::translate_sd(inst),
            RiscvInstId::SW => TranslateRiscv::translate_sw(inst),
            RiscvInstId::SH => TranslateRiscv::translate_sh(inst),
            RiscvInstId::SB => TranslateRiscv::translate_sb(inst),

            RiscvInstId::SLLI => TranslateRiscv::translate_slli(inst),
            RiscvInstId::SRLI => TranslateRiscv::translate_srli(inst),
            RiscvInstId::SRAI => TranslateRiscv::translate_srai(inst),
            RiscvInstId::SLL => TranslateRiscv::translate_sll(inst),
            RiscvInstId::SRL => TranslateRiscv::translate_srl(inst),
            RiscvInstId::SRA => TranslateRiscv::translate_sra(inst),

            RiscvInstId::SLLIW => TranslateRiscv::translate_slliw(inst),
            RiscvInstId::SRLIW => TranslateRiscv::translate_srliw(inst),
            RiscvInstId::SRAIW => TranslateRiscv::translate_sraiw(inst),
            RiscvInstId::SLLW => TranslateRiscv::translate_sllw(inst),
            RiscvInstId::SRLW => TranslateRiscv::translate_srlw(inst),
            RiscvInstId::SRAW => TranslateRiscv::translate_sraw(inst),

            RiscvInstId::SLT => TranslateRiscv::translate_slt(inst),
            RiscvInstId::SLTI => TranslateRiscv::translate_slti(inst),
            RiscvInstId::SLTU => TranslateRiscv::translate_sltu(inst),
            RiscvInstId::SLTIU => TranslateRiscv::translate_sltiu(inst),

            RiscvInstId::JALR => TranslateRiscv::translate_jalr(inst),
            RiscvInstId::JAL => TranslateRiscv::translate_jal(inst),

            RiscvInstId::CSRRS => TranslateRiscv::translate_csrrs(inst),
            RiscvInstId::CSRRW => TranslateRiscv::translate_csrrw(inst),
            RiscvInstId::CSRRC => TranslateRiscv::translate_csrrc(inst),
            RiscvInstId::CSRRSI => TranslateRiscv::translate_csrrsi(inst),
            RiscvInstId::CSRRWI => TranslateRiscv::translate_csrrwi(inst),
            RiscvInstId::CSRRCI => TranslateRiscv::translate_csrrci(inst),

            RiscvInstId::FENCE => TranslateRiscv::translate_fence(inst),
            RiscvInstId::MRET => TranslateRiscv::translate_mret(inst),
            RiscvInstId::ECALL => TranslateRiscv::translate_ecall(inst),

            RiscvInstId::FLD => TranslateRiscv::translate_fld(inst),
            RiscvInstId::FLW => TranslateRiscv::translate_flw(inst),
            RiscvInstId::FSD => TranslateRiscv::translate_fsd(inst),
            RiscvInstId::FSW => TranslateRiscv::translate_fsw(inst),

            RiscvInstId::FADD_D => TranslateRiscv::translate_fadd_d(inst),
            RiscvInstId::FSUB_D => TranslateRiscv::translate_fsub_d(inst),
            RiscvInstId::FMUL_D => TranslateRiscv::translate_fmul_d(inst),
            RiscvInstId::FDIV_D => TranslateRiscv::translate_fdiv_d(inst),

            RiscvInstId::FMADD_D => TranslateRiscv::translate_fmadd_d(inst),
            RiscvInstId::FMSUB_D => TranslateRiscv::translate_fmsub_d(inst),
            RiscvInstId::FNMSUB_D => TranslateRiscv::translate_fnmsub_d(inst),
            RiscvInstId::FNMADD_D => TranslateRiscv::translate_fnmadd_d(inst),

            RiscvInstId::FSQRT_D => TranslateRiscv::translate_fsqrt_d(inst),

            RiscvInstId::FMV_X_D => TranslateRiscv::translate_fmv_x_d(inst),
            RiscvInstId::FMV_D_X => TranslateRiscv::translate_fmv_d_x(inst),

            RiscvInstId::FEQ_D => TranslateRiscv::translate_feq_d(inst),
            RiscvInstId::FLT_D => TranslateRiscv::translate_flt_d(inst),
            RiscvInstId::FLE_D => TranslateRiscv::translate_fle_d(inst),
            RiscvInstId::FCLASS_D => TranslateRiscv::translate_fclass_d(inst),

            RiscvInstId::FMIN_D => TranslateRiscv::translate_fmin_d(inst),
            RiscvInstId::FMAX_D => TranslateRiscv::translate_fmax_d(inst),

            RiscvInstId::FSGNJ_D  => TranslateRiscv::translate_fsgnj_d(inst),
            RiscvInstId::FSGNJN_D => TranslateRiscv::translate_fsgnjn_d(inst),   
            RiscvInstId::FSGNJX_D => TranslateRiscv::translate_fsgnjx_d(inst),

            RiscvInstId::FADD_S => TranslateRiscv::translate_fadd_s(inst),
            RiscvInstId::FSUB_S => TranslateRiscv::translate_fsub_s(inst),
            RiscvInstId::FMUL_S => TranslateRiscv::translate_fmul_s(inst),
            RiscvInstId::FDIV_S => TranslateRiscv::translate_fdiv_s(inst),

            RiscvInstId::FMADD_S => TranslateRiscv::translate_fmadd_s(inst),
            RiscvInstId::FMSUB_S => TranslateRiscv::translate_fmsub_s(inst),
            RiscvInstId::FNMSUB_S => TranslateRiscv::translate_fnmsub_s(inst),
            RiscvInstId::FNMADD_S => TranslateRiscv::translate_fnmadd_s(inst),

            RiscvInstId::FSQRT_S => TranslateRiscv::translate_fsqrt_s(inst),

            RiscvInstId::FMV_X_W => TranslateRiscv::translate_fmv_x_w(inst),
            RiscvInstId::FMV_W_X => TranslateRiscv::translate_fmv_w_x(inst),

            RiscvInstId::FEQ_S => TranslateRiscv::translate_feq_s(inst),
            RiscvInstId::FLT_S => TranslateRiscv::translate_flt_s(inst),
            RiscvInstId::FLE_S => TranslateRiscv::translate_fle_s(inst),
            RiscvInstId::FCLASS_S => TranslateRiscv::translate_fclass_s(inst),

            RiscvInstId::FMIN_S => TranslateRiscv::translate_fmin_s(inst),
            RiscvInstId::FMAX_S => TranslateRiscv::translate_fmax_s(inst),

            RiscvInstId::FSGNJ_S  => TranslateRiscv::translate_fsgnj_s(inst),
            RiscvInstId::FSGNJN_S => TranslateRiscv::translate_fsgnjn_s(inst),   
            RiscvInstId::FSGNJX_S => TranslateRiscv::translate_fsgnjx_s(inst),

            other_id => panic!("InstID={:?} : Not supported these instructions.", other_id),
        };
    }

    pub fn translate_rrr(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        if rd_addr != 0 {
            let tcg_inst = TCGOp::new_3op(op, *rd, *rs1, *rs2);
            return vec![tcg_inst];
        } else {
            return vec![];
        }
    }

    pub fn translate_rri(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        if rd_addr != 0 {
            let tcg_inst = TCGOp::new_3op(op, *rd, *rs1, *imm);
            return vec![tcg_inst];
        } else {
            return vec![];
        }
    }

    pub fn translate_shift_i(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst >> 20) & 0x3f) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        if rd_addr != 0 {
            let tcg_inst = TCGOp::new_3op(op, *rd, *rs1, *imm);
            return vec![tcg_inst];
        } else {
            return vec![];
        }
    }

    pub fn translate_store(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = get_s_imm_field!(inst.inst);
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let imm_const = ((imm_const as i32) << (32 - 12)) >> (32 - 12);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const as i64 as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));

        let tcg_inst = TCGOp::new_3op(op, *rs1, *rs2, *imm);

        vec![tcg_inst]
    }

    pub fn translate_branch(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let target: u64 = get_sb_field!(inst.inst) + inst.addr;

        let target = ((target as i32) << (32 - 13)) >> (32 - 13);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let addr = Box::new(TCGv::new_imm(target as i32 as u64));

        let label = Rc::new(RefCell::new(TCGLabel::new()));

        let tcg_inst = TCGOp::new_4op(op, *rs1, *rs2, *addr, Rc::clone(&label));
        let tcg_true_tb = TCGOp::new_goto_tb(TCGv::new_imm(inst.addr + 4));
        let tcg_set_label = TCGOp::new_label(Rc::clone(&label));
        let tcg_false_tb = TCGOp::new_goto_tb(TCGv::new_imm(target as i32 as u64));

        vec![tcg_inst, tcg_true_tb, tcg_set_label, tcg_false_tb]
    }

    pub fn translate_float_rri(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = TCGOp::new_3op(op, *rd, *rs1, *imm);
        return vec![tcg_inst];
    }



}
