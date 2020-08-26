#![feature(ptr_offset_from)]

use self::tcg::{TCGOp, TCGOpcode, TCGvType, TCG};
use super::tcg;
use std::mem;

extern crate mmap;

#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
enum X86Opcode {
    MOV_EV_IV = 0xc7,
    MOV_GV_EV = 0x8b,
    MOV_EV_GV = 0x89,
    ADD_EV_IV = 0x81,
    ADD_GV_EV = 0x03,
    ADD_EAX_IV = 0x05,
    SUB_GV_EV = 0x2b,
    AND_GV_EV = 0x23,
    OR_GV_EV = 0x0b,
    XOR_GV_EV = 0x33,
    AND_EAX_IV = 0x25,
    OR_EAX_IV = 0x0d,
    XOR_EAX_IV = 0x35,
    CMP_GV_EV = 0x3b,
    MOV_EAX_IV = 0xb8,
    RETN = 0xc3,
    JMP_JZ = 0xe9,
}

enum X86_2Wd_Opcode {}

#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
enum X86ModRM {
    MOD_00 = 0x05,
    MOD_01 = 0x45,
    MOD_10 = 0x85,
    MOD_11 = 0xc5,
}

macro_rules! conv_gpr_offset {
    ($gpr_addr: expr) => {
        ($gpr_addr as u32) * 8
    };
}

pub struct TCGX86;

impl TCGX86 {
    fn tcg_modrm_out(op: X86Opcode, modrm: X86ModRM, mc: &mut Vec<u8>) {
        Self::tcg_out(((modrm as u32) << 16) | (op as u32) << 8 | 0x48, 3, mc);
    }

    fn tcg_gen_rrr(op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        // mov    reg_offset(%rbp),%eax
        Self::tcg_modrm_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10, mc);
        Self::tcg_out(conv_gpr_offset!(arg1.value), 4, mc);

        // add    reg_offset(%rbp),%eax
        Self::tcg_modrm_out(op, X86ModRM::MOD_10, mc);
        Self::tcg_out(conv_gpr_offset!(arg2.value), 4, mc);

        // mov    %eax,reg_offset(%rbp)
        Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
        Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
    }

    fn tcg_gen_rri(op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Immediate);

        // mov    reg_offset(%rbp),%eax
        Self::tcg_modrm_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10, mc);
        Self::tcg_out(conv_gpr_offset!(arg1.value), 4, mc);

        // add    imm16,%eax
        Self::tcg_out(op as u32, 1, mc);
        Self::tcg_out(arg2.value as u32, 4, mc);

        // mov    %eax,reg_offset(%rbp)
        Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
        Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
    }

    fn tcg_out(inst: u32, byte_len: usize, v: &mut Vec<u8>) {
        for (i, be) in inst.to_le_bytes().iter().enumerate() {
            if i < byte_len {
                println!("register = {:02x}", &be);
                v.push(*be);
            }
        }
    }
}

impl TCG for TCGX86 {
    fn tcg_gen(
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
        pe_map: &mmap::MemoryMap,
        tb_map: &mmap::MemoryMap,
    ) {
        match tcg.op {
            Some(op) => {
                return match op {
                    TCGOpcode::ADD => TCGX86::tcg_gen_addi(pc_address, tcg, mc),
                    TCGOpcode::SUB => TCGX86::tcg_gen_sub(pc_address, tcg, mc),
                    TCGOpcode::AND => TCGX86::tcg_gen_and(pc_address, tcg, mc),
                    TCGOpcode::OR => TCGX86::tcg_gen_or(pc_address, tcg, mc),
                    TCGOpcode::XOR => TCGX86::tcg_gen_xor(pc_address, tcg, mc),
                    TCGOpcode::JMP => TCGX86::tcg_gen_ret(pc_address, tcg, mc, pe_map, tb_map),
                    TCGOpcode::EQ => TCGX86::tcg_gen_eq(pc_address, tcg, mc),
                    TCGOpcode::MOV => TCGX86::tcg_gen_mov(pc_address, tcg, mc),
                    other => panic!("{:?} : Not supported now", other),
                };
            }
            None => {}
        }
        assert!(tcg.label.is_some());
    }

    fn tcg_gen_addi(pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        if arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return;
        }

        if arg2.t == tcg::TCGvType::Immediate {
            if arg1.value == 0 {
                // if source register is x0, just generate immediate value.
                // movl   imm,reg_addr(%rbp)
                Self::tcg_modrm_out(X86Opcode::MOV_EV_IV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                Self::tcg_out(arg2.value as u32, 4, mc);
                return;
            }

            Self::tcg_gen_rri(X86Opcode::ADD_EAX_IV, tcg, mc);
            return;
        } else {
            if arg1.value == 0 {
                // if source register is x0, just mov gpr value.
                // movl   reg_addr(%rbp),%eax
                Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg2.value), 4, mc);
                // movl   %eax,reg_addr(%rbp)
                Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                return;
            }
            Self::tcg_gen_rrr(X86Opcode::ADD_GV_EV, tcg, mc);
            return;
        }
    }

    fn tcg_gen_sub(pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) {
        let arg0 = tcg.arg0.unwrap();

        if arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return;
        }
        Self::tcg_gen_rrr(X86Opcode::SUB_GV_EV, tcg, mc);
    }

    fn tcg_gen_and(pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        if arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return;
        }
        if arg2.t == tcg::TCGvType::Immediate {
            if arg1.value == 0 {
                // if source register is x0, just generate immediate value.
                // movl   imm,reg_addr(%rbp)
                Self::tcg_modrm_out(X86Opcode::MOV_EV_IV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                Self::tcg_out(arg2.value as u32, 4, mc);
                return;
            }

            Self::tcg_gen_rri(X86Opcode::AND_EAX_IV, tcg, mc);
            return;
        } else {
            if arg1.value == 0 {
                // if source register is x0, just mov gpr value.
                // movl   reg_addr(%rbp),%eax
                Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg2.value), 4, mc);
                // movl   %eax,reg_addr(%rbp)
                Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                return;
            }
            Self::tcg_gen_rrr(X86Opcode::AND_GV_EV, tcg, mc);
            return;
        }
    }

    fn tcg_gen_or(pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        if arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return;
        }
        if arg2.t == tcg::TCGvType::Immediate {
            if arg1.value == 0 {
                // if source register is x0, just generate immediate value.
                // movl   imm,reg_addr(%rbp)
                Self::tcg_modrm_out(X86Opcode::MOV_EV_IV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                Self::tcg_out(arg2.value as u32, 4, mc);
                return;
            }

            Self::tcg_gen_rri(X86Opcode::OR_EAX_IV, tcg, mc);
            return;
        } else {
            if arg1.value == 0 {
                // if source register is x0, just mov gpr value.
                // movl   reg_addr(%rbp),%eax
                Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg2.value), 4, mc);
                // movl   %eax,reg_addr(%rbp)
                Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                return;
            }
            Self::tcg_gen_rrr(X86Opcode::OR_GV_EV, tcg, mc);
            return;
        }
    }

    fn tcg_gen_xor(pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        if arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return;
        }

        if arg2.t == tcg::TCGvType::Immediate {
            if arg1.value == 0 {
                // if source register is x0, just generate immediate value.
                // movl   imm,reg_addr(%rbp)
                Self::tcg_modrm_out(X86Opcode::MOV_EV_IV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                Self::tcg_out(arg2.value as u32, 4, mc);
                return;
            }

            Self::tcg_gen_rri(X86Opcode::XOR_EAX_IV, tcg, mc);
            return;
        } else {
            if arg1.value == 0 {
                // if source register is x0, just mov gpr value.
                // movl   reg_addr(%rbp),%eax
                Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg2.value), 4, mc);
                // movl   %eax,reg_addr(%rbp)
                Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
                Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                return;
            }
            Self::tcg_gen_rrr(X86Opcode::XOR_GV_EV, tcg, mc);
            return;
        }
    }

    fn tcg_gen_ret(
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
        pe_map: &mmap::MemoryMap,
        tb_map: &mmap::MemoryMap,
    ) {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(op, TCGOpcode::JMP);

        if arg0.t == tcg::TCGvType::Register
            && arg0.value == 0
            && arg1.t == tcg::TCGvType::Register
            && arg1.value == 1
        {
            Self::tcg_out(X86Opcode::JMP_JZ as u32, 1, mc);
            let tb_map_ptr = tb_map.data() as *const u64;
            let pe_map_ptr = pe_map.data() as *const u64;

            let mut addr_diff = unsafe { pe_map_ptr.offset_from(tb_map_ptr) };
            addr_diff *= 8;
            addr_diff += 32;
            addr_diff -= pc_address as isize;
            addr_diff -= 5;
            Self::tcg_out(addr_diff as u32, 4, mc);

            println!("tb_map_u64 = {:p}", unsafe { tb_map_ptr.offset(0) });
            println!("pe_map_u64 = {:p}", unsafe { pe_map_ptr.offset(0) });
            println!("addr_diff = {:x}", addr_diff);

            println!("tb_map_u64 = {:p}", tb_map.data());
            println!("pe_map_u64 = {:p}", pe_map.data());

            return;
        }
        panic!("This function is not supported!")
    }

    fn tcg_gen_eq(pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let label = tcg.label.unwrap();

        // mov    reg_offset(%rbp),%eax
        Self::tcg_modrm_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10, mc);
        Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);

        // cmp    reg_offset(%rbp),%eax
        Self::tcg_modrm_out(X86Opcode::CMP_GV_EV, X86ModRM::MOD_10, mc);
        Self::tcg_out(conv_gpr_offset!(arg1.value), 4, mc);

        Self::tcg_out(0x84_0f, 2, mc);
        Self::tcg_out(label.offset as u32, 4, mc);
    }

    fn tcg_gen_mov(pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        assert_eq!(op, TCGOpcode::MOV);
        assert_eq!(arg0.t, TCGvType::ProgramCounter);

        Self::tcg_out(X86Opcode::MOV_EAX_IV as u32, 1, mc);
        Self::tcg_out(arg1.value as u32, 4, mc);

        Self::tcg_modrm_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10, mc);
        Self::tcg_out(8 * 32, 4, mc); // Set Program Counter
    }
}
