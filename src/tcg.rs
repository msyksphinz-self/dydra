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
    MOV,
    ADD,
    SUB,
    AND,
    OR,
    XOR,
    SRL,
    SLL,
    SRA,
    JMPR,
    JMPIM,
    EQ,
    NE,
    LT,
    GE,
    LTU,
    GEU,
    LD,
    LW,
    LH,
    LB,
    LWU,
    LHU,
    LBU,
    SD,
    SW,
    SH,
    SB,
    CSR_CSRRW,
    CSR_CSRRS,
    CSR_CSRRC,
    ADD_32BIT
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
    pub label: Option<Rc<RefCell<TCGLabel>>>,
    pub helper_idx: usize,
}

impl TCGOp {
    pub fn new_2op(opcode: TCGOpcode, a1: TCGv, a2: TCGv) -> TCGOp {
        TCGOp {
            op: Some(opcode),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: None,
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
            label: None,
            helper_idx: helper_idx,
        }
    }

    pub fn new_helper_call_arg1(helper_idx: usize, a1: TCGv) -> TCGOp {
        TCGOp {
            op: Some(TCGOpcode::HELPER_CALL_ARG0),
            arg0: Some(a1),
            arg1: None,
            arg2: None,
            label: None,
            helper_idx: helper_idx,
        }
    }

    pub fn new_helper_call_arg2(helper_idx: usize, a1: TCGv, a2: TCGv) -> TCGOp {
        TCGOp {
            op: Some(TCGOpcode::HELPER_CALL_ARG0),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: None,
            label: None,
            helper_idx: helper_idx,
        }
    }

    pub fn new_helper_call_arg3(helper_idx: usize, a1: TCGv, a2: TCGv, a3: TCGv) -> TCGOp {
        TCGOp {
            op: Some(TCGOpcode::HELPER_CALL_ARG0),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: Some(a3),
            label: None,
            helper_idx: helper_idx,
        }
    }

    pub fn new_goto_tb(addr: TCGv) -> TCGOp {
        assert_eq!(addr.t, TCGvType::Immediate);

        Self::new_2op(TCGOpcode::MOV, TCGv::new_pc(), addr)
    }

    pub fn new_label(label: Rc<RefCell<TCGLabel>>) -> TCGOp {
        TCGOp {
            op: None,
            arg0: None,
            arg1: None,
            arg2: None,
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

    fn tcg_gen_add(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sub(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_and(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_or(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_xor(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_jmpr(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_jmpim(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_eq(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_ne(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_lt(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_ge(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_ltu(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_geu(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_mov(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    fn tcg_gen_add_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>)-> usize;

    fn tcg_gen_srl(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sll(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;
    fn tcg_gen_sra(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize;

    /* Memory Access */
    fn tcg_gen_load(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
        mem_size: MemOpType,
    ) -> usize;

    fn tcg_gen_store(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
        mem_size: MemOpType,
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

    fn tcg_exit_tb(emu: &EmuEnv, gen_size: usize, mc: &mut Vec<u8>) -> usize;
}
