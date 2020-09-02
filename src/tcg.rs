extern crate mmap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TCGOpcode {
    MOV,
    ADD,
    SUB,
    AND,
    OR,
    XOR,
    JMP,
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
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
}

impl TCGOp {
    pub fn new_2op(opcode: TCGOpcode, a1: TCGv, a2: TCGv) -> TCGOp {
        TCGOp {
            op: Some(opcode),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: None,
            label: None,
        }
    }

    pub fn new_3op(opcode: TCGOpcode, a1: TCGv, a2: TCGv, a3: TCGv) -> TCGOp {
        TCGOp {
            op: Some(opcode),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: Some(a3),
            label: None,
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
    fn tcg_gen(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;

    fn tcg_gen_addi(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_sub(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_and(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_or(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_xor(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_ret(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_eq(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_ne(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_lt(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_ge(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_ltu(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_geu(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;
    fn tcg_gen_mov(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize;

    /* Memory Access */
    fn tcg_gen_load(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
        mem_size: MemOpType,
    ) -> usize;

    fn tcg_gen_store(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
        mem_size: MemOpType,
    ) -> usize;

    /* Label Relocation */
    fn tcg_out_reloc(host_code_ptr: usize, label: &mut Rc<RefCell<TCGLabel>>) -> usize;

    fn tcg_gen_label(pc_address: u64, tcg: &mut TCGOp, mc: &mut Vec<u8>) -> usize;
}
