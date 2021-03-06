use super::super::super::tcg::tcg::{TCGLabel, TCGOp, TCGOpcode, TCGv};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

use super::super::super::instr_info::InstrInfo;
use super::riscv_inst_id::RiscvInstId;

#[derive(PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)]
pub enum ExceptCode {
    InstAddrMisalign = 0,
    InstAccessFault = 1,
    IllegalInst = 2,
    Breakpoint = 3,
    LoadAddrMisalign = 4,
    LoadAccessFault = 5,
    StoreAddrMisalign = 6,
    StoreAccessFault = 7,
    EcallFromUMode = 8,
    EcallFromSMode = 9,
    EcallFromHMode = 10,
    EcallFromMMode = 11,
    InstPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
}

#[allow(non_camel_case_types)]
pub enum CALL_HELPER_IDX {
    CALL_CSRRW_IDX = 0,
    CALL_CSRRS_IDX = 1,
    CALL_CSRRC_IDX = 2,
    CALL_CSRRWI_IDX = 3,
    CALL_CSRRSI_IDX = 4,
    CALL_CSRRCI_IDX = 5,
    CALL_MRET_IDX = 6,
    CALL_ECALL_IDX = 7,
    CALL_FADD_D_IDX = 8,
    CALL_FSUB_D_IDX = 9,
    CALL_FMUL_D_IDX = 10,
    CALL_FDIV_D_IDX = 11,
    CALL_FMADD_D_IDX = 12,
    CALL_FMSUB_D_IDX = 13,
    CALL_FNMSUB_D_IDX = 14,
    CALL_FNMADD_D_IDX = 15,
    CALL_FSQRT_D_IDX = 16,
    CALL_FEQ_D_IDX = 17,
    CALL_FLT_D_IDX = 18,
    CALL_FLE_D_IDX = 19,
    CALL_FCLASS_D_IDX = 20,
    CALL_FADD_S_IDX = 21,
    CALL_FSUB_S_IDX = 22,
    CALL_FMUL_S_IDX = 23,
    CALL_FDIV_S_IDX = 24,
    CALL_FMADD_S_IDX = 25,
    CALL_FMSUB_S_IDX = 26,
    CALL_FNMSUB_S_IDX = 27,
    CALL_FNMADD_S_IDX = 28,
    CALL_FSQRT_S_IDX = 29,
    CALL_FEQ_S_IDX = 30,
    CALL_FLT_S_IDX = 31,
    CALL_FLE_S_IDX = 32,
    CALL_FCLASS_S_IDX = 33,
    CALL_FMAX_D_IDX = 34,
    CALL_FMIN_D_IDX = 35,
    CALL_FMAX_S_IDX = 36,
    CALL_FMIN_S_IDX = 37,
    CALL_FSGNJ_S_IDX = 38,
    CALL_FSGNJN_S_IDX = 39,
    CALL_FSGNJX_S_IDX = 40,
    CALL_SRET_IDX = 41,
    CALL_LOAD64_IDX = 42,
    CALL_LOAD32_IDX = 43,
    CALL_LOAD16_IDX = 44,
    CALL_LOAD8_IDX  = 45,
    CALL_LOADU32_IDX = 46,
    CALL_LOADU16_IDX = 47,
    CALL_LOADU8_IDX  = 48,
    CALL_STORE64_IDX = 49,
    CALL_STORE32_IDX = 50,
    CALL_STORE16_IDX = 51,
    CALL_STORE8_IDX  = 52,
    CALL_FLOAT_LOAD64_IDX = 53,
    CALL_FLOAT_LOAD32_IDX = 54,
    CALL_FLOAT_STORE64_IDX = 55,
    CALL_FLOAT_STORE32_IDX = 56,
    CALL_SFENCE_VMA_IDX = 57,
    CALL_FCVT_IDX = 58,
}

#[allow(non_camel_case_types)]
pub enum CallFcvtIdx {
    W_S  = 0,
    WU_S = 1,
    S_W  = 2,
    S_WU = 3,
    S_D  = 4,
    D_S  = 5,
    W_D  = 6,
    WU_D = 7,
    D_W  = 8,
    D_WU = 9,
    L_S  = 10,
    LU_S = 11,
    S_L  = 12,
    S_LU = 13,
    L_D  = 14,
    LU_D = 15,
    D_L  = 16,
    D_LU = 17,
}
impl CallFcvtIdx {
    pub fn from_u64(from_bits: u64) -> CallFcvtIdx {
        match from_bits {
            0  => CallFcvtIdx::W_S ,
            1  => CallFcvtIdx::WU_S,
            2  => CallFcvtIdx::S_W ,
            3  => CallFcvtIdx::S_WU,
            4  => CallFcvtIdx::S_D ,
            5  => CallFcvtIdx::D_S ,
            6  => CallFcvtIdx::W_D ,
            7  => CallFcvtIdx::WU_D,
            8  => CallFcvtIdx::D_W ,
            9  => CallFcvtIdx::D_WU,
            10 => CallFcvtIdx::L_S ,
            11 => CallFcvtIdx::LU_S,
            12 => CallFcvtIdx::S_L ,
            13 => CallFcvtIdx::S_LU,
            14 => CallFcvtIdx::L_D ,
            15 => CallFcvtIdx::LU_D,
            16 => CallFcvtIdx::D_L ,
            17 => CallFcvtIdx::D_LU,
            _ => panic!("Unknown CallFcvtIdx : {:}", from_bits),
        }
    }
}


#[macro_export]
macro_rules! get_rs1_addr {
    ($inst:expr) => {
        ($inst >> 15) & 0x1f
    };
}

#[macro_export]
macro_rules! get_rs2_addr {
    ($inst:expr) => {
        ($inst >> 20) & 0x1f
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! get_rs3_addr {
    ($inst:expr) => {
        ($inst >> 27) & 0x1f
    };
}

#[macro_export]
macro_rules! get_rd_addr {
    ($inst:expr) => {
        ($inst >> 7) & 0x1f
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! get_imm12 {
    ($inst:expr) => {
        ($inst >> 20) as u64
    };
}

#[macro_export]
macro_rules! get_sb_field {
    ($inst:expr) => {
        ((($inst as u64 >> 7) & 0x01) << 11)
            | ((($inst as u64 >> 8) & 0x0f) << 1)
            | ((($inst as u64 >> 25) & 0x3f) << 5)
            | ((($inst as u64 >> 31) & 0x01) << 12) as u64
    };
}

#[macro_export]
macro_rules! extract_j_field {
    ($inst:expr) => {
        ((((($inst >> 21) & 0x3ff) << 1)
            | ((($inst >> 20) & 0x001) << 11)
            | ((($inst >> 12) & 0x0ff) << 12)
            | ((($inst >> 31) & 0x001) << 20)) as i32) as u64
    };
}

macro_rules! get_s_imm_field {
    ($inst:expr) => {
        ((((($inst as u64 >> 25) & 0x7f) << 5) | ($inst as u64 >> 7 & 0x1f)) as i32) as u64
    };
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum PrivMode {
    User,
    Supervisor,
    Hypervisor,
    Machine,
}

impl PrivMode {
    pub fn from_u8(n: u8) -> PrivMode {
        match n {
            0 => PrivMode::User,
            1 => PrivMode::Supervisor,
            2 => PrivMode::Hypervisor,
            3 => PrivMode::Machine,
            _ => PrivMode::Machine,
        }
    }
}

pub type TCGRegType = u64;
pub struct TranslateRiscv {
    pub reg_bitmap: VecDeque<u64>,
}

impl TranslateRiscv {
    pub fn new() -> TranslateRiscv {
        let mut trans = TranslateRiscv {
            reg_bitmap: VecDeque::new()
        };
        for idx in 0..5 {
            trans.reg_bitmap.push_back(idx);
        }
        trans
    }

    pub fn tcg_temp_new(&mut self) -> TCGv {
        let new_idx = match self.reg_bitmap.pop_front() {
            Some(idx) => {
                idx
            }
            None => panic!("New temporaries not found."),
        };
        let new_v = TCGv::new_temp(new_idx as u64);
        new_v
    }

    pub fn tcg_temp_free(&mut self, idx: TCGv) {
        self.reg_bitmap.push_front(idx.value);
    }

    pub fn translate(&mut self, id: RiscvInstId, inst: &InstrInfo) -> Vec<TCGOp> {
        return match id {
            RiscvInstId::ADDI => self.translate_addi(inst),
            RiscvInstId::ADD => self.translate_add(inst),
            RiscvInstId::SUB => self.translate_sub(inst),
            RiscvInstId::AND => self.translate_and(inst),
            RiscvInstId::OR => self.translate_or(inst),
            RiscvInstId::XOR => self.translate_xor(inst),
            RiscvInstId::ANDI => self.translate_andi(inst),
            RiscvInstId::ORI => self.translate_ori(inst),
            RiscvInstId::XORI => self.translate_xori(inst),
            RiscvInstId::ADDW => self.translate_addw(inst),
            RiscvInstId::SUBW => self.translate_subw(inst),

            RiscvInstId::ADDIW => self.translate_addiw(inst),

            RiscvInstId::LUI => self.translate_lui(inst),
            RiscvInstId::AUIPC => self.translate_auipc(inst),

            RiscvInstId::BEQ => self.translate_beq(inst),
            RiscvInstId::BNE => self.translate_bne(inst),
            RiscvInstId::BLT => self.translate_blt(inst),
            RiscvInstId::BGE => self.translate_bge(inst),
            RiscvInstId::BLTU => self.translate_bltu(inst),
            RiscvInstId::BGEU => self.translate_bgeu(inst),

            RiscvInstId::LD => self.translate_ld(inst),
            RiscvInstId::LW => self.translate_lw(inst),
            RiscvInstId::LH => self.translate_lh(inst),
            RiscvInstId::LB => self.translate_lb(inst),
            RiscvInstId::LWU => self.translate_lwu(inst),
            RiscvInstId::LHU => self.translate_lhu(inst),
            RiscvInstId::LBU => self.translate_lbu(inst),
            RiscvInstId::SD => self.translate_sd(inst),
            RiscvInstId::SW => self.translate_sw(inst),
            RiscvInstId::SH => self.translate_sh(inst),
            RiscvInstId::SB => self.translate_sb(inst),

            RiscvInstId::SLLI => self.translate_slli(inst),
            RiscvInstId::SRLI => self.translate_srli(inst),
            RiscvInstId::SRAI => self.translate_srai(inst),
            RiscvInstId::SLL => self.translate_sll(inst),
            RiscvInstId::SRL => self.translate_srl(inst),
            RiscvInstId::SRA => self.translate_sra(inst),

            RiscvInstId::SLLIW => self.translate_slliw(inst),
            RiscvInstId::SRLIW => self.translate_srliw(inst),
            RiscvInstId::SRAIW => self.translate_sraiw(inst),
            RiscvInstId::SLLW => self.translate_sllw(inst),
            RiscvInstId::SRLW => self.translate_srlw(inst),
            RiscvInstId::SRAW => self.translate_sraw(inst),

            RiscvInstId::SLT => self.translate_slt(inst),
            RiscvInstId::SLTI => self.translate_slti(inst),
            RiscvInstId::SLTU => self.translate_sltu(inst),
            RiscvInstId::SLTIU => self.translate_sltiu(inst),

            RiscvInstId::JALR => self.translate_jalr(inst),
            RiscvInstId::JAL => self.translate_jal(inst),

            RiscvInstId::CSRRS => self.translate_csrrs(inst),
            RiscvInstId::CSRRW => self.translate_csrrw(inst),
            RiscvInstId::CSRRC => self.translate_csrrc(inst),
            RiscvInstId::CSRRSI => self.translate_csrrsi(inst),
            RiscvInstId::CSRRWI => self.translate_csrrwi(inst),
            RiscvInstId::CSRRCI => self.translate_csrrci(inst),

            RiscvInstId::FENCE => self.translate_fence(inst),
            RiscvInstId::FENCE_I => self.translate_fence_i(inst),
            RiscvInstId::SFENCE_VMA => self.translate_sfence_vma(inst),
            RiscvInstId::MRET => self.translate_mret(inst),
            RiscvInstId::ECALL => self.translate_ecall(inst),
            RiscvInstId::SRET => self.translate_sret(inst),

            RiscvInstId::FLD => self.translate_fld(inst),
            RiscvInstId::FLW => self.translate_flw(inst),
            RiscvInstId::FSD => self.translate_fsd(inst),
            RiscvInstId::FSW => self.translate_fsw(inst),

            RiscvInstId::FADD_D => self.translate_fadd_d(inst),
            RiscvInstId::FSUB_D => self.translate_fsub_d(inst),
            RiscvInstId::FMUL_D => self.translate_fmul_d(inst),
            RiscvInstId::FDIV_D => self.translate_fdiv_d(inst),

            RiscvInstId::FMADD_D => self.translate_fmadd_d(inst),
            RiscvInstId::FMSUB_D => self.translate_fmsub_d(inst),
            RiscvInstId::FNMSUB_D => self.translate_fnmsub_d(inst),
            RiscvInstId::FNMADD_D => self.translate_fnmadd_d(inst),

            RiscvInstId::FSQRT_D => self.translate_fsqrt_d(inst),

            RiscvInstId::FMV_X_D => self.translate_fmv_x_d(inst),
            RiscvInstId::FMV_D_X => self.translate_fmv_d_x(inst),

            RiscvInstId::FEQ_D => self.translate_feq_d(inst),
            RiscvInstId::FLT_D => self.translate_flt_d(inst),
            RiscvInstId::FLE_D => self.translate_fle_d(inst),
            RiscvInstId::FCLASS_D => self.translate_fclass_d(inst),

            RiscvInstId::FMIN_D => self.translate_fmin_d(inst),
            RiscvInstId::FMAX_D => self.translate_fmax_d(inst),

            RiscvInstId::FSGNJ_D  => self.translate_fsgnj_d(inst),
            RiscvInstId::FSGNJN_D => self.translate_fsgnjn_d(inst),
            RiscvInstId::FSGNJX_D => self.translate_fsgnjx_d(inst),

            RiscvInstId::FADD_S => self.translate_fadd_s(inst),
            RiscvInstId::FSUB_S => self.translate_fsub_s(inst),
            RiscvInstId::FMUL_S => self.translate_fmul_s(inst),
            RiscvInstId::FDIV_S => self.translate_fdiv_s(inst),

            RiscvInstId::FMADD_S => self.translate_fmadd_s(inst),
            RiscvInstId::FMSUB_S => self.translate_fmsub_s(inst),
            RiscvInstId::FNMSUB_S => self.translate_fnmsub_s(inst),
            RiscvInstId::FNMADD_S => self.translate_fnmadd_s(inst),

            RiscvInstId::FSQRT_S => self.translate_fsqrt_s(inst),

            RiscvInstId::FMV_X_W => self.translate_fmv_x_w(inst),
            RiscvInstId::FMV_W_X => self.translate_fmv_w_x(inst),

            RiscvInstId::FEQ_S => self.translate_feq_s(inst),
            RiscvInstId::FLT_S => self.translate_flt_s(inst),
            RiscvInstId::FLE_S => self.translate_fle_s(inst),
            RiscvInstId::FCLASS_S => self.translate_fclass_s(inst),

            RiscvInstId::FMIN_S => self.translate_fmin_s(inst),
            RiscvInstId::FMAX_S => self.translate_fmax_s(inst),

            RiscvInstId::FSGNJ_S  => self.translate_fsgnj_s(inst),
            RiscvInstId::FSGNJN_S => self.translate_fsgnjn_s(inst),
            RiscvInstId::FSGNJX_S => self.translate_fsgnjx_s(inst),

            RiscvInstId::MUL    => self.translate_mul(inst),
            RiscvInstId::MULH   => self.translate_mulh(inst),
            RiscvInstId::MULHU  => self.translate_mulhu(inst),
            RiscvInstId::MULHSU => self.translate_mulhsu(inst),
            RiscvInstId::MULW   => self.translate_mulw(inst),

            RiscvInstId::DIV   => self.translate_div(inst),
            RiscvInstId::DIVU  => self.translate_divu(inst),
            RiscvInstId::DIVW  => self.translate_divw(inst),
            RiscvInstId::DIVUW => self.translate_divuw(inst),

            RiscvInstId::REM   => self.translate_rem(inst),
            RiscvInstId::REMU  => self.translate_remu(inst),
            RiscvInstId::REMW  => self.translate_remw(inst),
            RiscvInstId::REMUW => self.translate_remuw(inst),

            RiscvInstId::FCVT_W_S  => self.translate_fcvt_w_s(inst),
            RiscvInstId::FCVT_WU_S => self.translate_fcvt_wu_s(inst),
            RiscvInstId::FCVT_S_W  => self.translate_fcvt_s_w(inst),
            RiscvInstId::FCVT_S_WU => self.translate_fcvt_s_wu(inst),
            RiscvInstId::FCVT_S_D  => self.translate_fcvt_s_d(inst),
            RiscvInstId::FCVT_D_S  => self.translate_fcvt_d_s(inst),
            RiscvInstId::FCVT_W_D  => self.translate_fcvt_w_d(inst),
            RiscvInstId::FCVT_WU_D => self.translate_fcvt_wu_d(inst),
            RiscvInstId::FCVT_D_W  => self.translate_fcvt_d_w(inst),
            RiscvInstId::FCVT_D_WU => self.translate_fcvt_d_wu(inst),
            RiscvInstId::FCVT_L_S  => self.translate_fcvt_l_s(inst),
            RiscvInstId::FCVT_LU_S => self.translate_fcvt_lu_s(inst),
            RiscvInstId::FCVT_S_L  => self.translate_fcvt_s_l(inst),
            RiscvInstId::FCVT_S_LU => self.translate_fcvt_s_lu(inst),
            RiscvInstId::FCVT_L_D  => self.translate_fcvt_l_d(inst),
            RiscvInstId::FCVT_LU_D => self.translate_fcvt_lu_d(inst),
            RiscvInstId::FCVT_D_L  => self.translate_fcvt_d_l(inst),
            RiscvInstId::FCVT_D_LU => self.translate_fcvt_d_lu(inst),

            RiscvInstId::C_ADDI4SPN => self.translate_c_addi4spn(inst),
            RiscvInstId::C_FLD      => self.translate_c_fld     (inst),
            RiscvInstId::C_LW       => self.translate_c_lw      (inst),
            RiscvInstId::C_FLW      => self.translate_c_flw     (inst),
            RiscvInstId::C_LD       => self.translate_c_ld      (inst),
            RiscvInstId::C_FSD      => self.translate_c_fsd     (inst),
            RiscvInstId::C_SW       => self.translate_c_sw      (inst),
            RiscvInstId::C_FSW      => self.translate_c_fsw     (inst),
            RiscvInstId::C_SD       => self.translate_c_sd      (inst),
            RiscvInstId::C_NOP      => self.translate_c_nop     (inst),
            RiscvInstId::C_ADDI     => self.translate_c_addi    (inst),
            RiscvInstId::C_JAL      => self.translate_c_jal     (inst),
            RiscvInstId::C_ADDIW    => self.translate_c_addiw   (inst),
            RiscvInstId::C_LI       => self.translate_c_li      (inst),
            RiscvInstId::C_ADDI16SP => self.translate_c_addi16sp(inst),
            RiscvInstId::C_LUI      => self.translate_c_lui     (inst),
            RiscvInstId::C_SRLI     => self.translate_c_srli    (inst),
            RiscvInstId::C_SRLI64   => self.translate_c_srli64  (inst),
            RiscvInstId::C_SRAI     => self.translate_c_srai    (inst),
            RiscvInstId::C_SRAI64   => self.translate_c_srai64  (inst),
            RiscvInstId::C_ANDI     => self.translate_c_andi    (inst),
            RiscvInstId::C_SUB      => self.translate_c_sub     (inst),
            RiscvInstId::C_XOR      => self.translate_c_xor     (inst),
            RiscvInstId::C_OR       => self.translate_c_or      (inst),
            RiscvInstId::C_AND      => self.translate_c_and     (inst),
            RiscvInstId::C_SUBW     => self.translate_c_subw    (inst),
            RiscvInstId::C_ADDW     => self.translate_c_addw    (inst),
            RiscvInstId::C_J        => self.translate_c_j       (inst),
            RiscvInstId::C_BEQZ     => self.translate_c_beqz    (inst),
            RiscvInstId::C_BNEZ     => self.translate_c_bnez    (inst),
            RiscvInstId::C_SLLI     => self.translate_c_slli    (inst),
            RiscvInstId::C_FLDSP    => self.translate_c_fldsp   (inst),
            RiscvInstId::C_LWSP     => self.translate_c_lwsp    (inst),
            RiscvInstId::C_FLWSP    => self.translate_c_flwsp   (inst),
            RiscvInstId::C_LDSP     => self.translate_c_ldsp    (inst),
            RiscvInstId::C_JR       => self.translate_c_jr      (inst),
            RiscvInstId::C_MV       => self.translate_c_mv      (inst),
            RiscvInstId::C_EBREAK   => self.translate_c_ebreak  (inst),
            RiscvInstId::C_JALR     => self.translate_c_jalr    (inst),
            RiscvInstId::C_ADD      => self.translate_c_add     (inst),
            RiscvInstId::C_FSDSP    => self.translate_c_fsdsp   (inst),
            RiscvInstId::C_SWSP     => self.translate_c_swsp    (inst),
            RiscvInstId::C_FSWSP    => self.translate_c_fswsp   (inst),
            RiscvInstId::C_SDSP     => self.translate_c_sdsp    (inst),

            other_id => panic!("InstID={:?} : Not supported these instructions.", other_id),
        };
    }

    pub fn translate_rrr(&mut self, op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr= get_rs1_addr!(inst.inst);
        let rs2_addr= get_rs2_addr!(inst.inst);
        let rd_addr = get_rd_addr!(inst.inst);

        if rd_addr == 0 {
            return vec![];
        }

        let source1 = self.tcg_temp_new();
        let source2 = self.tcg_temp_new();

        let rs1_op = TCGOp::tcg_get_gpr(source1, rs1_addr);
        let rs2_op = TCGOp::tcg_get_gpr(source2, rs2_addr);  // Box::new(TCGv::new_reg(rs2_addr as u64));

        let tcg_inst = TCGOp::new_3op(op, source1, source1, source2);

        let rd_op = TCGOp::tcg_set_gpr(rd_addr, source1);

        self.tcg_temp_free(source2);
        self.tcg_temp_free(source1);

        vec![rs1_op, rs2_op, tcg_inst, rd_op]
    }

    pub fn translate_rrr_32bit(&mut self, op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr= get_rs1_addr!(inst.inst);
        let rs2_addr= get_rs2_addr!(inst.inst);
        let rd_addr = get_rd_addr!(inst.inst);

        if rd_addr == 0 {
            return vec![];
        }

        let source1 = self.tcg_temp_new();
        let source2 = self.tcg_temp_new();

        let mut tcg_list = vec![];

        tcg_list.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_list.push(TCGOp::tcg_get_gpr(source2, rs2_addr));

        tcg_list.push(TCGOp::new_3op(op, source1, source1, source2));
        tcg_list.push(TCGOp::new_2op(TCGOpcode::SIGN_EXT_32_64, source1, source1));
        tcg_list.push(TCGOp::tcg_set_gpr(rd_addr, source1));

        self.tcg_temp_free(source2);
        self.tcg_temp_free(source1);

        tcg_list
    }


    pub fn translate_rri(&mut self, op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr= get_rs1_addr!(inst.inst);
        let rd_addr = get_rd_addr!(inst.inst);

        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let tcg_imm = TCGv::new_imm(imm_const);

        if rd_addr == 0 {
            return vec![];
        }

        let source1 = self.tcg_temp_new();
        let rs1_op = TCGOp::tcg_get_gpr(source1, rs1_addr);
        let tcg_inst = TCGOp::new_3op(op, source1, source1, tcg_imm);
        let rd_op = TCGOp::tcg_set_gpr(rd_addr, source1);
        self.tcg_temp_free(source1);
        vec![rs1_op, tcg_inst, rd_op]
    }

    pub fn translate_shift_r(&mut self, op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr= get_rs1_addr!(inst.inst);
        let rs2_addr= get_rs2_addr!(inst.inst);
        let rd_addr = get_rd_addr!(inst.inst);

        if rd_addr == 0 {
            return vec![];
        }

        let source1 = self.tcg_temp_new();
        let source2 = self.tcg_temp_new();

        let mut tcg_list = vec![];

        tcg_list.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_list.push(TCGOp::tcg_get_gpr(source2, rs2_addr));

        tcg_list.push(TCGOp::new_3op(op, source1, source1, source2));
        if op != TCGOpcode::SLL_64BIT && op != TCGOpcode::SRA_64BIT && op != TCGOpcode::SRL_64BIT {
            tcg_list.push(TCGOp::new_2op(TCGOpcode::SIGN_EXT_32_64, source1, source1));
        }
        tcg_list.push(TCGOp::tcg_set_gpr(rd_addr, source1));

        self.tcg_temp_free(source2);
        self.tcg_temp_free(source1);

        tcg_list
    }


    pub fn translate_shift_i(&mut self, op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr = get_rs1_addr!(inst.inst);
        let imm_const: u64 = ((inst.inst >> 20) & 0x3f) as u64;
        let rd_addr = get_rd_addr!(inst.inst);

        if rd_addr == 0 {
            return vec![];
        }

        let mut tcg_list = vec![];

        let source1 = self.tcg_temp_new();
        tcg_list.push(TCGOp::tcg_get_gpr(source1, rs1_addr));
        tcg_list.push(TCGOp::new_3op(op, source1, source1, TCGv::new_imm(imm_const)));
        if op != TCGOpcode::SLL_64BIT && op != TCGOpcode::SRA_64BIT && op != TCGOpcode::SRL_64BIT {
            tcg_list.push(TCGOp::new_2op(TCGOpcode::SIGN_EXT_32_64, source1, source1));
        }
        tcg_list.push(TCGOp::tcg_set_gpr(rd_addr, source1));
        self.tcg_temp_free(source1);

        tcg_list
    }

    /*
    pub fn translate_store(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = get_s_imm_field!(inst.inst);
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;

        let imm_const = ((imm_const as i32) << (32 - 12)) >> (32 - 12);

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const as i64 as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));

        let tcg_inst = TCGOp::new_3op(op, *rs1, *rs2, *imm);

        vec![tcg_inst]
    }
    */

    pub fn translate_branch(&mut self, op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(inst.inst) as usize;
        let target: u64 = get_sb_field!(inst.inst);
        let target = ((target as i64) << (64 - 13)) >> (64 - 13);
        let target = inst.addr.wrapping_add(target as u64);

        // let rs1 = TCGv::new_reg(rs1_addr as u64);
        // let rs2 = TCGv::new_reg(rs2_addr as u64);
        let addr = TCGv::new_imm(target as u64);

        let label = Rc::new(RefCell::new(TCGLabel::new()));

        let rs1 = self.tcg_temp_new();
        let rs2 = self.tcg_temp_new();

        let mut tcg_list = vec![];

        tcg_list.push(TCGOp::tcg_get_gpr(rs1, rs1_addr as u32));
        tcg_list.push(TCGOp::tcg_get_gpr(rs2, rs2_addr as u32));

        tcg_list.push(TCGOp::new_4op(op, rs1, rs2, addr, Rc::clone(&label)));
        tcg_list.push(TCGOp::new_goto_tb(TCGv::new_imm(inst.addr + 4)));
        tcg_list.push(TCGOp::new_label(Rc::clone(&label)));
        tcg_list.push(TCGOp::new_goto_tb(TCGv::new_imm(target  as u64)));

        self.tcg_temp_free(rs2);
        self.tcg_temp_free(rs1);

        tcg_list
    }

    pub fn translate_float_rri(op: TCGOpcode, inst: &InstrInfo) -> Vec<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(inst.inst) as usize;
        let imm_const: u64 = ((inst.inst as i32) >> 20) as u64;
        let rd_addr: usize = get_rd_addr!(inst.inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = TCGOp::new_3op(op, *rd, *rs1, *imm);
        return vec![tcg_inst];
    }

}
