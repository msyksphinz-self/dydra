use super::tcg::{TCGLabel, TCGOp, TCGOpcode, TCGv};
use std::cell::RefCell;
use std::rc::Rc;

use super::instr_info::InstrInfo;
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
}

macro_rules! get_rs1_addr {
    ($inst:expr) => {
        ($inst >> 15) & 0x1f
    };
}

macro_rules! get_rs2_addr {
    ($inst:expr) => {
        ($inst >> 20) & 0x1f
    };
}

#[allow(unused_macros)]
macro_rules! get_rs3_addr {
    ($inst:expr) => {
        ($inst >> 27) & 0x1f
    };
}

macro_rules! get_rd_addr {
    ($inst:expr) => {
        ($inst >> 7) & 0x1f
    };
}

#[allow(unused_macros)]
macro_rules! get_imm12 {
    ($inst:expr) => {
        ($inst >> 20) as u64
    };
}

macro_rules! get_sb_field {
    ($inst:expr) => {
        ((($inst as u64 >> 7) & 0x01) << 11)
            | ((($inst as u64 >> 8) & 0x0f) << 1)
            | ((($inst as u64 >> 25) & 0x3f) << 5)
            | ((($inst as u64 >> 31) & 0x01) << 12) as u64
    };
}

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

            other_id => panic!("InstID={:?} : Not supported these instructions.", other_id),
        };
    }

    fn translate_rrr(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        if rd_addr != 0 {
            let tcg_inst = TCGOp::new_3op(op, *rd, *rs1, *rs2);
            return vec![tcg_inst]
        } else {
            return vec![]
        }
    }

    fn translate_rri(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        if rd_addr != 0 {
            let tcg_inst = TCGOp::new_3op(op, *rd, *rs1, *imm);
            return vec![tcg_inst]
        } else {
            return vec![]
        }
    }

    fn translate_shift_i(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst >> 20) & 0x3f) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        if rd_addr != 0 {
            let tcg_inst = TCGOp::new_3op(op, *rd, *rs1, *imm);
            return vec![tcg_inst]
        } else {
            return vec![]
        }
    }

    fn translate_store(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = get_s_imm_field!(inst.inst);
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let imm_const = ((imm_const as i32) << (32 - 12)) >> (32-12);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const as i64 as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));

        let tcg_inst = TCGOp::new_3op(op, *rs1, *rs2, *imm);

        vec![tcg_inst]
    }

    fn translate_branch(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
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

    pub fn translate_jalr(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = TCGOp::new_3op(TCGOpcode::JMPR, *rd, *rs1, *imm);

        vec![tcg_inst]
    }

    pub fn translate_jal(inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const = extract_j_field!(inst.inst);
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let imm_const = ((imm_const as i32) << (32 - 21)) >> (32 - 21);

        let imm = Box::new(TCGv::new_imm(
            ((imm_const as i64).wrapping_add(inst.addr as i64)) as u64,
        ));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let zero = Box::new(TCGv::new_reg(0));
        let next_pc = Box::new(TCGv::new_imm(inst.addr.wrapping_add(4)));
        let mov_inst = TCGOp::new_3op(TCGOpcode::ADD_64BIT, *rd, *zero, *next_pc);
        let tcg_inst = TCGOp::new_2op(TCGOpcode::JMPIM, *rd, *imm);

        if rd_addr == 0 {
            return vec![tcg_inst];
        } else {
            return vec![mov_inst, tcg_inst];
        }
    }

    pub fn translate_lui(inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const: u64 = (inst.inst as u64) & !0xfff;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(0));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));


        if rd_addr != 0 {
            let tcg_inst = TCGOp::new_3op(TCGOpcode::ADD_64BIT, *rd, *rs1, *imm);
            return vec![tcg_inst]
        } else {
            return vec![]
        }
    }

    pub fn translate_auipc(inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const: u64 = ((inst.inst as i64 as u64) & !0xfff).wrapping_add(inst.addr);
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(0));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        if rd_addr != 0 {
            let tcg_inst = TCGOp::new_3op(TCGOpcode::ADD_64BIT, *rd, *rs1, *imm);
            return vec![tcg_inst]
        } else {
            return vec![]
        }
    }

    pub fn translate_add(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::ADD_64BIT, inst)
    }
    pub fn translate_sub(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SUB_64BIT, inst)
    }
    pub fn translate_and(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::AND_64BIT, inst)
    }
    pub fn translate_or(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::OR_64BIT, inst)
    }
    pub fn translate_xor(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::XOR_64BIT, inst)
    }

    pub fn translate_addi(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::ADD_64BIT, inst)
    }
    pub fn translate_andi(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::AND_64BIT, inst)
    }
    pub fn translate_ori(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::OR_64BIT, inst)
    }
    pub fn translate_xori(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::XOR_64BIT, inst)
    }

    pub fn translate_addiw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::ADD_32BIT, inst)
    }
    pub fn translate_addw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::ADD_32BIT, inst)
    }
    pub fn translate_subw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SUB_32BIT, inst)
    }

    pub fn translate_beq(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::EQ_64BIT, inst)
    }
    pub fn translate_bne(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::NE_64BIT, inst)
    }
    pub fn translate_blt(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::LT_64BIT, inst)
    }
    pub fn translate_bge(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::GE_64BIT, inst)
    }
    pub fn translate_bltu(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::LTU_64BIT, inst)
    }
    pub fn translate_bgeu(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::GEU_64BIT, inst)
    }

    pub fn translate_ld(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::LOAD_64BIT, inst)
    }
    pub fn translate_lw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::LOAD_32BIT, inst)
    }
    pub fn translate_lh(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::LOAD_16BIT, inst)
    }
    pub fn translate_lb(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::LOAD_8BIT, inst)
    }
    pub fn translate_lwu(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::LOADU_32BIT, inst)
    }
    pub fn translate_lhu(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::LOADU_16BIT, inst)
    }
    pub fn translate_lbu(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::LOADU_8BIT, inst)
    }

    pub fn translate_sd(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_store(TCGOpcode::STORE_64BIT, inst)
    }
    pub fn translate_sw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_store(TCGOpcode::STORE_32BIT, inst)
    }
    pub fn translate_sh(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_store(TCGOpcode::STORE_16BIT, inst)
    }
    pub fn translate_sb(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_store(TCGOpcode::STORE_8BIT, inst)
    }

    pub fn translate_csrrw(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRW_IDX as usize, *rd, *rs1, *csr);
        vec![csr_op]
    }
    pub fn translate_csrrs(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRS_IDX as usize, *rd, *rs1, *csr);

        vec![csr_op]
    }
    pub fn translate_csrrc(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRC_IDX as usize, *rd, *rs1, *csr);
        vec![csr_op]
    }
    pub fn translate_csrrwi(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_imm: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_imm(rs1_imm as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRWI_IDX as usize, *rd, *rs1, *csr);

        vec![csr_op]
    }
    pub fn translate_csrrsi(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_imm: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_imm(rs1_imm as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRSI_IDX as usize, *rd, *rs1, *csr);
        vec![csr_op]
    }
    pub fn translate_csrrci(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_imm: usize = get_rs1_addr!(inst.inst) as usize;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        let csr_const: u64 = get_imm12!(inst.inst);

        let rs1 = Box::new(TCGv::new_imm(rs1_imm as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        let csr = Box::new(TCGv::new_imm(csr_const));

        let csr_op = TCGOp::new_helper_call_arg3(CALL_HELPER_IDX::CALL_CSRRCI_IDX as usize, *rd, *rs1, *csr);
        vec![csr_op]
    }

    pub fn translate_slli(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SLL_64BIT, inst)
    }
    pub fn translate_srli(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SRL_64BIT, inst)
    }
    pub fn translate_srai(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SRA_64BIT, inst)
    }
    pub fn translate_sll(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SLL_64BIT, inst)
    }
    pub fn translate_srl(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SRL_64BIT, inst)
    }
    pub fn translate_sra(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SRA_64BIT, inst)
    }

    pub fn translate_slliw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SLL_32BIT, inst)
    }
    pub fn translate_srliw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SRL_32BIT, inst)
    }
    pub fn translate_sraiw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SRA_32BIT, inst)
    }
    pub fn translate_sllw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SLL_32BIT, inst)
    }
    pub fn translate_srlw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SRL_32BIT, inst)
    }
    pub fn translate_sraw(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SRA_32BIT, inst)
    }

    pub fn translate_slt(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SLT_64BIT, inst)
    }
    pub fn translate_slti(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::SLT_64BIT, inst)
    }
    pub fn translate_sltu(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rrr(TCGOpcode::SLTU_64BIT, inst)
    }
    pub fn translate_sltiu(inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::SLTU_64BIT, inst)
    }


    pub fn translate_fence(_inst: &InstrInfo) -> Vec<TCGOp> {
        vec![]
    }
    pub fn translate_mret(_inst: &InstrInfo) -> Vec<TCGOp> {
        let mret_op = TCGOp::new_helper_call_arg0(CALL_HELPER_IDX::CALL_MRET_IDX as usize);
        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB);
        vec![mret_op, exit_tb]
    }
    pub fn translate_ecall(_inst: &InstrInfo) -> Vec<TCGOp> {
        let ecall_op = TCGOp::new_helper_call_arg0(CALL_HELPER_IDX::CALL_ECALL_IDX as usize);
        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB);
        vec![ecall_op, exit_tb]
    }
}
