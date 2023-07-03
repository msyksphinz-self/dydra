use crate::emu_env::EmuEnv;
use crate::target::riscv::riscv::{ExceptCode, PrivMode};
use crate::target::riscv::riscv_csr::CsrAddr;
use crate::target::riscv::riscv_csr_def;

impl EmuEnv {
    pub fn helper_func_csrrw(
        emu: &mut EmuEnv,
        dest: u64,
        source: u64,
        csr_addr: u64,
        _dummy: u64,
    ) -> usize {
        let data = emu.m_iregs[source as usize];
        let reg_data = emu
            .m_csr
            .csrrw(CsrAddr::from_u64(csr_addr as u64), data as i64);
        if dest != 0 {
            emu.m_iregs[dest as usize] = reg_data as u64;
        }
        return 0;
    }

    pub fn helper_func_csrrs(
        emu: &mut EmuEnv,
        dest: u64,
        source: u64,
        csr_addr: u64,
        _dummy: u64,
    ) -> usize {
        let data = emu.m_iregs[source as usize];
        let reg_data = emu
            .m_csr
            .csrrs(CsrAddr::from_u64(csr_addr as u64), data as i64);
        if dest != 0 {
            emu.m_iregs[dest as usize] = reg_data as u64;
        }
        return 0;
    }

    pub fn helper_func_csrrc(
        emu: &mut EmuEnv,
        dest: u64,
        source: u64,
        csr_addr: u64,
        _dummy: u64,
    ) -> usize {
        let data = emu.m_iregs[source as usize];
        let reg_data = emu
            .m_csr
            .csrrc(CsrAddr::from_u64(csr_addr as u64), data as i64);
        if dest != 0 {
            emu.m_iregs[dest as usize] = reg_data as u64;
        }
        return 0;
    }

    pub fn helper_func_csrrwi(
        emu: &mut EmuEnv,
        dest: u64,
        imm: u64,
        csr_addr: u64,
        _dummy: u64,
    ) -> usize {
        let reg_data = emu
            .m_csr
            .csrrw(CsrAddr::from_u64(csr_addr as u64), imm as i64);
        if dest != 0 {
            emu.m_iregs[dest as usize] = reg_data as u64;
        }
        return 0;
    }

    pub fn helper_func_csrrsi(
        emu: &mut EmuEnv,
        dest: u64,
        imm: u64,
        csr_addr: u64,
        _dummy: u64,
    ) -> usize {
        let reg_data = emu
            .m_csr
            .csrrs(CsrAddr::from_u64(csr_addr as u64), imm as i64);
        if dest != 0 {
            emu.m_iregs[dest as usize] = reg_data as u64;
        }
        return 0;
    }

    pub fn helper_func_csrrci(
        emu: &mut EmuEnv,
        dest: u64,
        imm: u64,
        csr_addr: u64,
        _dummy: u64,
    ) -> usize {
        let reg_data = emu
            .m_csr
            .csrrc(CsrAddr::from_u64(csr_addr as u64), imm as i64);
        if dest != 0 {
            emu.m_iregs[dest as usize] = reg_data as u64;
        }
        return 0;
    }

    pub fn helper_func_ecall(
        emu: &mut EmuEnv,
        _dest: u64,
        _imm: u64,
        _csr_addr: u64,
        guest_pc: u64,
    ) -> usize {
        emu.m_csr.csrrw(CsrAddr::Mepc, emu.m_pc[0] as i64); // MEPC

        let current_priv: PrivMode = emu.m_priv;
        match current_priv {
            PrivMode::User => emu.generate_exception(guest_pc, ExceptCode::EcallFromUMode, 0),
            PrivMode::Supervisor => emu.generate_exception(guest_pc, ExceptCode::EcallFromSMode, 0),
            PrivMode::Hypervisor => emu.generate_exception(guest_pc, ExceptCode::EcallFromHMode, 0),
            PrivMode::Machine => emu.generate_exception(guest_pc, ExceptCode::EcallFromMMode, 0),
        }

        return 0;
    }

    pub fn helper_func_mret(
        emu: &mut EmuEnv,
        _dest: u64,
        _imm: u64,
        _csr_addr: u64,
        _dummy: u64,
    ) -> usize {
        emu.m_pc[0] = emu.m_csr.csrrc(CsrAddr::Mepc, 0 as i64) as u64;
        return 0;
    }

    pub fn helper_func_sret(
        emu: &mut EmuEnv,
        _dest: u64,
        _imm: u64,
        _csr_addr: u64,
        _dummy: u64,
    ) -> usize {
        let mstatus: i64 = emu.m_csr.csrrs(CsrAddr::Mstatus, PrivMode::Machine as i64);
        let next_priv_uint: i64 = Self::extract_bit_field(
            mstatus,
            riscv_csr_def::SYSREG_MSTATUS_SPP_MSB,
            riscv_csr_def::SYSREG_MSTATUS_SPP_LSB,
        );
        let next_priv: PrivMode = PrivMode::from_u8(next_priv_uint as u8);
        let mut next_mstatus: i64 = mstatus;
        next_mstatus = Self::set_bit_field(
            next_mstatus,
            Self::extract_bit_field(
                mstatus,
                riscv_csr_def::SYSREG_MSTATUS_SPIE_MSB,
                riscv_csr_def::SYSREG_MSTATUS_SPIE_LSB,
            ),
            riscv_csr_def::SYSREG_MSTATUS_SIE_MSB,
            riscv_csr_def::SYSREG_MSTATUS_SIE_LSB,
        );
        next_mstatus = Self::set_bit_field(
            next_mstatus,
            1,
            riscv_csr_def::SYSREG_MSTATUS_SPIE_MSB,
            riscv_csr_def::SYSREG_MSTATUS_SPIE_LSB,
        );
        next_mstatus = Self::set_bit_field(
            next_mstatus,
            PrivMode::User as i64,
            riscv_csr_def::SYSREG_MSTATUS_SPP_MSB,
            riscv_csr_def::SYSREG_MSTATUS_SPP_LSB,
        );

        emu.m_csr.csrrw(CsrAddr::Mstatus, next_mstatus);
        let ret_pc = emu.m_csr.csrrs(CsrAddr::Sepc, 0);
        emu.m_priv = next_priv;

        emu.m_pc[0] = ret_pc as u64;

        return 0;
    }

    pub fn helper_func_sfence_vma(
        emu: &mut EmuEnv,
        _dest: u64,
        _imm: u64,
        _csr_addr: u64,
        _dummy: u64,
    ) -> usize {
        // Clear TLB
        for idx in 0..4096 {
            emu.m_tlb_vec[idx] = 0xdeadbeef_01234567;
        }

        for addr in emu.m_tb_text_hash_address.iter_mut() {
            *addr = 0xdeadbeef;
        }
        return 0;
    }
}
