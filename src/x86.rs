#![feature(ptr_offset_from)]

use self::tcg::{MemOpType, TCGLabel, TCGOp, TCGOpcode, TCGvType, TCG};
use super::tcg;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

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
    JA_rel16_32 = 0x87_0f, // JA rel16/32	CF=0 and ZF=0	より上の場合ニアジャンプします
    JAE_rel16_32 = 0x83_0f, // JAE rel16/32	CF=0	より上か等しい場合ニアジャンプします
    JB_rel16_32 = 0x82_0f, // JB rel16/32	CF=1	より下の場合ニアジャンプします
    JBE_rel16_32 = 0x86_0f, // JBE rel16/32	CF=1 or ZF=1	より下か等しい場合ニアジャンプします
    // JC_rel16_32 = 0x82_0f,  // JC rel16/32	CF=1	キャリーがある場合ニアジャンプします
    JE_rel16_32 = 0x84_0f, // JE rel16/32	ZF=1	等しい場合ニアジャンプします
    // JZ_rel16_32 = 0x84_0f,  // JZ rel16/32	ZF=1	ゼロの場合ニアジャンプします
    JG_rel16_32 = 0x8F_0f, // JG_rel16_32	ZF=0 or SF=OF	より大きい場合ニアジャンプします
    JGE_rel16_32 = 0x8D_0f, // JGE_rel16_32	SF=OF	より大きいか等しい場合ニアジャンプします
    JL_rel16_32 = 0x8C_0f, // JL_rel16_32	SF< > OF	より小さい場合ニアジャンプします
    JLE_rel16_32 = 0x8E_0f, // JLE_rel16_32	ZF=1 or SF< > OF	より小さいか等しい場合ニアジャンプします
    // JNA_rel16_32 = 0x86_0f, // JNA_rel16_32	CF=1 or ZF=1	より上でない場合ニアジャンプします
    // JNAE_rel16_32 = 0x82_0f, // JNAE_rel16_32	CF=1	より上でなく等しくない場合ニアジャンプします
    // JNB_rel16_32 = 0x83_0f, // JNB_rel16_32	CF=0	より下でない場合ニアジャンプします
    // JNBE_rel16_32 = 0x87_0f, // JNBE_rel16_32	CF=0 and ZF=0	より下でなく等しくない場合ニアジャンプします
    // JNC_rel16_32 = 0x83_0f,  // JNC_rel16_32	CF=0	キャリーがない場合ニアジャンプします
    JNE_rel16_32 = 0x85_0f, // JNE_rel16_32	ZF=0	等しくない場合ニアジャンプします
    // JNG_rel16_32 = 0x8E_0f,  // JNG_rel16_32	ZF=1 or SF< > OF	より大きくない場合ニアジャンプします
    // JNGE_rel16_32 = 0x8C_0f, // JNGE_rel16_32	SF< > OF	より大きくなく等しくない場合ニアジャンプします
    // JNL_rel16_32 = 0x8D_0f,  // JNL_rel16_32	SF=OF	より小さくない場合ニアジャンプします
    // JNLE_rel16_32 = 0x8F_0f, // JNLE_rel16_32	ZF=0 and SF=OF	より小さくなく等しくない場合ニアジャンプします
    // JNO_rel16_32 = 0x81_0f,  // JNO_rel16_32	OF=0	オーバーフローがない場合ニアジャンプします
    // JNP_rel16_32 = 0x8B_0f,  // JNP_rel16_32	PF=0	パリティがない場合ニアジャンプします
    // JNS_rel16_32 = 0x89_0f,  // JNS_rel16_32	SF=0	符号がない場合ニアジャンプします
    // JNZ_rel16_32 = 0x85_0f,  // JNZ_rel16_32	ZF=0	ゼロでない場合ニアジャンプします
    // JO_rel16_32 = 0x80_0f,   // JO_rel16_32	OF=1	オーバーフローがある場合ニアジャンプします
    // JP_rel16_32 = 0x8A_0f,   // JP_rel16_32	PF=1	パリティがある場合ニアジャンプします
    // JPE_rel16_32 = 0x8A_0f,  // JPE_rel16_32	PF=1	パリティが偶数の場合ニアジャンプします
    // JPO_rel16_32 = 0x8B_0f,  // JPO_rel16_32	PF=0	パリティが奇数の場合ニアジャンプします
    // JS_rel16_32 = 0x88_0f,   // JS_rel16_32	SF=1	符号がある場合ニアジャンプします
    // JZ_rel16_32 = 0x84_0f,   // JZ_rel16_32	ZF=1	ゼロの場合ニアジャンプします
    ADD_EV_GV = 0x01,
    MOV_GV_EV_32BIT = 0x63,
    MOV_GV_EV_S_16BIT = 0xbf0f,
    MOV_GV_EV_S_8BIT = 0xbe0f,
    MOV_GV_EV_U_16BIT = 0xb70f,
    MOV_GV_EV_U_8BIT = 0xb60f,
}

enum X86_2Wd_Opcode {}

#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
enum X86ModRM {
    MOD_00_DISP_RBP = 0x05,
    MOD_01_DISP_RBP = 0x45,
    MOD_10_DISP_RBP = 0x85,
    MOD_11_DISP_RBP = 0xc5,
    MOD_10_DISP_RSI = 0x86,
    MOD_00_DISP_RSI = 0x00,
    MOD_10_DISP_RAX = 0x80,
    MOD_11_DISP_RSI = 0xf0,
    MOD_11_DISP_RDX = 0xd0,
}

macro_rules! conv_gpr_offset {
    ($gpr_addr: expr) => {
        ($gpr_addr as u32) * 8
    };
}

pub struct TCGX86;

impl TCGX86 {
    fn tcg_modrm_64bit_out(op: X86Opcode, modrm: X86ModRM, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(((modrm as u32) << 16) | (op as u32) << 8 | 0x48, 3, mc);
        return 3;
    }

    fn tcg_modrm_2byte_64bit_out(op: X86Opcode, modrm: X86ModRM, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(((modrm as u32) << 24) | (op as u32) << 8 | 0x48, 4, mc);
        return 4;
    }

    fn tcg_modrm_32bit_out(op: X86Opcode, modrm: X86ModRM, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(((modrm as u32) << 8) | (op as u32) << 0, 2, mc);
        return 2;
    }

    fn tcg_modrm_2byte_32bit_out(op: X86Opcode, modrm: X86ModRM, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(((modrm as u32) << 16) | (op as u32) << 0, 3, mc);
        return 3;
    }

    fn tcg_gen_rrr(op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Register);

        let mut gen_size: usize = 0;

        // mov    reg_offset(%rbp),%eax
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, mc);
        gen_size += Self::tcg_out(conv_gpr_offset!(arg1.value), 4, mc);

        // add    reg_offset(%rbp),%eax
        gen_size += Self::tcg_modrm_64bit_out(op, X86ModRM::MOD_10_DISP_RBP, mc);
        gen_size += Self::tcg_out(conv_gpr_offset!(arg2.value), 4, mc);

        // mov    %eax,reg_offset(%rbp)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
        gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);

        return gen_size;
    }

    fn tcg_gen_rri(op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Immediate);

        let mut gen_size: usize = 0;

        // mov    reg_offset(%rbp),%eax
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, mc);
        gen_size += Self::tcg_out(conv_gpr_offset!(arg1.value), 4, mc);

        // add    imm16,%eax
        gen_size += Self::tcg_out(op as u32, 1, mc);
        gen_size += Self::tcg_out(arg2.value as u32, 4, mc);

        // mov    %eax,reg_offset(%rbp)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
        gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);

        return gen_size;
    }

    fn tcg_out(inst: u32, byte_len: usize, v: &mut Vec<u8>) -> usize {
        for (i, be) in inst.to_le_bytes().iter().enumerate() {
            if i < byte_len {
                // println!("register = {:02x}", &be);
                v.push(*be);
            }
        }
        return byte_len;
    }

    fn tcg_gen_jcc(
        gen_size: usize,
        x86_op: X86Opcode,
        mc: &mut Vec<u8>,
        label: &mut Rc<RefCell<tcg::TCGLabel>>,
    ) -> usize {
        let mut gen_size = gen_size;

        gen_size += Self::tcg_out(x86_op as u32, 2, mc);
        gen_size += Self::tcg_out(10 as u32, 4, mc);
        gen_size += Self::tcg_out_reloc(gen_size - 4, label);

        return gen_size;
    }

    fn tcg_gen_cmp_branch(
        diff_from_epilogue: isize,
        pc_address: u64,
        x86_op: X86Opcode,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();
        let mut label = match &mut tcg.label {
            Some(l) => l,
            None => panic!("Label is not defined."),
        };

        let mut gen_size: usize = pc_address as usize;

        // mov    reg_offset(%rbp),%eax
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, mc);
        gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);

        // cmp    reg_offset(%rbp),%eax
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::CMP_GV_EV, X86ModRM::MOD_10_DISP_RBP, mc);
        gen_size += Self::tcg_out(conv_gpr_offset!(arg1.value), 4, mc);

        gen_size = Self::tcg_gen_jcc(gen_size, x86_op, mc, label);
        // // je     label

        return gen_size;
    }
}

impl TCG for TCGX86 {
    fn tcg_gen(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        match tcg.op {
            Some(op) => {
                return match op {
                    TCGOpcode::ADD => TCGX86::tcg_gen_addi(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::SUB => TCGX86::tcg_gen_sub(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::AND => TCGX86::tcg_gen_and(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::OR => TCGX86::tcg_gen_or(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::XOR => TCGX86::tcg_gen_xor(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::JMP => TCGX86::tcg_gen_ret(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::EQ => TCGX86::tcg_gen_eq(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::NE => TCGX86::tcg_gen_ne(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::LT => TCGX86::tcg_gen_lt(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::GE => TCGX86::tcg_gen_ge(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::LTU => TCGX86::tcg_gen_ltu(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::GEU => TCGX86::tcg_gen_geu(diff_from_epilogue, pc_address, tcg, mc),
                    TCGOpcode::LD => TCGX86::tcg_gen_load(
                        diff_from_epilogue,
                        pc_address,
                        tcg,
                        mc,
                        MemOpType::LOAD_64BIT,
                    ),
                    TCGOpcode::LW => {
                        println!("Load LW");
                        TCGX86::tcg_gen_load(
                            diff_from_epilogue,
                            pc_address,
                            tcg,
                            mc,
                            MemOpType::LOAD_32BIT,
                        )
                    }
                    TCGOpcode::LH => TCGX86::tcg_gen_load(
                        diff_from_epilogue,
                        pc_address,
                        tcg,
                        mc,
                        MemOpType::LOAD_16BIT,
                    ),
                    TCGOpcode::LB => TCGX86::tcg_gen_load(
                        diff_from_epilogue,
                        pc_address,
                        tcg,
                        mc,
                        MemOpType::LOAD_8BIT,
                    ),
                    TCGOpcode::LWU => TCGX86::tcg_gen_load(
                        diff_from_epilogue,
                        pc_address,
                        tcg,
                        mc,
                        MemOpType::LOAD_U_32BIT,
                    ),
                    TCGOpcode::LHU => TCGX86::tcg_gen_load(
                        diff_from_epilogue,
                        pc_address,
                        tcg,
                        mc,
                        MemOpType::LOAD_U_16BIT,
                    ),
                    TCGOpcode::LBU => TCGX86::tcg_gen_load(
                        diff_from_epilogue,
                        pc_address,
                        tcg,
                        mc,
                        MemOpType::LOAD_U_8BIT,
                    ),
                    TCGOpcode::SD => TCGX86::tcg_gen_store(
                        diff_from_epilogue,
                        pc_address,
                        tcg,
                        mc,
                        MemOpType::STORE_64BIT,
                    ),
                    TCGOpcode::SW => TCGX86::tcg_gen_store(
                        diff_from_epilogue,
                        pc_address,
                        tcg,
                        mc,
                        MemOpType::STORE_32BIT,
                    ),
                    TCGOpcode::SH => TCGX86::tcg_gen_store(
                        diff_from_epilogue,
                        pc_address,
                        tcg,
                        mc,
                        MemOpType::STORE_16BIT,
                    ),
                    TCGOpcode::SB => TCGX86::tcg_gen_store(
                        diff_from_epilogue,
                        pc_address,
                        tcg,
                        mc,
                        MemOpType::STORE_8BIT,
                    ),

                    TCGOpcode::MOV => TCGX86::tcg_gen_mov(diff_from_epilogue, pc_address, tcg, mc),
                    other => panic!("{:?} : Not supported now", other),
                };
            }
            None => match &mut tcg.label {
                Some(_l) => TCGX86::tcg_gen_label(pc_address, tcg, mc),
                None => panic!("Illegal Condition"),
            },
        }
    }

    fn tcg_gen_addi(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &tcg::TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        if arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return gen_size;
        }

        if arg2.t == tcg::TCGvType::Immediate {
            if arg1.value == 0 {
                // if source register is x0, just generate immediate value.
                // movl   imm,reg_addr(%rbp)
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_IV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                return gen_size;
            }

            gen_size += Self::tcg_gen_rri(X86Opcode::ADD_EAX_IV, tcg, mc);
            return gen_size;
        } else {
            if arg1.value == 0 {
                // if source register is x0, just mov gpr value.
                // movl   reg_addr(%rbp),%eax
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg2.value), 4, mc);
                // movl   %eax,reg_addr(%rbp)
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                return gen_size;
            }
            gen_size += Self::tcg_gen_rrr(X86Opcode::ADD_GV_EV, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_sub(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let arg0 = tcg.arg0.unwrap();

        let mut gen_size: usize = pc_address as usize;

        if arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return gen_size;
        }
        gen_size += Self::tcg_gen_rrr(X86Opcode::SUB_GV_EV, tcg, mc);
        return gen_size;
    }

    fn tcg_gen_and(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        if arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return gen_size;
        }
        if arg2.t == tcg::TCGvType::Immediate {
            if arg1.value == 0 {
                // if source register is x0, just generate immediate value.
                // movl   imm,reg_addr(%rbp)
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_IV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                return gen_size;
            }

            Self::tcg_gen_rri(X86Opcode::AND_EAX_IV, tcg, mc);
            return gen_size;
        } else {
            if arg1.value == 0 {
                // if source register is x0, just mov gpr value.
                // movl   reg_addr(%rbp),%eax
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg2.value), 4, mc);
                // movl   %eax,reg_addr(%rbp)
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                return gen_size;
            }
            gen_size += Self::tcg_gen_rrr(X86Opcode::AND_GV_EV, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_or(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        if arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return gen_size;
        }
        if arg2.t == tcg::TCGvType::Immediate {
            if arg1.value == 0 {
                // if source register is x0, just generate immediate value.
                // movl   imm,reg_addr(%rbp)
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_IV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                return gen_size;
            }

            gen_size += Self::tcg_gen_rri(X86Opcode::OR_EAX_IV, tcg, mc);
            return gen_size;
        } else {
            if arg1.value == 0 {
                // if source register is x0, just mov gpr value.
                // movl   reg_addr(%rbp),%eax
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg2.value), 4, mc);
                // movl   %eax,reg_addr(%rbp)
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                return gen_size;
            }
            gen_size += Self::tcg_gen_rrr(X86Opcode::OR_GV_EV, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_xor(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        if arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return gen_size;
        }

        if arg2.t == tcg::TCGvType::Immediate {
            if arg1.value == 0 {
                // if source register is x0, just generate immediate value.
                // movl   imm,reg_addr(%rbp)
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_IV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                return gen_size;
            }

            gen_size += Self::tcg_gen_rri(X86Opcode::XOR_EAX_IV, tcg, mc);
            return gen_size;
        } else {
            if arg1.value == 0 {
                // if source register is x0, just mov gpr value.
                // movl   reg_addr(%rbp),%eax
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg2.value), 4, mc);
                // movl   %eax,reg_addr(%rbp)
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
                gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);
                return gen_size;
            }
            gen_size += Self::tcg_gen_rrr(X86Opcode::XOR_GV_EV, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_ret(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(op, TCGOpcode::JMP);

        let mut gen_size: usize = pc_address as usize;

        if arg0.t == tcg::TCGvType::Register
            && arg0.value == 0
            && arg1.t == tcg::TCGvType::Register
            && arg1.value == 1
        {
            gen_size += Self::tcg_out(X86Opcode::JMP_JZ as u32, 1, mc);
            gen_size += Self::tcg_out((diff_from_epilogue - gen_size as isize - 4) as u32, 4, mc);

            return gen_size;
        }
        // xxx : panic!("This function is not supported!")
        return 0;
    }

    fn tcg_gen_eq(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        return Self::tcg_gen_cmp_branch(
            diff_from_epilogue,
            pc_address,
            X86Opcode::JE_rel16_32,
            tcg,
            mc,
        );
    }

    fn tcg_gen_ne(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        return Self::tcg_gen_cmp_branch(
            diff_from_epilogue,
            pc_address,
            X86Opcode::JNE_rel16_32,
            tcg,
            mc,
        );
    }

    fn tcg_gen_lt(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        return Self::tcg_gen_cmp_branch(
            diff_from_epilogue,
            pc_address,
            X86Opcode::JL_rel16_32,
            tcg,
            mc,
        );
    }

    fn tcg_gen_ge(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        return Self::tcg_gen_cmp_branch(
            diff_from_epilogue,
            pc_address,
            X86Opcode::JGE_rel16_32,
            tcg,
            mc,
        );
    }

    fn tcg_gen_ltu(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        return Self::tcg_gen_cmp_branch(
            diff_from_epilogue,
            pc_address,
            X86Opcode::JB_rel16_32,
            tcg,
            mc,
        );
    }

    fn tcg_gen_geu(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &mut TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        return Self::tcg_gen_cmp_branch(
            diff_from_epilogue,
            pc_address,
            X86Opcode::JAE_rel16_32,
            tcg,
            mc,
        );
    }

    fn tcg_gen_mov(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        assert_eq!(op, TCGOpcode::MOV);
        assert_eq!(arg0.t, TCGvType::ProgramCounter);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_out(X86Opcode::MOV_EAX_IV as u32, 1, mc);
        gen_size += Self::tcg_out(arg1.value as u32, 4, mc);

        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
        gen_size += Self::tcg_out(8 * 32, 4, mc); // Set Program Counter

        // jmp    epilogue
        gen_size += Self::tcg_out(X86Opcode::JMP_JZ as u32, 1, mc);
        gen_size += Self::tcg_out((diff_from_epilogue - gen_size as isize - 4) as u32, 4, mc);
        return gen_size;
    }

    fn tcg_out_reloc(host_code_ptr: usize, label: &mut Rc<RefCell<TCGLabel>>) -> usize {
        let mut l = &mut *label.borrow_mut();
        l.code_ptr_vec.push(host_code_ptr);
        println!("Added offset. code_ptr = {:x}", host_code_ptr);
        return 0;
    }

    fn tcg_gen_label(pc_address: u64, tcg: &mut TCGOp, mc: &mut Vec<u8>) -> usize {
        match &mut tcg.label {
            Some(label) => {
                let mut l = &mut *label.borrow_mut();
                l.offset = pc_address;
                println!("Offset is set {:x}", l.offset);
            }
            None => panic!("Unknown behavior"),
        }
        return 0;
    }

    /* Memory Access : Load */
    fn tcg_gen_load(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
        mem_size: MemOpType,
    ) -> usize {
        let mut gen_size: usize = pc_address as usize;

        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Immediate);

        // Load value from rs1
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, mc);
        gen_size += Self::tcg_out(conv_gpr_offset!(arg1.value), 4, mc);

        // Execute Load
        // GPR value + Memory Head Address
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_EV_GV, X86ModRM::MOD_11_DISP_RDX, mc); // ADD RSI+EAX=EAX
        gen_size += match mem_size {
            MemOpType::LOAD_64BIT => {
                let mut gen_size = 0;
                gen_size +=
                    Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RAX, mc);
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                gen_size
            }
            MemOpType::LOAD_32BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_64bit_out(
                    X86Opcode::MOV_GV_EV_32BIT,
                    X86ModRM::MOD_10_DISP_RAX,
                    mc,
                );
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                gen_size
            }
            MemOpType::LOAD_16BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_2byte_64bit_out(
                    X86Opcode::MOV_GV_EV_S_16BIT,
                    X86ModRM::MOD_10_DISP_RAX,
                    mc,
                );
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                gen_size
            }
            MemOpType::LOAD_8BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_2byte_64bit_out(
                    X86Opcode::MOV_GV_EV_S_8BIT,
                    X86ModRM::MOD_10_DISP_RAX,
                    mc,
                );
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                gen_size
            }
            MemOpType::LOAD_U_32BIT => {
                let mut gen_size = 0;
                gen_size +=
                    Self::tcg_modrm_32bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RAX, mc);
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                gen_size
            }
            MemOpType::LOAD_U_16BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_2byte_32bit_out(
                    X86Opcode::MOV_GV_EV_U_16BIT,
                    X86ModRM::MOD_10_DISP_RAX,
                    mc,
                );
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                gen_size
            }
            MemOpType::LOAD_U_8BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_2byte_32bit_out(
                    X86Opcode::MOV_GV_EV_U_8BIT,
                    X86ModRM::MOD_10_DISP_RAX,
                    mc,
                );
                gen_size += Self::tcg_out(arg2.value as u32, 4, mc);
                gen_size
            }
            _ => panic!("Not supported load instruction."),
        };

        // Store Loaded value into destination register.
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, mc);
        gen_size += Self::tcg_out(conv_gpr_offset!(arg0.value), 4, mc);

        return gen_size;
    }

    /* Memory Access : Store */
    fn tcg_gen_store(
        diff_from_epilogue: isize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
        mem_size: MemOpType,
    ) -> usize {
        return 0;
    }
}
