use super::tcg::{TCGLabel, TCGOp, TCGOpcode, TCGv};

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

macro_rules! get_imm12 {
    ($inst:expr) => {
        ($inst >> 20) as u64
    };
}

pub struct TranslateRiscv;

impl TranslateRiscv {
    pub fn translate_jalr(inst: &u32) -> Box<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let imm_const: u64 = (*inst as u64) >> 20 & 0xfff;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = Box::new(TCGOp::new_3op(TCGOpcode::JMP, *rd, *rs1, *imm));

        tcg_inst
    }

    pub fn translate_lui(inst: &u32) -> Box<TCGOp> {
        let imm_const: u64 = (*inst as u64) & !0xfff;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(0));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = Box::new(TCGOp::new_3op(TCGOpcode::ADD, *rd, *rs1, *imm));

        tcg_inst
    }

    fn translate_rrr(op: TCGOpcode, inst: &u32) -> Box<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(*inst) as usize;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = Box::new(TCGOp::new_3op(op, *rd, *rs1, *rs2));

        tcg_inst
    }

    fn translate_rri(op: TCGOpcode, inst: &u32) -> Box<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let imm_const: u64 = (*inst >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = Box::new(TCGOp::new_3op(op, *rd, *rs1, *imm));

        tcg_inst
    }

    fn translate_branch(op: TCGOpcode, inst: &u32) -> Box<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(*inst) as usize;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;
        let imm: u64 = get_imm12!(*inst);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));

        let label = Box::new(TCGLabel::new());

        let tcg_inst = Box::new(TCGOp::new_4op(op, *rs1, *rs2, *label));
        let tcg_true_tb = Box::new(TCGOp::new_goto_tb(TCGv::new_imm(4)));
        let tcg_set_label = Box::new(TCGOp::new_label(*label));

        let tcg_false_tb = Box::new(TCGOp::new_goto_tb(TCGv::new_imm(imm)));

        tcg_inst
    }

    pub fn translate_add(inst: &u32) -> Box<TCGOp> {
        Self::translate_rrr(TCGOpcode::ADD, inst)
    }
    pub fn translate_sub(inst: &u32) -> Box<TCGOp> {
        Self::translate_rrr(TCGOpcode::SUB, inst)
    }
    pub fn translate_and(inst: &u32) -> Box<TCGOp> {
        Self::translate_rrr(TCGOpcode::AND, inst)
    }
    pub fn translate_or(inst: &u32) -> Box<TCGOp> {
        Self::translate_rrr(TCGOpcode::OR, inst)
    }
    pub fn translate_xor(inst: &u32) -> Box<TCGOp> {
        Self::translate_rrr(TCGOpcode::XOR, inst)
    }

    pub fn translate_addi(inst: &u32) -> Box<TCGOp> {
        Self::translate_rri(TCGOpcode::ADD, inst)
    }
    pub fn translate_andi(inst: &u32) -> Box<TCGOp> {
        Self::translate_rri(TCGOpcode::AND, inst)
    }
    pub fn translate_ori(inst: &u32) -> Box<TCGOp> {
        Self::translate_rri(TCGOpcode::OR, inst)
    }
    pub fn translate_xori(inst: &u32) -> Box<TCGOp> {
        Self::translate_rri(TCGOpcode::XOR, inst)
    }

    pub fn translate_beq(inst: &u32) -> Box<TCGOp> {
        Self::translate_branch(TCGOpcode::EQ, inst)
    }
}
