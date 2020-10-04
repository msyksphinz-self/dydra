use crate::target::riscv::riscv_csr::{CsrAddr};
use crate::target::riscv::riscv_csr_def;
use crate::emu_env::EmuEnv;
use crate::target::riscv::riscv::{ExceptCode, PrivMode};

impl EmuEnv {
    pub fn helper_func_csrrw(
        emu: &mut EmuEnv,
        dest: u32,
        source: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
        println!(
            "helper_csrrw(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, source, csr_addr
        );
        let data = emu.m_regs[source as usize];
        let reg_data = emu
            .m_csr
            .csrrw(CsrAddr::from_u64(csr_addr as u64), data as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        return 0;
    }

    pub fn helper_func_csrrs(
        emu: &mut EmuEnv,
        dest: u32,
        source: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
        println!(
            "helper_csrrs(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, source, csr_addr
        );
        let data = emu.m_regs[source as usize];
        let reg_data = emu
            .m_csr
            .csrrs(CsrAddr::from_u64(csr_addr as u64), data as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        return 0;
    }

    pub fn helper_func_csrrc(
        emu: &mut EmuEnv,
        dest: u32,
        source: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
        println!(
            "helper_csrrc(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, source, csr_addr
        );
        let data = emu.m_regs[source as usize];
        let reg_data = emu
            .m_csr
            .csrrc(CsrAddr::from_u64(csr_addr as u64), data as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        return 0;
    }

    pub fn helper_func_csrrwi(
        emu: &mut EmuEnv,
        dest: u32,
        imm: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
        println!(
            "helper_csrrw(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, imm, csr_addr
        );
        let reg_data = emu
            .m_csr
            .csrrw(CsrAddr::from_u64(csr_addr as u64), imm as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        return 0;
    }

    pub fn helper_func_csrrsi(
        emu: &mut EmuEnv,
        dest: u32,
        imm: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
        println!(
            "helper_csrrs(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, imm, csr_addr
        );
        let reg_data = emu
            .m_csr
            .csrrs(CsrAddr::from_u64(csr_addr as u64), imm as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        return 0;
    }

    pub fn helper_func_csrrci(
        emu: &mut EmuEnv,
        dest: u32,
        imm: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
        println!(
            "helper_csrrc(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, imm, csr_addr
        );
        let reg_data = emu
            .m_csr
            .csrrc(CsrAddr::from_u64(csr_addr as u64), imm as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        return 0;
    }

    pub fn helper_func_ecall( emu: &mut EmuEnv, dest: u32, imm: u32, csr_addr: u32, _dummy: u32) -> usize {
        println!(
            "helper_mret(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, imm, csr_addr
        );
        emu.m_csr.csrrw(CsrAddr::Mepc, emu.m_pc[0] as i64); // MEPC

        // let current_priv: PrivMode = self.m_priv;
        // match current_priv {
        //     PrivMode::User => self.generate_exception(ExceptCode::EcallFromUMode, 0),
        //     PrivMode::Supervisor => self.generate_exception(ExceptCode::EcallFromSMode, 0),
        //     PrivMode::Hypervisor => self.generate_exception(ExceptCode::EcallFromHMode, 0),
        //     PrivMode::Machine => self.generate_exception(ExceptCode::EcallFromMMode, 0),
        // }

        emu.generate_exception(ExceptCode::EcallFromMMode, 0);
        return 0;
    }

    pub fn helper_func_mret(emu: &mut EmuEnv, dest: u32, imm: u32, csr_addr: u32, _dummy: u32) -> usize {
        println!(
            "helper_mret(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, imm, csr_addr
        );
        emu.m_pc[0] = emu.m_csr.csrrc(CsrAddr::Mepc, 0 as i64) as u64;
        print!("PC is set to 0x{:08x}\n", emu.m_pc[0]);
        return 0;
    }
    
    pub fn helper_func_sret(emu: &mut EmuEnv, _dest: u32, _imm: u32, _csr_addr: u32, _dummy: u32) -> usize {
        let mstatus: i64 = emu.m_csr.csrrs(CsrAddr::Mstatus, PrivMode::Machine as i64);
        let next_priv_uint: i64 = Self::extract_bit_field( mstatus, riscv_csr_def::SYSREG_MSTATUS_SPP_MSB, riscv_csr_def::SYSREG_MSTATUS_SPP_LSB,
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
        println!("helper_sret(emu) is called! PC = {:012x}", emu.m_pc[0]);

        return 0;
    }



}
