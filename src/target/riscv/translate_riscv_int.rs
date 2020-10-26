use std::cell::RefCell;
use std::rc::Rc;

use super::super::super::tcg::tcg::{TCGOp, TCGOpcode, TCGv, TCGLabel};
use super::super::super::instr_info::InstrInfo;

use super::super::super::get_rs1_addr;
use super::super::super::get_rd_addr;
use super::super::super::extract_j_field;

use super::riscv::{TranslateRiscv, CALL_HELPER_IDX};

impl TranslateRiscv {
    pub fn translate_jal(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
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

        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);

        if rd_addr == 0 {
            return vec![tcg_inst, exit_tb];
        } else {
            return vec![mov_inst, tcg_inst, exit_tb];
        }
    }


    pub fn translate_jalr(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
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

        let exit_tb = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        if rd_addr == 0 {
            return vec![jmp_inst, exit_tb];
        } else {
            return vec![mov_inst, jmp_inst, exit_tb];
        }
    }


    pub fn translate_lui(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const: u64 = ((inst.inst as i32 as i64) & !0xfff) as u64;
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

    pub fn translate_auipc(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
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

    pub fn translate_add(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let source1 = self.tcg_temp_new();
        let source2 = self.tcg_temp_new();

        let rs1_addr= get_rs1_addr!(inst.inst);
        let rs2_addr= get_rs2_addr!(inst.inst);
        let rd_addr = get_rd_addr!(inst.inst); 

        if rd_addr == 0 {
            return vec![];
        }

        let rs1_op = TCGOp::new_get_gpr(source1, rs1_addr);  // Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2_op = TCGOp::new_get_gpr(source2, rs2_addr);  // Box::new(TCGv::new_reg(rs2_addr as u64));

        let tcg_inst = TCGOp::new_3op(TCGOpcode::ADD_TEMP, source1, source1, source2);

        let rd_op = TCGOp::new_set_gpr(rd_addr, source1);  // Box::new(TCGv::new_reg(rs1_addr as u64));

        self.tcg_temp_free(source1);
        self.tcg_temp_free(source2);

        vec![rs1_op, rs2_op, tcg_inst, rd_op]
    }
    pub fn translate_sub(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SUB_64BIT, inst)
    }
    pub fn translate_and(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::AND_64BIT, inst)
    }
    pub fn translate_or(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::OR_64BIT, inst)
    }
    pub fn translate_xor(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::XOR_64BIT, inst)
    }

    pub fn translate_addi(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::ADD_64BIT, inst)
    }
    pub fn translate_andi(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::AND_64BIT, inst)
    }
    pub fn translate_ori(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::OR_64BIT, inst)
    }
    pub fn translate_xori(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::XOR_64BIT, inst)
    }

    pub fn translate_addiw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::ADD_32BIT, inst)
    }
    pub fn translate_addw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::ADD_32BIT, inst)
    }
    pub fn translate_subw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SUB_32BIT, inst)
    }

    pub fn translate_beq(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::EQ_64BIT, inst)
    }
    pub fn translate_bne(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::NE_64BIT, inst)
    }
    pub fn translate_blt(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::LT_64BIT, inst)
    }
    pub fn translate_bge(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::GE_64BIT, inst)
    }
    pub fn translate_bltu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::LTU_64BIT, inst)
    }
    pub fn translate_bgeu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_branch(TCGOpcode::GEU_64BIT, inst)
    }

    pub fn translate_ld(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));

        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));

        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));

        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_LOAD64_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);

        let mut load_op = Self::translate_rri(TCGOpcode::LOAD_64BIT, inst);

        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));

        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);

        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut load_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;
    }

    pub fn translate_lw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        
        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        
        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));
        
        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));
        
        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));
        
        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_LOAD32_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);
        
        let mut load_op = Self::translate_rri(TCGOpcode::LOAD_32BIT, inst);
        
        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));
        
        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        
        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut load_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;
    }

    pub fn translate_lh(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        
        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        
        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));
        
        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));
        
        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));
        
        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_LOAD16_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);
        
        let mut load_op = Self::translate_rri(TCGOpcode::LOAD_16BIT, inst);
        
        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));
        
        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        
        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut load_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;
    }
    pub fn translate_lb(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        
        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        
        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));
        
        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));
        
        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));
        
        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_LOAD8_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);
        
        let mut load_op = Self::translate_rri(TCGOpcode::LOAD_8BIT, inst);
        
        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));
        
        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        
        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut load_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;
    }
    pub fn translate_lwu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        
        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        
        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));
        
        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));
        
        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));
        
        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_LOADU32_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);
        
        let mut load_op = Self::translate_rri(TCGOpcode::LOADU_32BIT, inst);
        
        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));
        
        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        
        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut load_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;

    }
    pub fn translate_lhu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        
        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        
        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));
        
        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));
        
        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));
        
        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_LOADU16_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);
        
        let mut load_op = Self::translate_rri(TCGOpcode::LOADU_16BIT, inst);
        
        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));
        
        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        
        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut load_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;

    }
    pub fn translate_lbu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;
        
        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));
        
        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));
        
        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));
        
        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));
        
        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_LOADU8_IDX as usize, *rd, *rs1, *imm, *tcg_inst_addr);
        
        let mut load_op = Self::translate_rri(TCGOpcode::LOADU_8BIT, inst);
        
        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));
        
        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        
        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut load_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;
    }

    pub fn translate_sd(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = get_s_imm_field!(inst.inst);
        let imm_const = ((imm_const as i32) << (32 - 12)) >> (32 - 12);
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        
        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        
        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));
        
        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));
        
        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));
        
        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_STORE64_IDX as usize, *rs2, *rs1, *imm, *tcg_inst_addr);
        
        let mut store_op = Self::translate_store(TCGOpcode::STORE_64BIT, inst);
        
        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));
        
        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        
        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut store_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;
    }

    pub fn translate_sw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = get_s_imm_field!(inst.inst);
        let imm_const = ((imm_const as i32) << (32 - 12)) >> (32 - 12);
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        
        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        
        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));
        
        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));
        
        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));
        
        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_STORE32_IDX as usize, *rs2, *rs1, *imm, *tcg_inst_addr);
        
        let mut store_op = Self::translate_store(TCGOpcode::STORE_32BIT, inst);
        
        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));
        
        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        
        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut store_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;
    }
    pub fn translate_sh(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = get_s_imm_field!(inst.inst);
        let imm_const = ((imm_const as i32) << (32 - 12)) >> (32 - 12);
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        
        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        
        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));
        
        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));
        
        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));
        
        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_STORE16_IDX as usize, *rs2, *rs1, *imm, *tcg_inst_addr);
        
        let mut store_op = Self::translate_store(TCGOpcode::STORE_16BIT, inst);
        
        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));
        
        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        
        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut store_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;
    }
    pub fn translate_sb(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = get_s_imm_field!(inst.inst);
        let imm_const = ((imm_const as i32) << (32 - 12)) >> (32 - 12);
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        
        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        
        let tcg_inst_addr = Box::new(TCGv::new_imm(inst.addr));
        
        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));
        
        let tcg_match_op = TCGOp::new_2op(TCGOpcode::TLB_MATCH_CHECK, *rs1, *imm);
        let tcg_tlb_tcg_reslut_excp_cmp_op = TCGOp::new_0op(TCGOpcode::CMP_EQ, Some(Rc::clone(&label_tlb_match)));
        
        let tcg_helper_call_op = TCGOp::new_helper_call_arg4(CALL_HELPER_IDX::CALL_STORE8_IDX as usize, *rs2, *rs1, *imm, *tcg_inst_addr);
        
        let mut store_op = Self::translate_store(TCGOpcode::STORE_8BIT, inst);
        
        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));
        
        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));
        
        let tcg_result_excp_cmp_op = TCGOp::new_4op(TCGOpcode::EQ_EAX_64BIT, *rs1, *zero, *dummy_addr, Rc::clone(&label_load_excp));
        let tcg_exit_tb1 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        let tcg_exit_tb2 = TCGOp::new_0op(TCGOpcode::EXIT_TB, None);
        
        let mut vec_ops = vec![tcg_match_op, tcg_tlb_tcg_reslut_excp_cmp_op, tcg_helper_call_op, tcg_result_excp_cmp_op, tcg_exit_tb1, tcg_exit_tb2, tcg_label_tlb_match];
        vec_ops.append(&mut store_op);
        vec_ops.push(tcg_label_load_excp);
        return vec_ops;
    }


    pub fn translate_slli(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SLL_64BIT, inst)
    }
    pub fn translate_srli(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SRL_64BIT, inst)
    }
    pub fn translate_srai(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SRA_64BIT, inst)
    }
    pub fn translate_sll(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SLL_64BIT, inst)
    }
    pub fn translate_srl(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SRL_64BIT, inst)
    }
    pub fn translate_sra(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SRA_64BIT, inst)
    }

    pub fn translate_slliw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SLL_32BIT, inst)
    }
    pub fn translate_srliw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SRL_32BIT, inst)
    }
    pub fn translate_sraiw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_shift_i(TCGOpcode::SRA_32BIT, inst)
    }
    pub fn translate_sllw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SLL_32BIT, inst)
    }
    pub fn translate_srlw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SRL_32BIT, inst)
    }
    pub fn translate_sraw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SRA_32BIT, inst)
    }

    pub fn translate_slt(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SLT_64BIT, inst)
    }
    pub fn translate_slti(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::SLT_64BIT, inst)
    }
    pub fn translate_sltu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SLTU_64BIT, inst)
    }
    pub fn translate_sltiu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        Self::translate_rri(TCGOpcode::SLTU_64BIT, inst)
    }


    pub fn translate_mul(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::MUL_64BIT, inst)
    }
    pub fn translate_mulh(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::MULH_64BIT, inst)
    }
    pub fn translate_mulhu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::MULHU_64BIT, inst)
    }
    pub fn translate_mulhsu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::MULHSU_64BIT, inst)
    }
    pub fn translate_mulw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::MUL_32BIT, inst)
    }

    pub fn translate_div(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::DIV_64BIT, inst)
    }
    pub fn translate_divu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::DIVU_64BIT, inst)
    }
    pub fn translate_divw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::DIV_32BIT, inst)
    }
    pub fn translate_divuw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::DIVU_32BIT, inst)
    }

    pub fn translate_rem(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::REM_64BIT, inst)
    }
    pub fn translate_remu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::REMU_64BIT, inst)
    }
    pub fn translate_remw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::REM_32BIT, inst)
    }
    pub fn translate_remuw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::REMU_32BIT, inst)
    }

}
