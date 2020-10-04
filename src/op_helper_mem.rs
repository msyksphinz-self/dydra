use crate::emu_env::EmuEnv;
use crate::target::riscv::mmu::{MemAccType};

impl EmuEnv {
    pub fn helper_func_load64(
        emu: &mut EmuEnv,
        rd: u32,
        rs1: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr :u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Read) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.m_regs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as u64;
        return 0;
    }

    pub fn helper_func_load32(
        emu: &mut EmuEnv,
        rd: u32,
        rs1: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Read) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.m_regs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as u64;
        return 0;
    }

    pub fn helper_func_load16(
        emu: &mut EmuEnv,
        rd: u32,
        rs1: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Read) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.m_regs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as u64;
        return 0;
    }

    pub fn helper_func_load8(
        emu: &mut EmuEnv,
        rd: u32,
        rs1: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Read) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.m_regs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as u64;
        return 0;
    }

    pub fn helper_func_loadu32(
        emu: &mut EmuEnv,
        rd: u32,
        rs1: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Read) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.m_regs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as u64;
        return 0;
    }

    pub fn helper_func_loadu16(
        emu: &mut EmuEnv,
        rd: u32,
        rs1: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Read) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.m_regs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as u64;
        return 0;
    }

    pub fn helper_func_loadu8(
        emu: &mut EmuEnv,
        rd: u32,
        rs1: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Read) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.m_regs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as u64;
        return 0;
    }

    pub fn helper_func_store64(
        emu: &mut EmuEnv,
        rs1: u32,
        rs2: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let rs2_data = emu.m_regs[rs2 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr: u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Write) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.write_mem_4byte(guest_phy_addr, rs2_data as u32);
    return 0;}

    pub fn helper_func_store32(
        emu: &mut EmuEnv,
        rs1: u32,
        rs2: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let rs2_data = emu.m_regs[rs2 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Write) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.write_mem_4byte(guest_phy_addr, rs2_data as u32);
        return 0;
    }

    pub fn helper_func_store16(
        emu: &mut EmuEnv,
        rs1: u32,
        rs2: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let rs2_data = emu.m_regs[rs2 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Write) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.write_mem_4byte(guest_phy_addr, rs2_data as u32);
        return 0;
    }

    pub fn helper_func_store8(
        emu: &mut EmuEnv,
        rs1: u32,
        rs2: u32,
        imm: u32,
        _dummy: u32,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let rs2_data = emu.m_regs[rs2 as usize];
        let addr = rs1_data + imm as u64;

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(addr, MemAccType::Write) {
            Ok(addr) => guest_phy_addr = addr,
            Err(error) => {
                panic!("Read Error: {:?}\n", error);
            }
        };
        emu.write_mem_4byte(guest_phy_addr, rs2_data as u32);
        return 0;
    }

}
