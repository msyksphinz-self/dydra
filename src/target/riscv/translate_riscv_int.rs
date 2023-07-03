use std::cell::RefCell;
use std::rc::Rc;

use super::super::super::instr_info::InstrInfo;
use super::super::super::tcg::tcg::{TCGLabel, TCGOp, TCGOpcode, TCGv};

use super::super::super::extract_j_field;
use super::super::super::get_rd_addr;
use super::super::super::get_rs1_addr;

use super::riscv::{TranslateRiscv, CALL_HELPER_IDX};

impl TranslateRiscv {
    pub fn translate_jal(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const = extract_j_field!(inst.inst);
        let rd_addr = get_rd_addr!(inst.inst);

        let imm_const = ((imm_const as i32) << (32 - 21)) >> (32 - 21);
        let imm = TCGv::new_imm(((imm_const as i64).wrapping_add(inst.addr as i64)) as u64);

        let mut tcg_lists = vec![];

        let dest_temp = self.tcg_temp_new();
        let next_pc = TCGv::new_imm(inst.addr.wrapping_add(4));
        if rd_addr != 0 {
            tcg_lists.push(TCGOp::new_2op(TCGOpcode::MOV_IMM_64BIT, dest_temp, next_pc));
            tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, dest_temp));
        }
        tcg_lists.push(TCGOp::new_2op(TCGOpcode::JMPIM, dest_temp, imm));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));
        self.tcg_temp_free(dest_temp);

        tcg_lists
    }

    pub fn translate_jalr(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr = get_rs1_addr!(inst.inst);
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr = get_rd_addr!(inst.inst);

        let mut tcg_lists = vec![];

        let source1 = self.tcg_temp_new();
        let dest = self.tcg_temp_new();
        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rs1_addr));

        let imm = TCGv::new_imm(imm_const);

        if rd_addr != 0 {
            let zero = self.tcg_temp_new();
            tcg_lists.push(TCGOp::tcg_get_gpr(zero, 0));
            let next_pc = TCGv::new_imm((inst.addr as u64).wrapping_add(4));
            tcg_lists.push(TCGOp::new_2op(TCGOpcode::MOV_IMM_64BIT, dest, next_pc));
            self.tcg_temp_free(zero);
            tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, dest));
        }
        tcg_lists.push(TCGOp::new_3op(TCGOpcode::JMPR, dest, source1, imm));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));

        self.tcg_temp_free(dest);
        self.tcg_temp_free(source1);

        tcg_lists
    }

    pub fn translate_lui(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rd_addr = get_rd_addr!(inst.inst);

        let imm_const: u64 = ((inst.inst as i32 as i64) & !0xfff) as u64;
        let tcg_imm = TCGv::new_imm(imm_const);

        if rd_addr == 0 {
            return vec![];
        }

        let source1 = self.tcg_temp_new();
        let rs1_op = TCGOp::tcg_get_gpr(source1, 0); // Box::new(TCGv::new_reg(rs1_addr as u64));
        let tcg_inst = TCGOp::new_3op(TCGOpcode::ADD_64BIT, source1, source1, tcg_imm);
        let rd_op = TCGOp::tcg_set_gpr(rd_addr, source1); // Box::new(TCGv::new_reg(rs1_addr as u64));
        self.tcg_temp_free(source1);
        vec![rs1_op, tcg_inst, rd_op]
    }

    pub fn translate_auipc(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let imm_const =
            (((inst.inst as i32 as i64) & !0xfff) as u64).wrapping_add(inst.addr as u64);
        let rd_addr = get_rd_addr!(inst.inst);

        let imm = TCGv::new_imm(imm_const as u64);

        let mut tcg_lists = vec![];
        let dest_temp = self.tcg_temp_new();

        if rd_addr != 0 {
            tcg_lists.push(TCGOp::new_2op(TCGOpcode::MOV_IMM_64BIT, dest_temp, imm));
            tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, dest_temp));
        }
        self.tcg_temp_free(dest_temp);

        tcg_lists
    }

    pub fn translate_add(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::ADD_64BIT, inst)
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
        self.translate_rri(TCGOpcode::ADD_64BIT, inst)
    }
    pub fn translate_andi(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rri(TCGOpcode::AND_64BIT, inst)
    }
    pub fn translate_ori(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rri(TCGOpcode::OR_64BIT, inst)
    }
    pub fn translate_xori(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rri(TCGOpcode::XOR_64BIT, inst)
    }

    pub fn translate_addiw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr = get_rs1_addr!(inst.inst);
        let rd_addr = get_rd_addr!(inst.inst);

        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let tcg_imm = TCGv::new_imm(imm_const);

        if rd_addr == 0 {
            return vec![];
        }

        let source1 = self.tcg_temp_new();
        let rs1_op = TCGOp::tcg_get_gpr(source1, rs1_addr); // Box::new(TCGv::new_reg(rs1_addr as u64));
        let tcg_inst = TCGOp::new_3op(TCGOpcode::ADD_32BIT, source1, source1, tcg_imm);
        let tcg_sign_ext = TCGOp::new_2op(TCGOpcode::SIGN_EXT_32_64, source1, source1);
        let rd_op = TCGOp::tcg_set_gpr(rd_addr, source1); // Box::new(TCGv::new_reg(rs1_addr as u64));
        self.tcg_temp_free(source1);
        vec![rs1_op, tcg_inst, tcg_sign_ext, rd_op]
    }
    pub fn translate_addw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr_32bit(TCGOpcode::ADD_32BIT, inst)
    }
    pub fn translate_subw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr_32bit(TCGOpcode::SUB_32BIT, inst)
    }

    pub fn translate_beq(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_branch(TCGOpcode::EQ_64BIT, inst)
    }
    pub fn translate_bne(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_branch(TCGOpcode::NE_64BIT, inst)
    }
    pub fn translate_blt(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_branch(TCGOpcode::LT_64BIT, inst)
    }
    pub fn translate_bge(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_branch(TCGOpcode::GE_64BIT, inst)
    }
    pub fn translate_bltu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_branch(TCGOpcode::LTU_64BIT, inst)
    }
    pub fn translate_bgeu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_branch(TCGOpcode::GEU_64BIT, inst)
    }

    pub fn translate_raw_load(
        &mut self,
        base_reg: u32,
        offset: u64,
        dest_reg: u32,
        inst: &InstrInfo,
        load_op: TCGOpcode,
        helper_op: CALL_HELPER_IDX,
    ) -> Vec<TCGOp> {
        let src_addr = self.tcg_temp_new();
        let vaddr_low12bit = self.tcg_temp_new();
        let vaddr_tlb_idx = self.tcg_temp_new();
        let stack_reg = self.tcg_temp_new();
        let tlb_byte_addr = self.tcg_temp_new();

        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));

        let mut tcg_lists = vec![];

        // Read Register
        tcg_lists.push(TCGOp::tcg_get_gpr(src_addr, base_reg));
        // Extract TLB Index and offset
        if offset != 0 {
            tcg_lists.push(TCGOp::new_3op(
                TCGOpcode::ADD_64BIT,
                src_addr,
                src_addr,
                TCGv::new_imm(offset as u64),
            ));
        }
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::AND_64BIT,
            vaddr_low12bit,
            src_addr,
            TCGv::new_imm(0xfff),
        ));

        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::SRL_64BIT,
            vaddr_tlb_idx,
            src_addr,
            TCGv::new_imm(12),
        ));
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::AND_64BIT,
            vaddr_tlb_idx,
            vaddr_tlb_idx,
            TCGv::new_imm(0xfff),
        ));
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::SLL_64BIT,
            vaddr_tlb_idx,
            vaddr_tlb_idx,
            TCGv::new_imm(3),
        ));

        // Make TLB Vaddr Index Address
        tcg_lists.push(TCGOp::new_1op(TCGOpcode::MOVE_STACK, stack_reg));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::ADD_TLBIDX_OFFSET,
            tlb_byte_addr,
            stack_reg,
        )); // Relative Addr of TLB
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::ADD_64BIT,
            tlb_byte_addr,
            tlb_byte_addr,
            vaddr_tlb_idx,
        ));

        // Make VAddr upper bit for compare TLB value
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::SRL_64BIT,
            src_addr,
            src_addr,
            TCGv::new_imm(24),
        ));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::MEM_LOAD,
            tlb_byte_addr,
            tlb_byte_addr,
        ));
        tcg_lists.push(TCGOp::new_2op_with_label(
            TCGOpcode::CMP_EQ,
            src_addr,
            tlb_byte_addr,
            Rc::clone(&label_tlb_match),
        ));
        // if TLB not hit, jump helper function
        tcg_lists.push(TCGOp::new_helper_call_arg4(
            helper_op as usize,
            TCGv::new_reg(dest_reg as u64),
            TCGv::new_reg(base_reg as u64),
            TCGv::new_imm(offset as u64),
            TCGv::new_imm(inst.addr),
        ));

        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));

        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));

        tcg_lists.push(TCGOp::new_4op(
            TCGOpcode::EQ_EAX_64BIT,
            src_addr,
            *zero,
            *dummy_addr,
            Rc::clone(&label_load_excp),
        ));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));

        // Extract lower 12bit address and add with TLB address
        tcg_lists.push(tcg_label_tlb_match);
        tcg_lists.push(TCGOp::new_1op(TCGOpcode::MOVE_STACK, stack_reg));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::ADD_TLBADDR_OFFSET,
            tlb_byte_addr,
            stack_reg,
        )); // Relative Addr of TLB Paddr
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::ADD_64BIT,
            tlb_byte_addr,
            tlb_byte_addr,
            vaddr_tlb_idx,
        ));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::MEM_LOAD,
            tlb_byte_addr,
            tlb_byte_addr,
        ));
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::ADD_64BIT,
            tlb_byte_addr,
            tlb_byte_addr,
            vaddr_low12bit,
        ));
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::ADD_64BIT,
            tlb_byte_addr,
            tlb_byte_addr,
            TCGv::new_imm(0x80000000),
        ));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::ADD_MEM_OFFSET,
            tlb_byte_addr,
            tlb_byte_addr,
        ));
        tcg_lists.push(TCGOp::new_2op(load_op, tlb_byte_addr, tlb_byte_addr));
        tcg_lists.push(TCGOp::tcg_set_gpr(dest_reg, tlb_byte_addr));
        tcg_lists.push(tcg_label_load_excp);

        self.tcg_temp_free(src_addr);
        self.tcg_temp_free(vaddr_low12bit);
        self.tcg_temp_free(vaddr_tlb_idx);
        self.tcg_temp_free(stack_reg);
        self.tcg_temp_free(tlb_byte_addr);

        return tcg_lists;
    }
    fn translate_load(
        &mut self,
        inst: &InstrInfo,
        load_op: TCGOpcode,
        helper_op: CALL_HELPER_IDX,
    ) -> Vec<TCGOp> {
        let rs1_addr = get_rs1_addr!(inst.inst);
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr = get_rd_addr!(inst.inst);

        self.translate_raw_load(rs1_addr, imm_const, rd_addr, inst, load_op, helper_op)
    }

    pub fn translate_ld(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_load(
            inst,
            TCGOpcode::LOAD_64BIT,
            CALL_HELPER_IDX::CALL_LOAD64_IDX,
        )
    }
    pub fn translate_lw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_load(
            inst,
            TCGOpcode::LOAD_32BIT,
            CALL_HELPER_IDX::CALL_LOAD32_IDX,
        )
    }
    pub fn translate_lh(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_load(
            inst,
            TCGOpcode::LOAD_16BIT,
            CALL_HELPER_IDX::CALL_LOAD16_IDX,
        )
    }
    pub fn translate_lb(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_load(inst, TCGOpcode::LOAD_8BIT, CALL_HELPER_IDX::CALL_LOAD8_IDX)
    }
    pub fn translate_lwu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_load(
            inst,
            TCGOpcode::LOADU_32BIT,
            CALL_HELPER_IDX::CALL_LOADU32_IDX,
        )
    }
    pub fn translate_lhu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_load(
            inst,
            TCGOpcode::LOADU_16BIT,
            CALL_HELPER_IDX::CALL_LOADU16_IDX,
        )
    }
    pub fn translate_lbu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_load(
            inst,
            TCGOpcode::LOADU_8BIT,
            CALL_HELPER_IDX::CALL_LOADU8_IDX,
        )
    }

    pub fn translate_raw_store(
        &mut self,
        base_reg: u32,
        offset: u64,
        dest_reg: u32,
        inst: &InstrInfo,
        store_op: TCGOpcode,
        helper_op: CALL_HELPER_IDX,
    ) -> Vec<TCGOp> {
        let src_addr = self.tcg_temp_new();
        let vaddr_low12bit = self.tcg_temp_new();
        let vaddr_tlb_idx = self.tcg_temp_new();
        let stack_reg = self.tcg_temp_new();
        let tlb_byte_addr = self.tcg_temp_new();

        let label_tlb_match = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_tlb_match = TCGOp::new_label(Rc::clone(&label_tlb_match));

        let mut tcg_lists = vec![];

        // Read Register
        tcg_lists.push(TCGOp::tcg_get_gpr(src_addr, base_reg));
        // Extract TLB Index and offset
        if offset != 0 {
            tcg_lists.push(TCGOp::new_3op(
                TCGOpcode::ADD_64BIT,
                src_addr,
                src_addr,
                TCGv::new_imm(offset as u64),
            ));
        }
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::AND_64BIT,
            vaddr_low12bit,
            src_addr,
            TCGv::new_imm(0xfff),
        ));

        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::SRL_64BIT,
            vaddr_tlb_idx,
            src_addr,
            TCGv::new_imm(12),
        ));
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::AND_64BIT,
            vaddr_tlb_idx,
            vaddr_tlb_idx,
            TCGv::new_imm(0xfff),
        ));
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::SLL_64BIT,
            vaddr_tlb_idx,
            vaddr_tlb_idx,
            TCGv::new_imm(3),
        ));

        // Make TLB Vaddr Index Address
        tcg_lists.push(TCGOp::new_1op(TCGOpcode::MOVE_STACK, stack_reg));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::ADD_TLBIDX_OFFSET,
            tlb_byte_addr,
            stack_reg,
        )); // Relative Addr of TLB
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::ADD_64BIT,
            tlb_byte_addr,
            tlb_byte_addr,
            vaddr_tlb_idx,
        ));
        //
        // Make VAddr upper bit for compare TLB value
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::SRL_64BIT,
            src_addr,
            src_addr,
            TCGv::new_imm(24),
        ));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::MEM_LOAD,
            tlb_byte_addr,
            tlb_byte_addr,
        ));
        tcg_lists.push(TCGOp::new_2op_with_label(
            TCGOpcode::CMP_EQ,
            src_addr,
            tlb_byte_addr,
            Rc::clone(&label_tlb_match),
        ));
        // if TLB not hit, jump helper function
        tcg_lists.push(TCGOp::new_helper_call_arg4(
            helper_op as usize,
            TCGv::new_reg(dest_reg as u64),
            TCGv::new_reg(base_reg as u64),
            TCGv::new_imm(offset as u64),
            TCGv::new_imm(inst.addr),
        ));

        let zero = Box::new(TCGv::new_reg(0 as u64));
        let dummy_addr = Box::new(TCGv::new_imm(0));

        let label_load_excp = Rc::new(RefCell::new(TCGLabel::new()));
        let tcg_label_load_excp = TCGOp::new_label(Rc::clone(&label_load_excp));

        tcg_lists.push(TCGOp::new_4op(
            TCGOpcode::EQ_EAX_64BIT,
            src_addr,
            *zero,
            *dummy_addr,
            Rc::clone(&label_load_excp),
        ));
        tcg_lists.push(TCGOp::new_0op(TCGOpcode::EXIT_TB, None));
        self.tcg_temp_free(src_addr);

        // Extract lower 12bit address and add with TLB address
        tcg_lists.push(tcg_label_tlb_match);
        tcg_lists.push(TCGOp::new_1op(TCGOpcode::MOVE_STACK, stack_reg));
        self.tcg_temp_free(stack_reg);
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::ADD_TLBADDR_OFFSET,
            tlb_byte_addr,
            stack_reg,
        )); // Relative Addr of TLB Paddr
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::ADD_64BIT,
            tlb_byte_addr,
            tlb_byte_addr,
            vaddr_tlb_idx,
        ));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::MEM_LOAD,
            tlb_byte_addr,
            tlb_byte_addr,
        ));
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::ADD_64BIT,
            tlb_byte_addr,
            tlb_byte_addr,
            vaddr_low12bit,
        ));
        tcg_lists.push(TCGOp::new_3op(
            TCGOpcode::ADD_64BIT,
            tlb_byte_addr,
            tlb_byte_addr,
            TCGv::new_imm(0x80000000),
        ));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::ADD_MEM_OFFSET,
            tlb_byte_addr,
            tlb_byte_addr,
        ));
        let rs2_data = self.tcg_temp_new();
        tcg_lists.push(TCGOp::tcg_get_gpr(rs2_data, dest_reg));
        tcg_lists.push(TCGOp::new_2op(store_op, rs2_data, tlb_byte_addr));
        tcg_lists.push(tcg_label_load_excp);

        self.tcg_temp_free(vaddr_low12bit);
        self.tcg_temp_free(vaddr_tlb_idx);
        self.tcg_temp_free(tlb_byte_addr);
        self.tcg_temp_free(rs2_data);

        return tcg_lists;
    }

    fn translate_store(
        &mut self,
        inst: &InstrInfo,
        store_op: TCGOpcode,
        helper_op: CALL_HELPER_IDX,
    ) -> Vec<TCGOp> {
        let rs1_addr = get_rs1_addr!(inst.inst);
        let imm_const: u64 = get_s_imm_field!(inst.inst);
        let imm_const = ((imm_const as i32) << (32 - 12)) >> (32 - 12);
        let rs2_addr = get_rs2_addr!(inst.inst);

        self.translate_raw_store(
            rs1_addr,
            imm_const as u64,
            rs2_addr,
            inst,
            store_op,
            helper_op,
        )
    }

    pub fn translate_sd(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_store(
            inst,
            TCGOpcode::STORE_64BIT,
            CALL_HELPER_IDX::CALL_STORE64_IDX,
        )
    }
    pub fn translate_sw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_store(
            inst,
            TCGOpcode::STORE_32BIT,
            CALL_HELPER_IDX::CALL_STORE32_IDX,
        )
    }
    pub fn translate_sh(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_store(
            inst,
            TCGOpcode::STORE_16BIT,
            CALL_HELPER_IDX::CALL_STORE16_IDX,
        )
    }
    pub fn translate_sb(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_store(
            inst,
            TCGOpcode::STORE_8BIT,
            CALL_HELPER_IDX::CALL_STORE8_IDX,
        )
    }

    pub fn translate_slli(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_i(TCGOpcode::SLL_64BIT, inst)
    }
    pub fn translate_srli(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_i(TCGOpcode::SRL_64BIT, inst)
    }
    pub fn translate_srai(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_i(TCGOpcode::SRA_64BIT, inst)
    }
    pub fn translate_sll(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_r(TCGOpcode::SLL_64BIT, inst)
    }
    pub fn translate_srl(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_r(TCGOpcode::SRL_64BIT, inst)
    }
    pub fn translate_sra(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_r(TCGOpcode::SRA_64BIT, inst)
    }

    pub fn translate_slliw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_i(TCGOpcode::SLL_32BIT, inst)
    }
    pub fn translate_srliw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_i(TCGOpcode::SRL_32BIT, inst)
    }
    pub fn translate_sraiw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_i(TCGOpcode::SRA_32BIT, inst)
    }
    pub fn translate_sllw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_r(TCGOpcode::SLL_32BIT, inst)
    }
    pub fn translate_srlw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_r(TCGOpcode::SRL_32BIT, inst)
    }
    pub fn translate_sraw(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_shift_r(TCGOpcode::SRA_32BIT, inst)
    }

    pub fn translate_slt(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SLT_64BIT, inst)
    }
    pub fn translate_slti(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rri(TCGOpcode::SLT_64BIT, inst)
    }
    pub fn translate_sltu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rrr(TCGOpcode::SLTU_64BIT, inst)
    }
    pub fn translate_sltiu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        self.translate_rri(TCGOpcode::SLTU_64BIT, inst)
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

    fn translate_div_common(
        &mut self,
        op: TCGOpcode,
        rd_addr: u32,
        rs1_addr: u32,
        rs2_addr: u32,
        inst: &InstrInfo,
    ) -> Vec<TCGOp> {
        if rd_addr == 0 {
            return vec![];
        }

        let label_src2_zero_fail = Rc::new(RefCell::new(TCGLabel::new()));
        let label_cond1_fail = Rc::new(RefCell::new(TCGLabel::new()));
        let label_cond2_fail = Rc::new(RefCell::new(TCGLabel::new()));
        let label_finish = Rc::new(RefCell::new(TCGLabel::new()));

        let mut tcg_lists = vec![];

        let source1 = self.tcg_temp_new();
        let source2 = self.tcg_temp_new();

        tcg_lists.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_lists.push(TCGOp::tcg_get_gpr(source2, rs2_addr));

        // when source2 == zero, doesn't execute.
        let zero = self.tcg_temp_new();
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::MOV_IMM_64BIT,
            zero,
            TCGv::new_imm(0),
        ));
        tcg_lists.push(TCGOp::new_4op(
            TCGOpcode::CMP_EQ,
            source2,
            zero,
            TCGv::new_imm(0),
            Rc::clone(&label_src2_zero_fail),
        ));
        self.tcg_temp_free(zero);

        // when source1 == 8000_0000_0000_0000, doesn't execute.
        let minus1 = self.tcg_temp_new();
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::MOV_IMM_64BIT,
            minus1,
            TCGv::new_imm(0x8000_0000_0000_0000),
        ));
        tcg_lists.push(TCGOp::new_4op(
            TCGOpcode::CMP_EQ,
            source1,
            minus1,
            TCGv::new_imm(0),
            Rc::clone(&label_cond1_fail),
        ));
        self.tcg_temp_free(minus1);

        tcg_lists.push(TCGOp::new_3op(op, source1, source1, source2));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, source1));
        // Actually this is no conditional jump
        tcg_lists.push(TCGOp::new_4op(
            TCGOpcode::CMP_EQ,
            source1,
            source1,
            TCGv::new_imm(0),
            Rc::clone(&label_finish),
        ));

        let zero = self.tcg_temp_new();
        tcg_lists.push(TCGOp::new_label(Rc::clone(&label_cond1_fail)));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::MOV_IMM_64BIT,
            zero,
            TCGv::new_imm((0 - 1) as u64),
        ));
        tcg_lists.push(TCGOp::new_4op(
            TCGOpcode::CMP_EQ,
            source2,
            zero,
            TCGv::new_imm(0),
            Rc::clone(&label_cond2_fail),
        ));
        self.tcg_temp_free(zero);

        tcg_lists.push(TCGOp::new_3op(op, source1, source1, source2));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, source1));
        // Actually this is no conditional jump
        tcg_lists.push(TCGOp::new_4op(
            TCGOpcode::CMP_EQ,
            source1,
            source1,
            TCGv::new_imm(0),
            Rc::clone(&label_finish),
        ));

        tcg_lists.push(TCGOp::new_label(Rc::clone(&label_cond2_fail)));
        // Actually this is no conditional jump
        tcg_lists.push(TCGOp::new_4op(
            TCGOpcode::CMP_EQ,
            source1,
            source1,
            TCGv::new_imm(0),
            Rc::clone(&label_finish),
        ));

        let minus1 = self.tcg_temp_new();
        tcg_lists.push(TCGOp::new_label(Rc::clone(&label_src2_zero_fail)));
        tcg_lists.push(TCGOp::new_2op(
            TCGOpcode::MOV_IMM_64BIT,
            minus1,
            TCGv::new_imm((0 - 1) as u64),
        ));
        tcg_lists.push(TCGOp::tcg_set_gpr(rd_addr, minus1));
        self.tcg_temp_free(minus1);

        tcg_lists.push(TCGOp::new_label(Rc::clone(&label_finish)));

        self.tcg_temp_free(source1);
        self.tcg_temp_free(source2);

        tcg_lists
    }

    pub fn translate_div(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr = get_rs1_addr!(inst.inst);
        let rs2_addr = get_rs2_addr!(inst.inst);
        let rd_addr = get_rd_addr!(inst.inst);

        self.translate_div_common(TCGOpcode::DIV_64BIT, rd_addr, rs1_addr, rs2_addr, inst)
    }
    pub fn translate_divu(&mut self, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr = get_rs1_addr!(inst.inst);
        let rs2_addr = get_rs2_addr!(inst.inst);
        let rd_addr = get_rd_addr!(inst.inst);

        self.translate_div_common(TCGOpcode::DIVU_64BIT, rd_addr, rs1_addr, rs2_addr, inst)
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
