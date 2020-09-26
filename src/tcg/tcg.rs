extern crate mmap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::emu_env::EmuEnv;

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TCGOpcode {
    HELPER_CALL_ARG0,
    HELPER_CALL_ARG1,
    HELPER_CALL_ARG2,
    HELPER_CALL_ARG3,
    HELPER_CALL_ARG4,
    MOV_64BIT,
    ADD_64BIT,
    SUB_64BIT,
    AND_64BIT,
    OR_64BIT,
    XOR_64BIT,
    SRL_64BIT,
    SLL_64BIT,
    SRA_64BIT,
    JMPR,
    JMPIM,
    EQ_64BIT,
    NE_64BIT,
    LT_64BIT,
    GE_64BIT,
    LTU_64BIT,
    GEU_64BIT,
    LOAD_64BIT,
    LOAD_32BIT,
    LOAD_16BIT,
    LOAD_8BIT,
    LOADU_32BIT,
    LOADU_16BIT,
    LOADU_8BIT,
    STORE_64BIT,
    STORE_32BIT,
    STORE_16BIT,
    STORE_8BIT,

    LOAD_FLOAT_64BIT,
    LOAD_FLOAT_32BIT,
    STORE_FLOAT_64BIT,
    STORE_FLOAT_32BIT,

    MOVE_TO_INT_FROM_FLOAT,
    MOVE_TO_FLOAT_FROM_INT,
    MOVE_TO_INT_FROM_FLOAT_32BIT,
    MOVE_TO_FLOAT_FROM_INT_32BIT,

    ADD_32BIT,
    SUB_32BIT,
    SRL_32BIT,
    SLL_32BIT,
    SRA_32BIT,

    SLT_64BIT,
    SLTU_64BIT,

    SGNJ_64BIT,
    SGNJN_64BIT,
    SGNJX_64BIT,
    
    SGNJ_32BIT,
    SGNJN_32BIT,
    SGNJX_32BIT,   

    EXIT_TB,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MemOpType {
    LOAD_64BIT,
    LOAD_32BIT,
    LOAD_16BIT,
    LOAD_8BIT,
    LOAD_U_32BIT,
    LOAD_U_16BIT,
    LOAD_U_8BIT,
    STORE_64BIT,
    STORE_32BIT,
    STORE_16BIT,
    STORE_8BIT,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TCGvType {
    Register,
    Immediate,
    ProgramCounter,
}

#[derive(Debug, Clone)]
pub struct TCGOp {
    pub op: Option<TCGOpcode>,
    pub arg0: Option<TCGv>,
    pub arg1: Option<TCGv>,
    pub arg2: Option<TCGv>,
    pub arg3: Option<TCGv>,
    pub label: Option<Rc<RefCell<TCGLabel>>>,
    pub helper_idx: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegisterType {
    IntRegister,
    FloatRegister,
}

impl TCGOp {
    pub fn new_0op(opcode: TCGOpcode) -> TCGOp {
        TCGOp {
            op: Some(opcode),
            arg0: None,
            arg1: None,
            arg2: None,
            arg3: None,
            label: None,
            helper_idx: 0,
        }
    }
    pub fn new_2op(opcode: TCGOpcode, a1: TCGv, a2: TCGv) -> TCGOp {
        TCGOp {
            op: Some(opcode),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: None,
            arg3: None,
            label: None,
            helper_idx: 0,
        }
    }

    pub fn new_3op(opcode: TCGOpcode, a1: TCGv, a2: TCGv, a3: TCGv) -> TCGOp {
        TCGOp {
            op: Some(opcode),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: Some(a3),
            arg3: None,
            label: None,
            helper_idx: 0,
        }
    }

    pub fn new_4op(
        opcode: TCGOpcode,
        a1: TCGv,
        a2: TCGv,
        a3: TCGv,
        label: Rc<RefCell<TCGLabel>>,
    ) -> TCGOp {
        TCGOp {
            op: Some(opcode),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: Some(a3),
            arg3: None,
            label: Some(label),
            helper_idx: 0,
        }
    }

    pub fn new_helper_call_arg0(helper_idx: usize) -> TCGOp {
        TCGOp {
            op: Some(TCGOpcode::HELPER_CALL_ARG0),
            arg0: None,
            arg1: None,
            arg2: None,
            arg3: None,
            label: None,
            helper_idx: helper_idx,
        }
    }

    pub fn new_helper_call_arg1(helper_idx: usize, a1: TCGv) -> TCGOp {
        TCGOp {
            op: Some(TCGOpcode::HELPER_CALL_ARG1),
            arg0: Some(a1),
            arg1: None,
            arg2: None,
            arg3: None,
            label: None,
            helper_idx: helper_idx,
        }
    }

    pub fn new_helper_call_arg2(helper_idx: usize, a1: TCGv, a2: TCGv) -> TCGOp {
        TCGOp {
            op: Some(TCGOpcode::HELPER_CALL_ARG2),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: None,
            arg3: None,
            label: None,
            helper_idx: helper_idx,
        }
    }

    pub fn new_helper_call_arg3(helper_idx: usize, a1: TCGv, a2: TCGv, a3: TCGv) -> TCGOp {
        TCGOp {
            op: Some(TCGOpcode::HELPER_CALL_ARG3),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: Some(a3),
            arg3: None,
            label: None,
            helper_idx: helper_idx,
        }
    }

    pub fn new_helper_call_arg4(
        helper_idx: usize,
        a1: TCGv,
        a2: TCGv,
        a3: TCGv,
        a4: TCGv,
    ) -> TCGOp {
        TCGOp {
            op: Some(TCGOpcode::HELPER_CALL_ARG4),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: Some(a3),
            arg3: Some(a4),
            label: None,
            helper_idx: helper_idx,
        }
    }

    pub fn new_goto_tb(addr: TCGv) -> TCGOp {
        assert_eq!(addr.t, TCGvType::Immediate);

        Self::new_2op(TCGOpcode::MOV_64BIT, TCGv::new_pc(), addr)
    }

    pub fn new_label(label: Rc<RefCell<TCGLabel>>) -> TCGOp {
        TCGOp {
            op: None,
            arg0: None,
            arg1: None,
            arg2: None,
            arg3: None,
            label: Some(label),
            helper_idx: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TCGv {
    pub t: TCGvType,
    pub value: u64,
}

impl TCGv {
    pub fn new_reg(val: u64) -> TCGv {
        TCGv {
            t: TCGvType::Register,
            value: val,
        }
    }

    pub fn new_imm(val: u64) -> TCGv {
        TCGv {
            t: TCGvType::Immediate,
            value: val,
        }
    }

    pub fn new_pc() -> TCGv {
        TCGv {
            t: TCGvType::ProgramCounter,
            value: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TCGLabel {
    pub offset: u64,
    pub code_ptr_vec: Vec<usize>,
}

impl TCGLabel {
    pub fn new() -> TCGLabel {
        TCGLabel {
            offset: 0,
            code_ptr_vec: vec![],
        }
    }
}

pub trait TCG {
    fn tcg_gen(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    fn tcg_gen_add_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sub_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_and_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_or_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_xor_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_jmpr(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_jmpim(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_eq_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_ne_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_lt_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_ge_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_ltu_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_geu_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_mov_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    fn tcg_gen_slt_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sltu_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    fn tcg_gen_add_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sub_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    fn tcg_gen_srl_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sll_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sra_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    fn tcg_gen_srl_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sll_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sra_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    /* Memory Access */
    fn tcg_gen_load(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
        mem_size: MemOpType,
        target_reg: RegisterType,
    ) -> usize;

    fn tcg_gen_store(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
        mem_size: MemOpType,
        target_reg: RegisterType,
    ) -> usize;

    /* Label Relocation */
    fn tcg_out_reloc(host_code_ptr: usize, label: &Rc<RefCell<TCGLabel>>) -> usize;

    fn tcg_gen_label(pc_address: u64, tcg: &TCGOp) -> usize;

    fn tcg_gen_csrrw(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_csrrs(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_csrrc(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    fn tcg_gen_helper_call(
        emu: &EmuEnv,
        arg_size: usize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;

    fn tcg_gen_int_reg_from_float_reg(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_float_reg_from_int_reg(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_int_reg_from_float_reg_32bit(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_float_reg_from_int_reg_32bit(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;

    fn tcg_gen_sgnj_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sgnjn_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sgnjx_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    fn tcg_gen_sgnj_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sgnjn_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sgnjx_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    fn tcg_exit_tb(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
}
