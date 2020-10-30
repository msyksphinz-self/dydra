use self::tcg::{MemOpType, RegisterType, TCGLabel, TCGOp, TCGOpcode, TCGvType, TCG};
use super::super::tcg;
use std::cell::RefCell;
use std::rc::Rc;

use crate::emu_env::EmuEnv;
use crate::target::riscv::riscv::CALL_HELPER_IDX;

extern crate mmap;

#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types, dead_code)]
enum X86Opcode {
    MOV_EV_IV = 0xc7,
    MOV_GV_EV = 0x8b,
    MOV_EB_GB = 0x88,
    MOV_EV_GV = 0x89,
    // ADD_GV_IMM /* ADD_EV_IV */ = 0x81,
    ADD_GV_EV = 0x03,
    ADD_EAX_IV = 0x05,
    SUB_GV_EV = 0x2b,
    SUB_EAX_IV = 0x2d,
    AND_GV_EV = 0x23,
    OR_GV_EV = 0x0b,
    OR_EV_GV = 0x09,
    XOR_GV_EV = 0x33,
    AND_EAX_IV = 0x25,
    OR_EAX_IV = 0x0d,
    XOR_EAX_IV = 0x35,
    NEG_GV = 0xf7,
    CMP_GV_EV = 0x3b,
    MOV_EAX_IV = 0xb8,
    SLL_GV_CL = 0x20_d3,
    SRL_GV_CL = 0x28_d3,
    SRA_GV_CL = 0x38_d3,
    SLL_GV_IMM = 0x20_c1,
    SRL_GV_IMM = 0x28_c1,
    SRA_GV_IMM = 0x38_c1,
    SIGN_EXT_A = 0x98,
    RETN = 0xc3,
    JMP_JZ = 0xe9,
    CALL = 0xff,
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

    ADD_GV_IMM = 0x00_81,
    SUB_GV_IMM = 0x28_81,
    AND_GV_IMM = 0x20_81,
    OR_GV_IMM  = 0x08_81,
    XOR_GV_IMM = 0x30_81,

    IMUL_RDX_RAX_R = 0xaf_0f,
    IDIV_RDX_RAX_R = 0x38_f7,

    // MOVSXD = 0x63,

    CQO = 0x99,

    SETB = 0x92_0f, // より下の場合バイトを設定します
    SETL = 0x9c_0f, // より小さい場合バイトを設定します
}

#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types, dead_code)]
enum X86ModRM {
    MOD_00_DISP_RAX = (0b00 << 6) | (X86TargetRM::RAX as isize), 
    MOD_00_DISP_RSI = 0x06,
    MOD_00_DISP_RBP = 0x05,
    MOD_00_DISP_RCX = (0b00 << 6) | (X86TargetRM::RCX as isize), 
    MOD_00_DISP_RDX = (0b00 << 6) | (X86TargetRM::RDX as isize), 

    MOD_01_DISP_RBP = 0x45,
    MOD_10_DISP_RBP = 0x85,
    MOD_10_DISP_RSI = 0x86,
    MOD_10_DISP_RDX = 0b10_000_010,
    MOD_10_DISP_RAX = 0x80,
    MOD_10_DISP_RDI = 0x87,

    MOD_11_DISP_RBP = 0xc5,
    MOD_11_DISP_RSI = 0xc6,
    MOD_11_DISP_RDX = 0xc2,
    MOD_11_DISP_RCX = 0xc1,
    MOD_11_DISP_RAX = 0xc0,
}

#[derive(PartialEq, Debug, Copy, Clone)]
#[allow(dead_code)]
enum X86TargetRM {
    RAX = 0b000,
    RCX = 0b001,
    RDX = 0b010,
    RBX = 0b011,
    SIB = 0b100,
    RIP = 0b101,
    RSI = 0b110,
    RDI = 0b111,
    R8 = 0b1000,
}

pub struct TCGX86;

impl TCGX86 {
    fn tcg_modrm_64bit_out(op: X86Opcode, modrm: X86ModRM, tgt_rm: X86TargetRM, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(
            ((modrm as u64 | ((tgt_rm as u64) << 3)) << 16) | (op as u64) << 8 | 0x48,
            3,
            mc,
        );
        return 3;
    }

    fn tcg_modrm_64bit_raw_out(op: X86Opcode, modrm: u8, tgt_rm: u8, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(
            ((modrm as u64 | ((tgt_rm as u64) << 3)) << 16) | (op as u64) << 8 | 0x48,
            3,
            mc,
        );
        return 3;
    }

    fn tcg_64bit_out(op: X86Opcode, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(((op as u64) << 8) | 0x48 << 0, 2, mc);
        return 2;
    }

    fn tcg_modrm_2byte_64bit_out(op: X86Opcode, modrm: X86ModRM, tgt_rm: X86TargetRM, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(
            ((modrm as u64 | ((tgt_rm as u64) << 3)) << 24) | (op as u64) << 8 | 0x48,
            4,
            mc,
        );
        return 4;
    }

    fn tcg_modrm_2byte_64bit_raw_out(op: X86Opcode, modrm: u8, tgt_rm: u8, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(
            ((modrm as u64 | ((tgt_rm as u64) << 3)) << 24) | (op as u64) << 8 | 0x48,
            4,
            mc,
        );
        return 4;
    }

    fn tcg_modrm_32bit_out(op: X86Opcode, modrm: X86ModRM, tgt_rm: X86TargetRM, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(
            ((modrm as u64 | ((tgt_rm as u64) << 3)) << 8) | (op as u64) << 0,
            2,
            mc,
        );
        return 2;
    }

    fn tcg_modrm_32bit_raw_out(op: X86Opcode, modrm: u8, tgt_rm: u8, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(
            ((modrm as u64 | ((tgt_rm as u64) << 3)) << 8) | (op as u64) << 0,
            2,
            mc,
        );
        return 2;
    }


    fn tcg_modrm_16bit_out(op: X86Opcode, modrm: X86ModRM, tgt_rm: X86TargetRM, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(
            ((modrm as u64 | ((tgt_rm as u64) << 3)) << 16) | (op as u64) << 8 | 0x66,
            3,
            mc,
        );
        return 3;
    }

    fn tcg_modrm_2byte_32bit_out(op: X86Opcode, modrm: X86ModRM, tgt_rm: X86TargetRM, mc: &mut Vec<u8>) -> usize {
        Self::tcg_out(
            ((modrm as u64 | ((tgt_rm as u64) << 3)) << 16) | (op as u64) << 0,
            3,
            mc,
        );
        return 3;
    }

    fn tcg_gen_mov_gpr_imm_64bit(emu: &EmuEnv, dest: u64, imm: u64, mc: &mut Vec<u8>) -> usize {
        let mut gen_size = 0;

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, imm, mc);

        gen_size += Self::tcg_modrm_64bit_out(
            X86Opcode::MOV_EV_GV,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RAX,
            mc,
        );
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(dest) as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_mov_gpr_gpr_64bit(emu: &EmuEnv, dest: u64, source: u64, mc: &mut Vec<u8>) -> usize {
        let mut gen_size = 0;
        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, source, mc);
        gen_size += Self::tcg_gen_store_gpr_64bit(emu, X86TargetRM::RAX, dest, mc);
        return gen_size;
    }

    fn tcg_gen_load_gpr_64bit(emu: &EmuEnv, dest: X86TargetRM, source: u64, mc: &mut Vec<u8>) -> usize {
        let mut gen_size = 0;
        gen_size +=
            Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, dest, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(source) as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_store_gpr_64bit(
        emu: &EmuEnv,
        source: X86TargetRM,
        dest: u64,
        mc: &mut Vec<u8>,
    ) -> usize {
        let mut gen_size = 0;
        gen_size +=
            Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, source, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(dest) as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_load_gpr_32bit(emu: &EmuEnv, dest: X86TargetRM, source: u64, mc: &mut Vec<u8>) -> usize {
        let mut gen_size = 0;
        gen_size +=
            Self::tcg_modrm_32bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, dest, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(source) as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_store_gpr_32bit(
        emu: &EmuEnv,
        source: X86TargetRM,
        dest: u64,
        mc: &mut Vec<u8>,
    ) -> usize {
        let mut gen_size = 0;

        gen_size += Self::tcg_64bit_out(X86Opcode::SIGN_EXT_A, mc);
        gen_size +=
            Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, source, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(dest) as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_load_fregs_64bit(emu: &EmuEnv, dest: X86TargetRM, source: u64, mc: &mut Vec<u8>) -> usize {
        let mut gen_size = 0;
        gen_size +=
            Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, dest, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(source) as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_store_fregs_64bit(
        emu: &EmuEnv,
        source: X86TargetRM,
        dest: u64,
        mc: &mut Vec<u8>,
    ) -> usize {
        let mut gen_size = 0;
        gen_size +=
            Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, source, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(dest) as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_load_fregs_32bit(emu: &EmuEnv, dest: X86TargetRM, source: u64, mc: &mut Vec<u8>) -> usize {
        let mut gen_size = 0;
        gen_size +=
            Self::tcg_modrm_32bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, dest, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(source) as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_store_fregs_32bit(
        emu: &EmuEnv,
        source: X86TargetRM,
        dest: u64,
        mc: &mut Vec<u8>,
    ) -> usize {
        let mut gen_size = 0;

        assert_eq!(source, X86TargetRM::RAX);
        
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, 0xffffffff_00000000, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::OR_GV_EV, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RAX, mc);
        
        gen_size +=
            Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, source, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(dest) as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_rrr_64bit(emu: &EmuEnv, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Register);

        let mut gen_size: usize = 0;

        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg1.value, mc);

        // add    reg_offset(%rbp),%eax
        gen_size += Self::tcg_modrm_64bit_out(op, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg2.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_store_gpr_64bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_rri_64bit(emu: &EmuEnv, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Immediate);

        let mut gen_size: usize = 0;

        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg1.value, mc);

        // add    imm16,%eax
        gen_size += Self::tcg_64bit_out(op, mc);
        gen_size += Self::tcg_out(arg2.value as u64, 4, mc);

        gen_size += Self::tcg_gen_store_gpr_64bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_rrr_32bit(emu: &EmuEnv, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Register);

        let mut gen_size: usize = 0;

        gen_size += Self::tcg_gen_load_gpr_32bit(emu, X86TargetRM::RAX, arg1.value, mc);

        // add    reg_offset(%rbp),%eax
        gen_size += Self::tcg_modrm_32bit_out(op, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg2.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_store_gpr_32bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_rri_32bit(emu: &EmuEnv, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Immediate);

        let mut gen_size: usize = 0;

        gen_size += Self::tcg_gen_load_gpr_32bit(emu, X86TargetRM::RAX, arg1.value, mc);

        // add    imm16,%eax
        gen_size += Self::tcg_out(op as u64, 1, mc);
        gen_size += Self::tcg_out(arg2.value as u64, 4, mc);

        gen_size += Self::tcg_gen_store_gpr_32bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_shift_r_64bit(_emu: &EmuEnv, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let src1_reg = tcg.arg1.unwrap();
        let src2_reg = tcg.arg2.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(src1_reg.t, TCGvType::TCGTemp);
        assert_eq!(src2_reg.t, TCGvType::TCGTemp);

        let mut gen_size: usize = 0;

        let target_x86reg = Self::convert_x86_reg(dest_reg.value);
        // let source1_x86reg = Self::convert_x86_reg(src1_reg.value);
        let source2_x86reg = Self::convert_x86_reg(src1_reg.value);

        gen_size += Self::tcg_modrm_64bit_raw_out(op, X86ModRM::MOD_11_DISP_RAX as u8 + source2_x86reg as u8, target_x86reg as u8, mc);

        return gen_size;
    }

    fn tcg_gen_shift_i_64bit(_emu: &EmuEnv, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let src1_reg = tcg.arg1.unwrap();
        let imm      = tcg.arg2.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(src1_reg.t, TCGvType::TCGTemp);
        assert_eq!(imm     .t, TCGvType::Immediate);

        let mut gen_size: usize = 0;

        let target_x86reg = Self::convert_x86_reg(dest_reg.value);
        let source1_x86reg = Self::convert_x86_reg(src1_reg.value);

        // shift_op   imm,%eax
        if dest_reg.value != src1_reg.value {
            gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX as u8 + source1_x86reg as u8, target_x86reg as u8, mc);    
        }
        gen_size += Self::tcg_modrm_64bit_raw_out(op, X86ModRM::MOD_11_DISP_RAX as u8 + target_x86reg as u8, 0 /*target_x86reg as u8 */, mc);
        gen_size += Self::tcg_out(imm.value as u64, 1, mc);

        return gen_size;
    }

    fn tcg_gen_shift_r_32bit(emu: &EmuEnv, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::TCGTemp);
        assert_eq!(arg1.t, TCGvType::TCGTemp);
        assert_eq!(arg2.t, TCGvType::TCGTemp);

        let mut gen_size: usize = 0;

        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg1.value, mc);

        // add    reg_offset(%rbp),%ecx
        gen_size += Self::tcg_modrm_64bit_out(
            X86Opcode::MOV_GV_EV,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RCX,
            mc,
        );
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg2.value) as u64, 4, mc);

        // shift_op   cl,%eax
        gen_size += Self::tcg_modrm_32bit_out(op, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);

        gen_size += Self::tcg_gen_store_gpr_32bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_shift_i_32bit(emu: &EmuEnv, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Immediate);

        let mut gen_size: usize = 0;

        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg1.value, mc);

        // shift_op   imm,%eax
        gen_size += Self::tcg_modrm_32bit_out(op, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(arg2.value as u64, 1, mc);

        // mov    %eax,reg_offset(%rbp)
        gen_size += Self::tcg_gen_store_gpr_32bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_out(inst: u64, byte_len: usize, v: &mut Vec<u8>) -> usize {
        for (i, be) in inst.to_le_bytes().iter().enumerate() {
            if i < byte_len {
                v.push(*be);
            }
        }
        return byte_len;
    }

    fn tcg_gen_jcc(gen_size: usize, x86_op: X86Opcode, mc: &mut Vec<u8>, label: &Rc<RefCell<tcg::TCGLabel>>) -> usize {
        let mut gen_size = gen_size;

        gen_size += Self::tcg_out(x86_op as u64, 2, mc);
        gen_size += Self::tcg_out(10 as u64, 4, mc);
        gen_size += Self::tcg_out_reloc(gen_size - 4, label);

        return gen_size;
    }

    fn tcg_gen_cmp_branch(emu: &EmuEnv, pc_address: u64, x86_op: X86Opcode, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        let label = match &tcg.label {
            Some(l) => l,
            None => panic!("Label is not defined."),
        };

        let mut gen_size: usize = pc_address as usize;

        // mov    reg_offset(%rbp),%eax
        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg0.value, mc);

        // cmp    reg_offset(%rbp),%eax
        gen_size += Self::tcg_modrm_64bit_out(
            X86Opcode::CMP_GV_EV,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RAX,
            mc,
        );
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg1.value) as u64, 4, mc);

        gen_size = Self::tcg_gen_jcc(gen_size, x86_op, mc, label);
        // je     label

        return gen_size;
    }

    fn tcg_gen_setcc(emu: &EmuEnv, pc_address: u64, x86_op: X86Opcode, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg1.value, mc);
        if arg2.t == TCGvType::Register {
            gen_size += Self::tcg_modrm_64bit_out(
                X86Opcode::CMP_GV_EV,
                X86ModRM::MOD_10_DISP_RBP,
                X86TargetRM::RAX,
                mc,
            );
            gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg2.value) as u64, 4, mc);
        } else {
            gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, arg2.value, mc);
            gen_size += Self::tcg_modrm_64bit_out(
                X86Opcode::CMP_GV_EV,
                X86ModRM::MOD_11_DISP_RCX,
                X86TargetRM::RAX,
                mc,
            );
        }
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, 0, mc); // initialize format RAX
        gen_size += Self::tcg_modrm_2byte_64bit_out(
            x86_op,
            X86ModRM::MOD_11_DISP_RAX,
            X86TargetRM::RAX,
            mc,
        );

        gen_size += Self::tcg_gen_store_gpr_64bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_imm_u64(dest: X86TargetRM, imm: u64, mc: &mut Vec<u8>) -> usize {
        let mut gen_size = 0;
        if dest == X86TargetRM::R8 {
            gen_size += Self::tcg_out(0x49, 1, mc);
        } else {
            gen_size += Self::tcg_out(0x48, 1, mc);
        }
        gen_size += Self::tcg_out(X86Opcode::MOV_EAX_IV as u64 + ((dest as u64) & 0x7), 1, mc);
        gen_size += Self::tcg_out(imm, 8, mc);
        gen_size
    }

    fn convert_x86_reg(temp: u64) -> X86TargetRM {
        return match temp {
            0 => X86TargetRM::RDX,
            1 => X86TargetRM::RBX,
            2 => X86TargetRM::RCX,
            3 => X86TargetRM::RSI,
            4 => X86TargetRM::RDI,
            5 => X86TargetRM::SIB,
            _ => panic!("Not supported yet")
        }
    }

    fn tcg_gen_op_temp(pc_address: u64, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let source1_reg = tcg.arg1.unwrap();
        let source2_reg = tcg.arg2.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(source1_reg.t, TCGvType::TCGTemp);
        assert_eq!(source2_reg.t, TCGvType::TCGTemp);

        let mut gen_size: usize = pc_address as usize;

        assert_eq!(dest_reg.value, source1_reg.value);

        let source1_x86reg = Self::convert_x86_reg(source1_reg.value);
        let source2_x86reg = Self::convert_x86_reg(source2_reg.value);

        gen_size += Self::tcg_modrm_64bit_raw_out(op, X86ModRM::MOD_11_DISP_RAX as u8 + source2_x86reg as u8, source1_x86reg as u8, mc);

        gen_size
    }

    fn tcg_gen_add_temp(_emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        Self::tcg_gen_op_temp(pc_address, X86Opcode::ADD_GV_EV, tcg, mc)
    }

    fn tcg_gen_op_temp_imm(pc_address: u64, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let source1_reg = tcg.arg1.unwrap();
        let source2_imm = tcg.arg2.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(source1_reg.t, TCGvType::TCGTemp);
        assert_eq!(source2_imm.t, TCGvType::Immediate);

        let mut gen_size: usize = pc_address as usize;

        let target_x86reg = Self::convert_x86_reg(dest_reg.value);
        let source1_x86reg = Self::convert_x86_reg(source1_reg.value);

        if dest_reg.value != source1_reg.value {
            gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX as u8 + source1_x86reg as u8, target_x86reg as u8, mc);    
        }

        gen_size += Self::tcg_modrm_64bit_raw_out(op, X86ModRM::MOD_11_DISP_RAX as u8 + target_x86reg as u8, 0, mc);
        gen_size += Self::tcg_out(source2_imm.value, 4, mc);

        gen_size
    }


    fn tcg_gen_op_32_temp(pc_address: u64, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let source1_reg = tcg.arg1.unwrap();
        let source2_reg = tcg.arg2.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(source1_reg.t, TCGvType::TCGTemp);
        assert_eq!(source2_reg.t, TCGvType::TCGTemp);

        let mut gen_size: usize = pc_address as usize;

        let source1_x86reg = Self::convert_x86_reg(source1_reg.value);
        let source2_x86reg = Self::convert_x86_reg(source2_reg.value);

        gen_size += Self::tcg_modrm_32bit_raw_out(op, X86ModRM::MOD_11_DISP_RAX as u8 + source2_x86reg as u8, source1_x86reg as u8, mc);

        gen_size
    }

    fn tcg_gen_op_32_temp_imm(pc_address: u64, op: X86Opcode, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let source1_reg = tcg.arg1.unwrap();
        let source2_imm = tcg.arg2.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(source1_reg.t, TCGvType::TCGTemp);
        assert_eq!(source2_imm.t, TCGvType::Immediate);

        let mut gen_size: usize = pc_address as usize;

        let source1_x86reg = Self::convert_x86_reg(source1_reg.value);

        gen_size += Self::tcg_modrm_32bit_raw_out(op, X86ModRM::MOD_11_DISP_RAX as u8 + source1_x86reg as u8, 0, mc);
        gen_size += Self::tcg_out(source2_imm.value, 4, mc);

        gen_size
    }

}

impl TCG for TCGX86 {
    fn tcg_gen(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        match tcg.op {
            Some(op) => {
                return match op {
                    TCGOpcode::GET_GPR => TCGX86::tcg_gen_get_gpr(emu, pc_address, tcg, mc),
                    TCGOpcode::SET_GPR => TCGX86::tcg_gen_set_gpr(emu, pc_address, tcg, mc),
                    TCGOpcode::ADD_TEMP => TCGX86::tcg_gen_add_temp(emu, pc_address, tcg, mc),

                    TCGOpcode::MOVE_STACK => TCGX86::tcg_gen_move_stack(emu, pc_address, tcg, mc),
                    TCGOpcode::MEM_LOAD => TCGX86::tcg_gen_mem_load(emu, pc_address, tcg, mc),
                    TCGOpcode::MEM_STORE => TCGX86::tcg_gen_mem_store(emu, pc_address, tcg, mc),

                    TCGOpcode::ADD_TLBIDX_OFFSET => TCGX86::tcg_gen_tlbidx_offset(emu, pc_address, tcg, mc),             
                    TCGOpcode::ADD_TLBADDR_OFFSET => TCGX86::tcg_gen_tlbaddr_offset(emu, pc_address, tcg, mc),                    

                    TCGOpcode::ADD_MEM_OFFSET => TCGX86::tcg_gen_mem_offset(emu, pc_address, tcg, mc),

                    TCGOpcode::ADD_64BIT => TCGX86::tcg_gen_add_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SUB_64BIT => TCGX86::tcg_gen_sub_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::AND_64BIT => TCGX86::tcg_gen_and_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::OR_64BIT => TCGX86::tcg_gen_or_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::XOR_64BIT => TCGX86::tcg_gen_xor_64bit(emu, pc_address, tcg, mc),

                    TCGOpcode::MUL_64BIT => TCGX86::tcg_gen_mul_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::MULH_64BIT => TCGX86::tcg_gen_mul_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::MULHU_64BIT => TCGX86::tcg_gen_mul_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::MULHSU_64BIT => TCGX86::tcg_gen_mul_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::MUL_32BIT => TCGX86::tcg_gen_mul_64bit(emu, pc_address, tcg, mc),

                    TCGOpcode::DIV_64BIT  => TCGX86::tcg_gen_div_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::DIVU_64BIT => TCGX86::tcg_gen_div_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::DIV_32BIT  => TCGX86::tcg_gen_div_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::DIVU_32BIT => TCGX86::tcg_gen_div_64bit(emu, pc_address, tcg, mc),

                    TCGOpcode::REM_64BIT  => TCGX86::tcg_gen_rem_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::REMU_64BIT => TCGX86::tcg_gen_rem_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::REM_32BIT  => TCGX86::tcg_gen_rem_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::REMU_32BIT => TCGX86::tcg_gen_rem_64bit(emu, pc_address, tcg, mc),

                    /* Shift operations */
                    TCGOpcode::SRL_64BIT => TCGX86::tcg_gen_srl_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SLL_64BIT => TCGX86::tcg_gen_sll_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SRA_64BIT => TCGX86::tcg_gen_sra_64bit(emu, pc_address, tcg, mc),

                    TCGOpcode::ADD_32BIT => TCGX86::tcg_gen_add_32bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SUB_32BIT => TCGX86::tcg_gen_sub_32bit(emu, pc_address, tcg, mc),

                    TCGOpcode::SRL_32BIT => TCGX86::tcg_gen_srl_32bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SLL_32BIT => TCGX86::tcg_gen_sll_32bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SRA_32BIT => TCGX86::tcg_gen_sra_32bit(emu, pc_address, tcg, mc),

                    TCGOpcode::JMPR => TCGX86::tcg_gen_jmpr(emu, pc_address, tcg, mc),
                    TCGOpcode::JMPIM => TCGX86::tcg_gen_jmpim(emu, pc_address, tcg, mc),
                    TCGOpcode::EQ_64BIT => TCGX86::tcg_gen_eq_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::NE_64BIT => TCGX86::tcg_gen_ne_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::LT_64BIT => TCGX86::tcg_gen_lt_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::GE_64BIT => TCGX86::tcg_gen_ge_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::LTU_64BIT => TCGX86::tcg_gen_ltu_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::GEU_64BIT => TCGX86::tcg_gen_geu_64bit(emu, pc_address, tcg, mc),

                    TCGOpcode::SLT_64BIT => TCGX86::tcg_gen_slt_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SLTU_64BIT => TCGX86::tcg_gen_sltu_64bit(emu, pc_address, tcg, mc),

                    TCGOpcode::EQ_EAX_64BIT => TCGX86::tcg_gen_eq_eax_64bit(emu, pc_address, tcg, mc),

                    TCGOpcode::LOAD_64BIT => TCGX86::tcg_gen_load( emu, pc_address, tcg, mc, MemOpType::LOAD_64BIT, RegisterType::IntRegister),
                    TCGOpcode::LOAD_32BIT => TCGX86::tcg_gen_load( emu, pc_address, tcg, mc, MemOpType::LOAD_32BIT, RegisterType::IntRegister),
                    TCGOpcode::LOAD_16BIT => TCGX86::tcg_gen_load( emu, pc_address, tcg, mc, MemOpType::LOAD_16BIT, RegisterType::IntRegister),
                    TCGOpcode::LOAD_8BIT => TCGX86::tcg_gen_load( emu, pc_address, tcg, mc, MemOpType::LOAD_8BIT, RegisterType::IntRegister),
                    TCGOpcode::LOADU_32BIT => TCGX86::tcg_gen_load( emu, pc_address, tcg, mc, MemOpType::LOAD_U_32BIT, RegisterType::IntRegister),
                    TCGOpcode::LOADU_16BIT => TCGX86::tcg_gen_load( emu, pc_address, tcg, mc, MemOpType::LOAD_U_16BIT, RegisterType::IntRegister),
                    TCGOpcode::LOADU_8BIT => TCGX86::tcg_gen_load( emu, pc_address, tcg, mc, MemOpType::LOAD_U_8BIT, RegisterType::IntRegister),
                    TCGOpcode::STORE_64BIT => TCGX86::tcg_gen_store( emu, pc_address, tcg, mc, MemOpType::STORE_64BIT, RegisterType::IntRegister),
                    TCGOpcode::STORE_32BIT => TCGX86::tcg_gen_store( emu, pc_address, tcg, mc, MemOpType::STORE_32BIT, RegisterType::IntRegister),
                    TCGOpcode::STORE_16BIT => TCGX86::tcg_gen_store( emu, pc_address, tcg, mc, MemOpType::STORE_16BIT, RegisterType::IntRegister),
                    TCGOpcode::STORE_8BIT => TCGX86::tcg_gen_store( emu, pc_address, tcg, mc, MemOpType::STORE_8BIT, RegisterType::IntRegister),

                    TCGOpcode::LOAD_FLOAT_64BIT => TCGX86::tcg_gen_load( emu, pc_address, tcg, mc, MemOpType::LOAD_64BIT, RegisterType::FloatRegister),
                    TCGOpcode::LOAD_FLOAT_32BIT => TCGX86::tcg_gen_load( emu, pc_address, tcg, mc, MemOpType::LOAD_32BIT, RegisterType::FloatRegister),

                    TCGOpcode::STORE_FLOAT_64BIT => TCGX86::tcg_gen_store( emu, pc_address, tcg, mc, MemOpType::STORE_64BIT, RegisterType::FloatRegister),
                    TCGOpcode::STORE_FLOAT_32BIT => TCGX86::tcg_gen_store( emu, pc_address, tcg, mc, MemOpType::STORE_32BIT, RegisterType::FloatRegister),

                    TCGOpcode::MOVE_TO_INT_FROM_FLOAT => { TCGX86::tcg_gen_int_reg_from_float_reg(emu, pc_address, tcg, mc) }
                    TCGOpcode::MOVE_TO_FLOAT_FROM_INT => { TCGX86::tcg_gen_float_reg_from_int_reg(emu, pc_address, tcg, mc) }
                    TCGOpcode::MOVE_TO_INT_FROM_FLOAT_32BIT => { TCGX86::tcg_gen_int_reg_from_float_reg_32bit(emu, pc_address, tcg, mc) }
                    TCGOpcode::MOVE_TO_FLOAT_FROM_INT_32BIT => { TCGX86::tcg_gen_float_reg_from_int_reg_32bit(emu, pc_address, tcg, mc) }

                    TCGOpcode::MOV_IMM_64BIT => TCGX86::tcg_gen_mov_imm_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::MOV_64BIT => TCGX86::tcg_gen_mov_64bit(emu, pc_address, tcg, mc),

                    TCGOpcode::SIGN_EXT_32_64 => TCGX86::tcg_gen_sign_ext_32_64(emu, pc_address, tcg, mc),

                    TCGOpcode::SGNJ_64BIT => TCGX86::tcg_gen_sgnj_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SGNJN_64BIT => TCGX86::tcg_gen_sgnjn_64bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SGNJX_64BIT => TCGX86::tcg_gen_sgnjx_64bit(emu, pc_address, tcg, mc),

                    TCGOpcode::SGNJ_32BIT => TCGX86::tcg_gen_sgnj_32bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SGNJN_32BIT => TCGX86::tcg_gen_sgnjn_32bit(emu, pc_address, tcg, mc),
                    TCGOpcode::SGNJX_32BIT => TCGX86::tcg_gen_sgnjx_32bit(emu, pc_address, tcg, mc),

                    TCGOpcode::HELPER_CALL_ARG0 => { TCGX86::tcg_gen_helper_call(emu, 0, pc_address, tcg, mc) }
                    TCGOpcode::HELPER_CALL_ARG1 => { TCGX86::tcg_gen_helper_call(emu, 1, pc_address, tcg, mc) }
                    TCGOpcode::HELPER_CALL_ARG2 => { TCGX86::tcg_gen_helper_call(emu, 2, pc_address, tcg, mc) }
                    TCGOpcode::HELPER_CALL_ARG3 => { TCGX86::tcg_gen_helper_call(emu, 3, pc_address, tcg, mc) }
                    TCGOpcode::HELPER_CALL_ARG4 => { TCGX86::tcg_gen_helper_call(emu, 4, pc_address, tcg, mc) }
                    TCGOpcode::CMP_EQ => { TCGX86::tcg_gen_cmp_eq(emu, pc_address, tcg, mc) }
                    TCGOpcode::TLB_MATCH_CHECK => { TCGX86::tcg_gen_match_check(emu, pc_address, tcg, mc) }
                    
                    TCGOpcode::EXIT_TB => TCGX86::tcg_exit_tb(emu, pc_address, tcg, mc),
                };
            }
            None => match &tcg.label {
                Some(_l) => TCGX86::tcg_gen_label(pc_address, tcg),
                None => panic!("Illegal Condition"),
            },
        }
    }

    fn tcg_gen_get_gpr(emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let src_reg = tcg.arg1.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(src_reg.t, TCGvType::Register);

        let target_x86reg = Self::convert_x86_reg(dest_reg.value);

        let mut gen_size = pc_address as usize;
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, target_x86reg, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(src_reg.value) as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_set_gpr(emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let src_reg = tcg.arg1.unwrap();

        assert_eq!(dest_reg.t, TCGvType::Register);
        assert_eq!(src_reg.t, TCGvType::TCGTemp);

        let source_x86reg = Self::convert_x86_reg(src_reg.value);

        let mut gen_size = pc_address as usize;
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_10_DISP_RBP, source_x86reg, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(dest_reg.value) as u64, 4, mc);
        return gen_size;
    }


    fn tcg_gen_move_stack(_emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);

        let target_x86reg = Self::convert_x86_reg(dest_reg.value);

        let mut gen_size = pc_address as usize;
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RBP, target_x86reg, mc);
        return gen_size;
    }


    fn tcg_gen_mem_load(_emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let src_reg = tcg.arg1.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(src_reg.t, TCGvType::TCGTemp);

        let target_x86reg = Self::convert_x86_reg(dest_reg.value);
        let source_x86reg = Self::convert_x86_reg(src_reg.value);

        let mut gen_size = pc_address as usize;
        gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_00_DISP_RAX as u8 + source_x86reg as u8, target_x86reg as u8, mc);
        return gen_size;
    }

    fn tcg_gen_mem_store(_emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let src_reg = tcg.arg1.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(src_reg.t, TCGvType::TCGTemp);

        let target_x86reg = Self::convert_x86_reg(dest_reg.value);
        let source_x86reg = Self::convert_x86_reg(src_reg.value);

        let mut gen_size = pc_address as usize;
        gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_00_DISP_RAX as u8 + source_x86reg as u8, target_x86reg as u8, mc);
        return gen_size;
    }

    fn tcg_gen_tlbidx_offset(emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let src_reg = tcg.arg1.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(src_reg.t, TCGvType::TCGTemp);

        let target_x86reg = Self::convert_x86_reg(dest_reg.value);
        let source_x86reg = Self::convert_x86_reg(src_reg.value);

        let mut gen_size = pc_address as usize;

        if dest_reg.value != src_reg.value {
            gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX as u8 + source_x86reg as u8, target_x86reg as u8, mc);    
        }
        gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::ADD_GV_IMM, X86ModRM::MOD_11_DISP_RAX as u8 + target_x86reg as u8, 0, mc);
        gen_size += Self::tcg_out(emu.calc_tlb_relat_address() as u64, 4, mc);
        return gen_size;
    }


    fn tcg_gen_tlbaddr_offset(emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let src_reg = tcg.arg1.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(src_reg.t, TCGvType::TCGTemp);

        let target_x86reg = Self::convert_x86_reg(dest_reg.value);
        let source_x86reg = Self::convert_x86_reg(src_reg.value);

        let mut gen_size = pc_address as usize;

        if dest_reg.value != src_reg.value {
            gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX as u8 + source_x86reg as u8, target_x86reg as u8, mc);    
        }
        gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::ADD_GV_IMM, X86ModRM::MOD_11_DISP_RAX as u8 + target_x86reg as u8, 0, mc);
        gen_size += Self::tcg_out(emu.calc_tlb_addr_relat_address() as u64, 4, mc);
        return gen_size;
    }

    fn tcg_gen_mem_offset(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let offset = tcg.arg1.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(offset.t, TCGvType::TCGTemp);

        let target_x86reg = Self::convert_x86_reg(dest_reg.value);
        let offset_x86reg = Self::convert_x86_reg(offset.value);
        let mut gen_size = pc_address as usize;

        let guestcode_addr = emu.calc_guest_data_mem_address();
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, guestcode_addr as u64, mc);

        gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::ADD_GV_EV, X86ModRM::MOD_11_DISP_RAX as u8, target_x86reg as u8, mc);
        if dest_reg.value != offset.value {
            gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::ADD_GV_EV, X86ModRM::MOD_11_DISP_RAX as u8 + offset_x86reg as u8, target_x86reg as u8, mc);
        }
        return gen_size;
    }


    fn tcg_gen_add_64bit(_emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        if tcg.arg2.unwrap().t == TCGvType::Immediate {
            Self::tcg_gen_op_temp_imm(pc_address, X86Opcode::ADD_GV_IMM, tcg, mc)
        } else {
            Self::tcg_gen_op_temp(pc_address, X86Opcode::ADD_GV_EV, tcg, mc)
        }
    }

    fn tcg_gen_add_32bit(_emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        if tcg.arg2.unwrap().t == TCGvType::Immediate {
            Self::tcg_gen_op_32_temp_imm(pc_address, X86Opcode::ADD_GV_IMM, tcg, mc)
        } else {
            Self::tcg_gen_op_32_temp(pc_address, X86Opcode::ADD_GV_EV, tcg, mc)
        }
    }

    fn tcg_gen_sub_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert!(arg1.t == TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        if arg2.t == tcg::TCGvType::Immediate {
            // xxxx: should be sub
            gen_size += Self::tcg_gen_rri_32bit(emu, X86Opcode::ADD_EAX_IV, tcg, mc);
            return gen_size;
        } else {
            gen_size += Self::tcg_gen_rrr_32bit(emu, X86Opcode::SUB_GV_EV, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_sub_64bit(_emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        if tcg.arg2.unwrap().t == TCGvType::Immediate {
            Self::tcg_gen_op_temp_imm(pc_address, X86Opcode::SUB_GV_IMM, tcg, mc)
        } else {
            Self::tcg_gen_op_temp(pc_address, X86Opcode::SUB_GV_EV, tcg, mc)
        }
    }

    fn tcg_gen_and_64bit(_emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        if tcg.arg2.unwrap().t == TCGvType::Immediate {
            Self::tcg_gen_op_temp_imm(pc_address, X86Opcode::AND_GV_IMM, tcg, mc)
        } else {
            Self::tcg_gen_op_temp(pc_address, X86Opcode::AND_GV_EV, tcg, mc)
        }
    }

    fn tcg_gen_or_64bit(_emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        if tcg.arg2.unwrap().t == TCGvType::Immediate {
            Self::tcg_gen_op_temp_imm(pc_address, X86Opcode::OR_GV_IMM, tcg, mc)
        } else {
            Self::tcg_gen_op_temp(pc_address, X86Opcode::OR_GV_EV, tcg, mc)
        }
    }

    fn tcg_gen_xor_64bit(_emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        if tcg.arg2.unwrap().t == TCGvType::Immediate {
            Self::tcg_gen_op_temp_imm(pc_address, X86Opcode::XOR_GV_IMM, tcg, mc)
        } else {
            Self::tcg_gen_op_temp(pc_address, X86Opcode::XOR_GV_EV, tcg, mc)
        }
    }

    fn tcg_gen_mul_64bit(emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;
        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg1.value, mc);

        gen_size += Self::tcg_modrm_2byte_64bit_out(X86Opcode::IMUL_RDX_RAX_R, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg2.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_store_gpr_64bit(emu, X86TargetRM::RAX, arg0.value, mc);


        gen_size
    }

    fn tcg_gen_div_64bit(emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;
        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg1.value, mc);

        gen_size += Self::tcg_64bit_out(X86Opcode::CQO, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::IDIV_RDX_RAX_R, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RDI, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg2.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_store_gpr_64bit(emu, X86TargetRM::RAX, arg0.value, mc);


        gen_size
    }

    fn tcg_gen_rem_64bit(emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;
        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg1.value, mc);

        gen_size += Self::tcg_modrm_2byte_64bit_out(X86Opcode::IDIV_RDX_RAX_R, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg2.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_store_gpr_64bit(emu, X86TargetRM::RAX, arg0.value, mc);


        gen_size
    }

    fn tcg_gen_sign_ext_32_64(_emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let dest_reg = tcg.arg0.unwrap();
        let source1_reg = tcg.arg1.unwrap();

        assert_eq!(dest_reg.t, TCGvType::TCGTemp);
        assert_eq!(source1_reg.t, TCGvType::TCGTemp);

        let mut gen_size: usize = pc_address as usize;

        let dest_x86reg = Self::convert_x86_reg(dest_reg.value);
        let source1_x86reg = Self::convert_x86_reg(source1_reg.value);

        gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::MOV_GV_EV_32BIT, X86ModRM::MOD_11_DISP_RAX as u8 + source1_x86reg as u8, 
                dest_x86reg as u8, mc);

        gen_size
    }

    fn tcg_gen_srl_64bit(emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::TCGTemp);
        assert_eq!(arg1.t, TCGvType::TCGTemp);

        let mut gen_size: usize = pc_address as usize;

        if arg2.t == tcg::TCGvType::Immediate {
            gen_size += Self::tcg_gen_shift_i_64bit(emu, X86Opcode::SRL_GV_IMM, tcg, mc);
            return gen_size;
        } else {
            gen_size += Self::tcg_gen_shift_r_64bit(emu, X86Opcode::SRL_GV_CL, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_sll_64bit(emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::TCGTemp);
        assert_eq!(arg1.t, TCGvType::TCGTemp);

        let mut gen_size: usize = pc_address as usize;

        if arg2.t == tcg::TCGvType::Immediate {
            gen_size += Self::tcg_gen_shift_i_64bit(emu, X86Opcode::SLL_GV_IMM, tcg, mc);
            return gen_size;
        } else {
            gen_size += Self::tcg_gen_shift_r_64bit(emu, X86Opcode::SLL_GV_CL, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_sra_64bit(emu: &EmuEnv, pc_address: u64, tcg: &tcg::TCGOp, mc: &mut Vec<u8>) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::TCGTemp);
        assert_eq!(arg1.t, TCGvType::TCGTemp);

        let mut gen_size: usize = pc_address as usize;

        if arg2.t == tcg::TCGvType::Immediate {
            gen_size += Self::tcg_gen_shift_i_64bit(emu, X86Opcode::SRA_GV_IMM, tcg, mc);
            return gen_size;
        } else {
            gen_size += Self::tcg_gen_shift_r_64bit(emu, X86Opcode::SRA_GV_CL, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_jmpr(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Immediate);
        assert_eq!(op, TCGOpcode::JMPR);

        let mut gen_size: usize = pc_address as usize;

        // GPR --> RAX
        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg1.value, mc);
        // RAX + arg2.value --> RAX
        gen_size += Self::tcg_64bit_out(X86Opcode::ADD_EAX_IV, mc);
        gen_size += Self::tcg_out(arg2.value as u64, 4, mc);
        // RAX --> PC
        gen_size += Self::tcg_modrm_64bit_out(
            X86Opcode::MOV_EV_GV,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RAX,
            mc,
        );
        gen_size += Self::tcg_out(emu.calc_pc_address() as u64, 4, mc); // Set Program Counter

        return gen_size;
        // }
    }

    fn tcg_gen_jmpim(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let imm = tcg.arg1.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(imm.t, TCGvType::Immediate);
        assert_eq!(op, TCGOpcode::JMPIM);

        let mut gen_size: usize = pc_address as usize;

        // Immediate Value --> RAX
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, imm.value as u64, mc);
        // RAX --> PC
        gen_size += Self::tcg_modrm_64bit_out(
            X86Opcode::MOV_EV_GV,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RAX,
            mc,
        );
        gen_size += Self::tcg_out(emu.calc_pc_address() as u64, 4, mc); // Set Program Counter

        return gen_size;
    }

    fn tcg_gen_eq_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        return Self::tcg_gen_cmp_branch(emu, pc_address, X86Opcode::JE_rel16_32, tcg, mc);
    }

    fn tcg_gen_ne_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        return Self::tcg_gen_cmp_branch(emu, pc_address, X86Opcode::JNE_rel16_32, tcg, mc);
    }

    fn tcg_gen_lt_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        return Self::tcg_gen_cmp_branch(emu, pc_address, X86Opcode::JL_rel16_32, tcg, mc);
    }

    fn tcg_gen_ge_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        return Self::tcg_gen_cmp_branch(emu, pc_address, X86Opcode::JGE_rel16_32, tcg, mc);
    }

    fn tcg_gen_ltu_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        return Self::tcg_gen_cmp_branch(emu, pc_address, X86Opcode::JB_rel16_32, tcg, mc);
    }

    fn tcg_gen_geu_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        return Self::tcg_gen_cmp_branch(emu, pc_address, X86Opcode::JAE_rel16_32, tcg, mc);
    }

    fn tcg_gen_slt_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        return Self::tcg_gen_setcc(emu, pc_address, X86Opcode::SETL, tcg, mc);
    }

    fn tcg_gen_sltu_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        return Self::tcg_gen_setcc(emu, pc_address, X86Opcode::SETB, tcg, mc);
    }

    fn tcg_gen_eq_eax_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        // let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        let label = match &tcg.label {
            Some(l) => l,
            None => panic!("Label is not defined."),
        };

        let mut gen_size: usize = pc_address as usize;

        // mov    reg_offset(%rbp),%eax
        // gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg0.value, mc);

        // cmp    reg_offset(%rbp),%eax
        gen_size += Self::tcg_modrm_64bit_out(
            X86Opcode::CMP_GV_EV,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RAX,
            mc,
        );
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg1.value) as u64, 4, mc);

        gen_size = Self::tcg_gen_jcc(gen_size, X86Opcode::JE_rel16_32, mc, label);
        // je     label

        return gen_size;
    }

    fn tcg_exit_tb(emu: &EmuEnv, pc_address: u64, _tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let mut gen_size: usize = pc_address as usize;
        // jmp    epilogue
        gen_size += Self::tcg_out(X86Opcode::JMP_JZ as u64, 1, mc);
        let diff_from_epilogue = emu.calc_epilogue_address();
        gen_size += Self::tcg_out((diff_from_epilogue - gen_size as isize - 4) as u64, 4, mc);

        return gen_size;
    }

    fn tcg_gen_mov_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        assert_eq!(op, TCGOpcode::MOV_64BIT);
        assert_eq!(arg0.t, TCGvType::ProgramCounter);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, arg1.value, mc);

        gen_size += Self::tcg_modrm_64bit_out(
            X86Opcode::MOV_EV_GV,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RAX,
            mc,
        );
        gen_size += Self::tcg_out(emu.calc_pc_address() as u64, 4, mc); // Set Program Counter

        // jmp    epilogue
        gen_size += Self::tcg_exit_tb(emu, gen_size as u64, tcg, mc);
        return gen_size;
    }

    fn tcg_gen_mov_imm_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        assert_eq!(op, TCGOpcode::MOV_IMM_64BIT);
        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Immediate);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, arg1.value, mc);

        gen_size += Self::tcg_modrm_64bit_out(
            X86Opcode::MOV_EV_GV,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RAX,
            mc,
        );
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg0.value) as u64, 4, mc); // Set Program Counter

        return gen_size;
    }


    fn tcg_out_reloc(host_code_ptr: usize, label: &Rc<RefCell<TCGLabel>>) -> usize {
        // let mut l = &mut *label.borrow_mut();
        let l2 = &mut *label.borrow_mut();
        l2.code_ptr_vec.push(host_code_ptr);
        // println!("Added offset. code_ptr = {:x}", host_code_ptr);
        return 0;
    }

    fn tcg_gen_label(pc_address: u64, tcg: &TCGOp) -> usize {
        match &tcg.label {
            Some(label) => {
                let mut l = &mut *label.borrow_mut();
                l.offset = pc_address;
                // println!("Offset is set {:x}", l.offset);
            }
            None => panic!("Unknown behavior"),
        }
        return 0;
    }

    /* Memory Access : Load */
    fn tcg_gen_load(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>, mem_size: MemOpType, target_reg: RegisterType) -> usize {
        let mut gen_size: usize = pc_address as usize;

        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Immediate);

        // Load Guest Memory Head into EAX
        let guestcode_addr = emu.calc_guest_data_mem_address();
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, guestcode_addr as u64, mc);
        
        // Move Guest Memory from EAX to ECX
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RCX, mc);
        
        // Load value from rs1
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(arg1.value) as u64, 4, mc);
        
        // rs1 + imm
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_IMM /* ADD_EV_IV */, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(arg2.value as u64, 4, mc);

        // Move RDX --> RSI (Base Address)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RSI, mc);

        // Extract lower 12-bit to search TLB table offset (RSI)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_IMM, X86ModRM::MOD_11_DISP_RSI, X86TargetRM::SIB, mc);
        gen_size += Self::tcg_out(0x0fff, 4, mc);        

        // Shift right 12-bit to search TLB table (RCX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::SRL_GV_IMM, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(12, 1, mc);

        // Extract lower 12-bit to search TLB table offset (RAX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_IMM, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(0x0fff, 4, mc);

        // Right shift 3 bit to align 64-bit entry size (RAX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::SLL_GV_IMM, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(3, 1, mc);

        // Move RAX --> RDX (Extracted Address Offset)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RDX, mc);

        // Move RBP --> RAX (Base Address)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RBP, X86TargetRM::RAX, mc);
        
        // Load TLB Vector Address Base Address
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_IMM /* ADD_EV_IV */, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_tlb_addr_relat_address() as u64, 4, mc);

        // Add TLB base address offset (RAX)
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_EV, X86ModRM::MOD_11_DISP_RDX, X86TargetRM::RAX, mc);

        // Load Physical Address from TLB address table
        Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RAX, mc);

        // Physical Address + 12bit offset in RSI
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_EV, X86ModRM::MOD_11_DISP_RSI, X86TargetRM::RAX, mc);

        // Physical Address + Memory Head Address
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_EV, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RAX, mc);

        // Address Calculation : Sub Bias 0x8000_0000
        gen_size += Self::tcg_64bit_out(X86Opcode::ADD_EAX_IV, mc);
        gen_size += Self::tcg_out(0x8000_0000 as u64, 4, mc);

        gen_size += match mem_size {
            MemOpType::LOAD_64BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RAX, mc);
                gen_size
            }
            MemOpType::LOAD_32BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV_32BIT, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RAX, mc);
                gen_size
            }
            MemOpType::LOAD_16BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_2byte_64bit_out(X86Opcode::MOV_GV_EV_S_16BIT, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RAX, mc);
                gen_size
            }
            MemOpType::LOAD_8BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_2byte_64bit_out(X86Opcode::MOV_GV_EV_S_8BIT, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RAX, mc);
                gen_size
            }
            MemOpType::LOAD_U_32BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_32bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RAX, mc);
                gen_size
            }
            MemOpType::LOAD_U_16BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_2byte_32bit_out(X86Opcode::MOV_GV_EV_U_16BIT, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RAX, mc);
                gen_size
            }
            MemOpType::LOAD_U_8BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_2byte_32bit_out(X86Opcode::MOV_GV_EV_U_8BIT, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RAX, mc);
                gen_size
            }
            _ => panic!("Not supported load instruction."),
        };

        // Store Loaded value into destination register.
        if target_reg == RegisterType::IntRegister {
            gen_size += Self::tcg_gen_store_gpr_64bit(emu, X86TargetRM::RAX, arg0.value, mc);
        } else if target_reg == RegisterType::FloatRegister && mem_size == MemOpType::LOAD_64BIT {
            gen_size += Self::tcg_gen_store_fregs_64bit(emu, X86TargetRM::RAX, arg0.value, mc);
        } else if target_reg == RegisterType::FloatRegister && mem_size == MemOpType::LOAD_32BIT {
            gen_size += Self::tcg_gen_store_fregs_32bit(emu, X86TargetRM::RAX, arg0.value, mc);
        } else {
            panic!("Unknown condition for Register Write")
        }
        return gen_size;
    }

    /* Memory Access : Store */
    fn tcg_gen_store(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>, mem_size: MemOpType, target_reg: RegisterType) -> usize {
        let mut gen_size: usize = pc_address as usize;

        let rs1 = tcg.arg0.unwrap();
        let rs2 = tcg.arg1.unwrap();
        let imm = tcg.arg2.unwrap();

        assert_eq!(rs1.t, TCGvType::Register);
        assert_eq!(rs2.t, TCGvType::Register);
        assert_eq!(imm.t, TCGvType::Immediate);

        // Load Guest Memory Head into EAX
        let guestcode_addr = emu.calc_guest_data_mem_address();
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, guestcode_addr as u64, mc);
        
        // Move Guest Memory from EAX to ECX
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RCX, mc);
        
        // Load value from rs1
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(rs1.value) as u64, 4, mc);
        
        // rs1 + imm
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_IMM /* ADD_EV_IV */, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(imm.value as u64, 4, mc);
    
        // Move RDX --> RSI (Base Address)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RSI, mc);
    
        // Extract lower 12-bit to search TLB table offset (RSI)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_IMM, X86ModRM::MOD_11_DISP_RSI, X86TargetRM::SIB, mc);
        gen_size += Self::tcg_out(0x0fff, 4, mc);        
    
        // Shift right 12-bit to search TLB table (RCX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::SRL_GV_IMM, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(12, 1, mc);
    
        // Extract lower 12-bit to search TLB table offset (RAX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_IMM, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(0x0fff, 4, mc);
    
        // Right shift 3 bit to align 64-bit entry size (RAX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::SLL_GV_IMM, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(3, 1, mc);
    
        // Move RAX --> RDX (Extracted Address Offset)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RDX, mc);
    
        // Move RBP --> RAX (Base Address)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RBP, X86TargetRM::RAX, mc);
        
        // Load TLB Vector Address Base Address
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_IMM /* ADD_EV_IV */, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_tlb_addr_relat_address() as u64, 4, mc);
    
        // Add TLB base address offset (RAX)
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_EV, X86ModRM::MOD_11_DISP_RDX, X86TargetRM::RAX, mc);
    
        // Load Physical Address from TLB address table
        Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RAX, mc);
    
        // Physical Address + 12bit offset in RSI
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_EV, X86ModRM::MOD_11_DISP_RSI, X86TargetRM::RAX, mc);
    
        // Physical Address + Memory Head Address
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_EV, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RAX, mc);
    
        // Address Calculation : Sub Bias 0x8000_0000
        gen_size += Self::tcg_64bit_out(X86Opcode::ADD_EAX_IV, mc);
        gen_size += Self::tcg_out(0x8000_0000 as u64, 4, mc);        
        
        // Load value from rs2 (data)
        if target_reg == RegisterType::IntRegister {
            gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RCX, rs2.value, mc);
        } else if target_reg == RegisterType::FloatRegister && mem_size == MemOpType::STORE_64BIT {
            gen_size += Self::tcg_gen_load_fregs_64bit(emu, X86TargetRM::RCX, rs2.value, mc);
        } else if target_reg == RegisterType::FloatRegister && mem_size == MemOpType::STORE_32BIT {
            gen_size += Self::tcg_gen_load_fregs_32bit(emu, X86TargetRM::RCX, rs2.value, mc);
        } else {
            panic!("Unknown Register read condition.")
        }
        gen_size += match mem_size {
            MemOpType::STORE_64BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RCX, mc);
                gen_size
            }
            MemOpType::STORE_32BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_32bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RCX, mc);
                gen_size
            }
            MemOpType::STORE_16BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_16bit_out(X86Opcode::MOV_EV_GV, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RCX, mc);
                gen_size
            }
            MemOpType::STORE_8BIT => {
                let mut gen_size = 0;
                gen_size += Self::tcg_modrm_32bit_out(X86Opcode::MOV_EB_GB, X86ModRM::MOD_00_DISP_RAX, X86TargetRM::RCX, mc);
                gen_size
            }
            _ => panic!("Unsupported memory size!"),
        };

        return gen_size;
    }

    fn tcg_gen_csrrw(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let mut gen_size: usize = pc_address as usize;

        let rd = tcg.arg0.unwrap();
        let rs1 = tcg.arg1.unwrap();
        let csr_addr = tcg.arg2.unwrap();

        assert_eq!(rd.t, TCGvType::Register);
        assert!(rs1.t == TCGvType::Register || rs1.t == TCGvType::Immediate);
        assert_eq!(csr_addr.t, TCGvType::Immediate);

        // Argument 0 : Env
        let self_ptr = emu.head.as_ptr() as *const u8;
        let self_diff = unsafe { self_ptr.offset(0) };
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RDI, self_diff as u64, mc);

        // Argument 1 : rd u32
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RSI, rd.value as u64, mc);

        // Argument 2 : rs1 u32
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RDX, rs1.value as u64, mc);

        // Argument 3 : csr_addr u32
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, csr_addr.value as u64, mc);

        gen_size += Self::tcg_modrm_32bit_out(
            X86Opcode::CALL,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RDX,
            mc,
        );
        let mut csr_helper_idx = CALL_HELPER_IDX::CALL_CSRRW_IDX;
        if rs1.t == TCGvType::Immediate {
            csr_helper_idx = CALL_HELPER_IDX::CALL_CSRRWI_IDX;
        }
        let helper_func_addr = emu.calc_helper_func_relat_address(csr_helper_idx as usize);
        gen_size += Self::tcg_out(helper_func_addr as u64, 4, mc);

        gen_size
    }

    fn tcg_gen_csrrs(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let mut gen_size: usize = pc_address as usize;

        let rd = tcg.arg0.unwrap();
        let rs1 = tcg.arg1.unwrap();
        let rs2 = tcg.arg2.unwrap();

        assert_eq!(rd.t, TCGvType::Register);
        assert!(rs1.t == TCGvType::Register || rs1.t == TCGvType::Immediate);
        assert_eq!(rs2.t, TCGvType::Immediate);

        // Argument 0 : Env
        let self_ptr = emu.head.as_ptr() as *const u8;
        let self_diff = unsafe { self_ptr.offset(0) };
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RDI, self_diff as u64, mc);

        // Argument 1 : rd u32
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RSI, rd.value as u64, mc);

        // Argument 2 : rs1 u32
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RDX, rs1.value as u64, mc);

        // Argument 3 : rs2 u32
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, rs2.value as u64, mc);

        gen_size += Self::tcg_modrm_32bit_out(
            X86Opcode::CALL,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RDX,
            mc,
        );

        let mut csr_helper_idx = CALL_HELPER_IDX::CALL_CSRRS_IDX;
        if rs1.t == TCGvType::Immediate {
            csr_helper_idx = CALL_HELPER_IDX::CALL_CSRRSI_IDX;
        }
        let helper_func_addr = emu.calc_helper_func_relat_address(csr_helper_idx as usize);
        gen_size += Self::tcg_out(helper_func_addr as u64, 4, mc);

        gen_size
    }

    fn tcg_gen_csrrc(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let mut gen_size: usize = pc_address as usize;

        let rd = tcg.arg0.unwrap();
        let rs1 = tcg.arg1.unwrap();
        let csr_addr = tcg.arg2.unwrap();

        assert_eq!(rd.t, TCGvType::Register);
        assert!(rs1.t == TCGvType::Register || rs1.t == TCGvType::Immediate);
        assert_eq!(csr_addr.t, TCGvType::Immediate);

        // Argument 0 : Env
        let self_ptr = emu.head.as_ptr() as *const u8;
        let self_diff = unsafe { self_ptr.offset(0) };
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RDI, self_diff as u64, mc);

        // Argument 1 : rd u32
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RSI, rd.value as u64, mc);

        // Argument 2 : rs1 u32
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RDX, rs1.value as u64, mc);

        // Argument 3 : csr_addr u32
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, csr_addr.value as u64, mc);

        gen_size += Self::tcg_modrm_32bit_out(
            X86Opcode::CALL,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RDX,
            mc,
        );
        let mut csr_helper_idx = CALL_HELPER_IDX::CALL_CSRRC_IDX;
        if rs1.t == TCGvType::Immediate {
            csr_helper_idx = CALL_HELPER_IDX::CALL_CSRRCI_IDX;
        }
        let helper_func_addr = emu.calc_helper_func_relat_address(csr_helper_idx as usize);
        gen_size += Self::tcg_out(helper_func_addr as u64, 4, mc);

        gen_size
    }

    fn tcg_gen_helper_call(
        emu: &EmuEnv,
        arg_size: usize,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let mut gen_size: usize = pc_address as usize;

        let self_ptr = emu.head.as_ptr() as *const u8;
        let self_diff = unsafe { self_ptr.offset(0) };
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RDI, self_diff as u64, mc);

        if arg_size >= 1 {
            let arg0 = tcg.arg0.unwrap();
            gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RSI, arg0.value as u64, mc);
        }
        if arg_size >= 2 {
            let arg1 = tcg.arg1.unwrap();
            gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RDX, arg1.value as u64, mc);
        }
        if arg_size >= 3 {
            let arg2 = tcg.arg2.unwrap();
            gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, arg2.value as u64, mc);
        }

        if arg_size >= 4 {
            let arg3 = tcg.arg3.unwrap();
            gen_size += Self::tcg_gen_imm_u64(X86TargetRM::R8, arg3.value as u64, mc);
        }

        gen_size += Self::tcg_modrm_32bit_out(
            X86Opcode::CALL,
            X86ModRM::MOD_10_DISP_RBP,
            X86TargetRM::RDX,
            mc,
        );
        let csr_helper_idx = tcg.helper_idx;
        let helper_func_addr = emu.calc_helper_func_relat_address(csr_helper_idx as usize);
        gen_size += Self::tcg_out(helper_func_addr as u64, 4, mc);

        return gen_size;
    }

    fn tcg_gen_srl_32bit(
        emu: &EmuEnv,
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

        if arg2.t == tcg::TCGvType::Immediate {
            gen_size += Self::tcg_gen_shift_i_32bit(emu, X86Opcode::SRL_GV_IMM, tcg, mc);
            return gen_size;
        } else {
            gen_size += Self::tcg_gen_shift_r_32bit(emu, X86Opcode::SRL_GV_CL, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_sll_32bit(
        emu: &EmuEnv,
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

        if arg2.t == tcg::TCGvType::Immediate {
            gen_size += Self::tcg_gen_shift_i_32bit(emu, X86Opcode::SLL_GV_IMM, tcg, mc);
            return gen_size;
        } else {
            gen_size += Self::tcg_gen_shift_r_32bit(emu, X86Opcode::SLL_GV_CL, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_sra_32bit(
        emu: &EmuEnv,
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

        if arg2.t == tcg::TCGvType::Immediate {
            gen_size += Self::tcg_gen_shift_i_32bit(emu, X86Opcode::SRA_GV_IMM, tcg, mc);
            return gen_size;
        } else {
            gen_size += Self::tcg_gen_shift_r_32bit(emu, X86Opcode::SRA_GV_CL, tcg, mc);
            return gen_size;
        }
    }

    fn tcg_gen_int_reg_from_float_reg(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_load_fregs_64bit(emu, X86TargetRM::RAX, arg1.value, mc);
        gen_size += Self::tcg_gen_store_gpr_64bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_float_reg_from_int_reg(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_load_gpr_64bit(emu, X86TargetRM::RAX, arg1.value, mc);
        gen_size += Self::tcg_gen_store_fregs_64bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_int_reg_from_float_reg_32bit(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_load_fregs_32bit(emu, X86TargetRM::RAX, arg1.value, mc);
        gen_size += Self::tcg_gen_store_gpr_32bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_float_reg_from_int_reg_32bit(
        emu: &EmuEnv,
        pc_address: u64,
        tcg: &TCGOp,
        mc: &mut Vec<u8>,
    ) -> usize {
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();

        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_load_gpr_32bit(emu, X86TargetRM::RAX, arg1.value, mc);
        gen_size += Self::tcg_gen_store_fregs_32bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }


    fn tcg_gen_sgnj_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(op, TCGOpcode::SGNJ_64BIT);
        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, 0x7fffffff_ffffffff, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RCX, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(arg1.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, 0x80000000_00000000, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(arg2.value) as u64, 4, mc);

        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::OR_GV_EV, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_gen_store_fregs_64bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_sgnjn_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(op, TCGOpcode::SGNJN_64BIT);
        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, 0x7fffffff_ffffffff, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RCX, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(arg1.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_load_fregs_64bit(emu, X86TargetRM::RDX, arg2.value, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::NEG_GV, X86ModRM::MOD_11_DISP_RDX, X86TargetRM::RDX, mc);
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, 0x80000000_00000000, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_11_DISP_RDX, X86TargetRM::RAX, mc);

        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::OR_GV_EV, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_gen_store_fregs_64bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }


    fn tcg_gen_sgnjx_64bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(op, TCGOpcode::SGNJX_64BIT);
        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, 0x7fffffff_ffffffff, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RCX, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(arg1.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_load_fregs_64bit(emu, X86TargetRM::RDX, arg1.value, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::XOR_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RDX, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(arg2.value) as u64, 4, mc);
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, 0x80000000_00000000, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_11_DISP_RDX, X86TargetRM::RAX, mc);

        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::OR_GV_EV, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_gen_store_fregs_64bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_sgnj_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(op, TCGOpcode::SGNJ_32BIT);
        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, 0x7fffffff, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RCX, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(arg1.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, 0x80000000, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(arg2.value) as u64, 4, mc);

        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::OR_GV_EV, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_gen_store_fregs_32bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_sgnjn_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(op, TCGOpcode::SGNJN_32BIT);
        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, 0x7fffffff, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RCX, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(arg1.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_load_fregs_64bit(emu, X86TargetRM::RDX, arg2.value, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::NEG_GV, X86ModRM::MOD_11_DISP_RDX, X86TargetRM::RDX, mc);
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, 0x80000000, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_11_DISP_RDX, X86TargetRM::RAX, mc);

        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::OR_GV_EV, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_gen_store_fregs_32bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }


    fn tcg_gen_sgnjx_32bit(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let op = tcg.op.unwrap();
        let arg0 = tcg.arg0.unwrap();
        let arg1 = tcg.arg1.unwrap();
        let arg2 = tcg.arg2.unwrap();

        assert_eq!(op, TCGOpcode::SGNJX_32BIT);
        assert_eq!(arg0.t, TCGvType::Register);
        assert_eq!(arg1.t, TCGvType::Register);
        assert_eq!(arg2.t, TCGvType::Register);

        let mut gen_size: usize = pc_address as usize;

        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RCX, 0x7fffffff, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RCX, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(arg1.value) as u64, 4, mc);

        gen_size += Self::tcg_gen_load_fregs_64bit(emu, X86TargetRM::RDX, arg1.value, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::XOR_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RDX, mc);
        gen_size += Self::tcg_out(emu.calc_fregs_relat_address(arg2.value) as u64, 4, mc);
        gen_size += Self::tcg_gen_imm_u64(X86TargetRM::RAX, 0x80000000, mc);
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_EV, X86ModRM::MOD_11_DISP_RDX, X86TargetRM::RAX, mc);

        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::OR_GV_EV, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_gen_store_fregs_32bit(emu, X86TargetRM::RAX, arg0.value, mc);

        return gen_size;
    }

    fn tcg_gen_cmp_eq(_emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let mut gen_size: usize = pc_address as usize;

        let src1 = tcg.arg0.unwrap();
        let src2 = tcg.arg1.unwrap();

        assert_eq!(src1.t, TCGvType::TCGTemp);
        assert_eq!(src2.t, TCGvType::TCGTemp);

        let src1_x86reg = Self::convert_x86_reg(src1.value);
        let src2_x86reg = Self::convert_x86_reg(src2.value);

        // Compare Offset Table and address upper bit
        gen_size += Self::tcg_modrm_64bit_raw_out(X86Opcode::CMP_GV_EV, X86ModRM::MOD_11_DISP_RAX as u8 + src1_x86reg as u8, src2_x86reg as u8, mc);

        gen_size += Self::tcg_out(X86Opcode::JE_rel16_32 as u64, 2, mc);
        gen_size += Self::tcg_out(0x4a as u64, 4, mc);

        gen_size
    }

    fn tcg_gen_match_check(emu: &EmuEnv, pc_address: u64, tcg: &TCGOp, mc: &mut Vec<u8>) -> usize {
        let mut gen_size: usize = pc_address as usize;

        let base_addr = tcg.arg0.unwrap();
        let addr_imm = tcg.arg1.unwrap();

        assert_eq!(base_addr.t, TCGvType::Register);
        assert_eq!(addr_imm.t, TCGvType::Immediate);

        // Load value from rs1
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_10_DISP_RBP, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_gpr_relat_address(base_addr.value) as u64, 4, mc);
        // rs1 + imm
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_IMM /* ADD_EV_IV */, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(addr_imm.value as u64, 4, mc);

        // Move RAX --> RCX
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RCX, mc);

        // Shift right 12-bit to search TLB table (RAX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::SRL_GV_IMM, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(12, 1, mc);
        

        // Shift right 12-bit to search TLB table (RCX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::SRL_GV_IMM, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(12, 1, mc);

        // Extract lower 12-bit to search TLB table offset (RAX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::AND_GV_IMM, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(0x0fff, 4, mc);

        // Right shift 3 bit to align 64-bit entry size (RAX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::SLL_GV_IMM, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(3, 1, mc);
        
        // Move RAX --> RDX (Extracted Address Offset)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RDX, mc);

        // Move RBP --> RAX (Base Address)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::MOV_GV_EV, X86ModRM::MOD_11_DISP_RBP, X86TargetRM::RAX, mc);
        
        // Shift right more 12-bit to search TLB table (RCX)
        gen_size += Self::tcg_modrm_64bit_out(X86Opcode::SRL_GV_IMM, X86ModRM::MOD_11_DISP_RCX, X86TargetRM::RCX, mc);
        gen_size += Self::tcg_out(12, 1, mc);
        
        // Load TLB Vector Base Address
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_IMM /* ADD_EV_IV */, X86ModRM::MOD_11_DISP_RAX, X86TargetRM::RAX, mc);
        gen_size += Self::tcg_out(emu.calc_tlb_relat_address() as u64, 4, mc);

        // Add TLB base address offset (RAX)
        Self::tcg_modrm_64bit_out(X86Opcode::ADD_GV_EV, X86ModRM::MOD_11_DISP_RDX, X86TargetRM::RAX, mc);

        return gen_size;
    }
}
