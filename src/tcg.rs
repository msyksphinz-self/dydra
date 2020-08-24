#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TCGOpcode {
    MOV,
    ADD,
    SUB, 
    AND, 
    OR, 
    XOR, 
    JMP, 
    EQ ,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TCGvType {
    Register,
    Immediate,
    ProgramCounter,
}

#[derive(Debug, Copy, Clone)]
pub struct TCGOp {
    pub op: Option<TCGOpcode>,
    pub arg0: Option<TCGv>,
    pub arg1: Option<TCGv>,
    pub arg2: Option<TCGv>,
    pub label: Option<TCGLabel>,
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

    pub fn new_4op(opcode: TCGOpcode, a1: TCGv, a2: TCGv, label: TCGLabel) -> TCGOp {
        TCGOp {
            op: Some(opcode),
            arg0: Some(a1),
            arg1: Some(a2),
            arg2: None,
            label: Some(label),
        }
    }

    pub fn new_goto_tb(addr: TCGv) -> TCGOp {
        assert_eq!(addr.t, TCGvType::Immediate);

        Self::new_2op(TCGOpcode::MOV, TCGv::new_pc(), addr)
    }

    pub fn new_label(label: TCGLabel) -> TCGOp {
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

#[derive(Debug, Copy, Clone)]
pub struct TCGLabel;

impl TCGLabel {
    pub fn new() -> TCGLabel {
        TCGLabel {}
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
