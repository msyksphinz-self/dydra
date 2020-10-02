use super::super::super::tcg::tcg::{TCGOp, TCGOpcode, TCGv};
use super::super::super::instr_info::InstrInfo;

use super::super::super::get_rs1_addr;
use super::super::super::get_rd_addr;
use super::super::super::extract_j_field;

use super::riscv::TranslateRiscv;

impl TranslateRiscv {
    pub fn translate_jal(inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const = extract_j_field!(inst.inst);
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let imm_const = ((imm_const as i32) << (32 - 21)) >> (32 - 21);

        let imm = Box::new(TCGv::new_imm(
            ((imm_const as i64).wrapping_add(inst.addr as i64)) as u64,
        ));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let next_pc = Box::new(TCGv::new_imm(inst.addr.wrapping_add(4)));
        let mov_inst = TCGOp::new_2op(TCGOpcode::MOV_IMM_64BIT, *rd, *next_pc);
        let tcg_inst = TCGOp::new_2op(TCGOpcode::JMPIM, *rd, *imm);

        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB);

        if rd_addr == 0 {
            return vec![tcg_inst, exit_tb];
        } else {
            return vec![mov_inst, tcg_inst, exit_tb];
        }
    }


    pub fn translate_jalr(inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let zero = Box::new(TCGv::new_reg(0));
        let next_pc = Box::new(TCGv::new_imm(inst.addr.wrapping_add(4)));
        let mov_inst = TCGOp::new_3op(TCGOpcode::ADD_64BIT, *rd, *zero, *next_pc);
        let jmp_inst = TCGOp::new_3op(TCGOpcode::JMPR, *rd, *rs1, *imm);

        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB);
        if rd_addr == 0 {
            return vec![jmp_inst, exit_tb];
        } else {
            return vec![mov_inst, jmp_inst, exit_tb];
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
            return vec![tcg_inst];
        } else {
            return vec![];
        }
    }

    pub fn translate_auipc(inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const = (((inst.inst as i32 as i64) & !0xfff) as u64).wrapping_add(inst.addr as u64);
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let imm = Box::new(TCGv::new_imm(imm_const as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        if rd_addr != 0 {
            let tcg_inst = TCGOp::new_2op(TCGOpcode::MOV_IMM_64BIT, *rd, *imm);
            return vec![tcg_inst];
        } else {
            return vec![];
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


}
