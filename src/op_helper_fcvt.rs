use crate::emu_env::EmuEnv;
use crate::target::riscv::riscv::CallFcvtIdx;
use crate::target::riscv::riscv_csr::CsrAddr;
use softfloat_wrapper::{ExceptionFlags, Float, RoundingMode, F32, F64};

impl EmuEnv {
    pub fn helper_func_fcvt(emu: &mut EmuEnv, call_idx: u64, rd: u64, rs1: u64, _: u64) -> usize {
        let mut flag = ExceptionFlags::default();
        flag.set();
        let helper_idx = CallFcvtIdx::from_u64(call_idx);
        match helper_idx {
            CallFcvtIdx::W_S => {
                let to_data = F32::from_bits(emu.m_fregs[rs1 as usize] as u32)
                    .to_i32(RoundingMode::TowardZero, true);
                emu.m_iregs[rd as usize] = to_data as u64;
            }
            CallFcvtIdx::WU_S => {
                let to_data = F32::from_bits(emu.m_fregs[rs1 as usize] as u32)
                    .to_u32(RoundingMode::TowardZero, true);
                emu.m_iregs[rd as usize] = to_data as i32 as u64;
            }
            CallFcvtIdx::S_W => {
                let to_data =
                    F32::from_i32(emu.m_iregs[rs1 as usize] as i32, RoundingMode::TowardZero);
                emu.m_fregs[rd as usize] = to_data.bits() as u64;
            }
            CallFcvtIdx::S_WU => {
                let to_data =
                    F32::from_u32(emu.m_iregs[rs1 as usize] as u32, RoundingMode::TiesToEven);
                emu.m_fregs[rd as usize] = to_data.bits() as u64;
            }
            CallFcvtIdx::S_D => {
                let to_data = F64::from_bits(emu.m_fregs[rs1 as usize] as u64)
                    .to_f32(RoundingMode::TowardZero);
                emu.m_fregs[rd as usize] = to_data.bits() as u64;
            }
            CallFcvtIdx::D_S => {
                let to_data = F32::from_bits(emu.m_fregs[rs1 as usize] as u32)
                    .to_f64(RoundingMode::TowardZero);
                emu.m_fregs[rd as usize] = to_data.bits() as u64;
            }
            CallFcvtIdx::W_D => {
                let to_data = F64::from_bits(emu.m_fregs[rs1 as usize] as u64)
                    .to_i32(RoundingMode::TowardZero, true);
                emu.m_iregs[rd as usize] = to_data as u64;
            }
            CallFcvtIdx::WU_D => {
                let to_data = F64::from_bits(emu.m_fregs[rs1 as usize] as u64)
                    .to_u32(RoundingMode::TowardZero, true);
                emu.m_iregs[rd as usize] = to_data as i32 as u64;
            }
            CallFcvtIdx::D_W => {
                let to_data =
                    F64::from_i32(emu.m_iregs[rs1 as usize] as i32, RoundingMode::TowardZero);
                emu.m_fregs[rd as usize] = to_data.bits() as u64;
            }
            CallFcvtIdx::D_WU => {
                let to_data =
                    F64::from_u32(emu.m_iregs[rs1 as usize] as u32, RoundingMode::TowardZero);
                emu.m_fregs[rd as usize] = to_data.bits() as u64;
            }
            CallFcvtIdx::L_S => {
                let to_data = F32::from_bits(emu.m_fregs[rs1 as usize] as u32)
                    .to_i64(RoundingMode::TowardZero, true);
                emu.m_iregs[rd as usize] = to_data as u64;
            }
            CallFcvtIdx::LU_S => {
                let to_data = F32::from_bits(emu.m_fregs[rs1 as usize] as u32)
                    .to_u64(RoundingMode::TowardZero, true);
                emu.m_iregs[rd as usize] = to_data as u64;
            }
            CallFcvtIdx::S_L => {
                let to_data =
                    F32::from_i64(emu.m_iregs[rs1 as usize] as i64, RoundingMode::TowardZero);
                emu.m_fregs[rd as usize] = to_data.bits() as u64;
            }
            CallFcvtIdx::S_LU => {
                let to_data =
                    F32::from_u64(emu.m_iregs[rs1 as usize] as u64, RoundingMode::TiesToEven);
                emu.m_fregs[rd as usize] = to_data.bits() as u64;
            }
            CallFcvtIdx::L_D => {
                let to_data = F64::from_bits(emu.m_fregs[rs1 as usize] as u64)
                    .to_i64(RoundingMode::TowardZero, true);
                emu.m_iregs[rd as usize] = to_data as u64;
            }
            CallFcvtIdx::LU_D => {
                let to_data = F64::from_bits(emu.m_fregs[rs1 as usize] as u64)
                    .to_u64(RoundingMode::TowardZero, true);
                emu.m_iregs[rd as usize] = to_data as u64;
            }
            CallFcvtIdx::D_L => {
                let to_data =
                    F64::from_i64(emu.m_iregs[rs1 as usize] as i64, RoundingMode::TowardZero);
                emu.m_fregs[rd as usize] = to_data.bits() as u64;
            }
            CallFcvtIdx::D_LU => {
                let to_data =
                    F64::from_u64(emu.m_iregs[rs1 as usize] as u64, RoundingMode::TiesToEven);
                emu.m_fregs[rd as usize] = to_data.bits() as u64;
            }
        };

        flag.get();
        let ret_flag = flag.bits();
        println!("ret_flags = {:x}", ret_flag);
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        return 0;
    }
}
