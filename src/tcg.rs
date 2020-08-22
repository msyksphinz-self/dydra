#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TCGOpcode {
    ADD = 0,
    SUB = 1,
    AND = 2,
    OR = 3,
    XOR = 4,
    JMP = 5,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TCGvType {
    Register = 0,
    Immediate = 1,
}

#[derive(Debug, Copy, Clone)]
pub struct TCGOp {
    pub op: TCGOpcode,
    pub arg0: TCGv,
    pub arg1: TCGv,
    pub arg2: TCGv,
}

impl TCGOp {
    pub fn new(opcode: TCGOpcode, a1: TCGv, a2: TCGv, a3: TCGv) -> TCGOp {
        TCGOp {
            op: opcode,
            arg0: a1,
            arg1: a2,
            arg2: a3,
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
}

pub trait TCG {
    fn tcg_gen_addi(tcg: &TCGOp, mc: &mut Vec<u8>);
    fn tcg_gen_sub(tcg: &TCGOp, mc: &mut Vec<u8>);
    fn tcg_gen_and(tcg: &TCGOp, mc: &mut Vec<u8>);
    fn tcg_gen_or(tcg: &TCGOp, mc: &mut Vec<u8>);
    fn tcg_gen_xor(tcg: &TCGOp, mc: &mut Vec<u8>);
    fn tcg_gen_ret(tcg: &TCGOp, mc: &mut Vec<u8>);
}
