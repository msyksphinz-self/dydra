use crate::emu_env::EmuEnv;
use crate::target::riscv::mmu::{MemAccType};
use crate::target::riscv::riscv::ExceptCode;

impl EmuEnv {
    pub fn helper_func_load64(
        emu: &mut EmuEnv,
        rd: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        println!("load64 : converted address: {:016x}", addr);

        #[allow(unused_assignments)]
        let mut guest_phy_addr :u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("load64 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.m_regs[rd as usize] = emu.read_mem_8byte(guest_phy_addr) as u64;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };

        return 0;
    }

    pub fn helper_func_load32(
        emu: &mut EmuEnv,
        rd: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("load64 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.m_regs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as i32 as u64;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };
        return 0;
    }

    pub fn helper_func_load16(
        emu: &mut EmuEnv,
        rd: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("load64 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.m_regs[rd as usize] = emu.read_mem_2byte(guest_phy_addr) as i16 as u64;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };
        return 0;
    }

    pub fn helper_func_load8(
        emu: &mut EmuEnv,
        rd: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("load64 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.m_regs[rd as usize] = emu.read_mem_1byte(guest_phy_addr) as i8 as u64;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };
        return 0;
    }

    pub fn helper_func_loadu32(
        emu: &mut EmuEnv,
        rd: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("load64 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.m_regs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as u64;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };
        return 0;
    }

    pub fn helper_func_loadu16(
        emu: &mut EmuEnv,
        rd: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("load64 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.m_regs[rd as usize] = emu.read_mem_2byte(guest_phy_addr) as u64;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };
        return 0;
    }

    pub fn helper_func_loadu8(
        emu: &mut EmuEnv,
        rd: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("load64 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.m_regs[rd as usize] = emu.read_mem_1byte(guest_phy_addr) as u64;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };
        return 0;
    }

    pub fn helper_func_store64(
        emu: &mut EmuEnv,
        rs2: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let rs2_data = emu.m_regs[rs2 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        #[allow(unused_assignments)]
        let mut guest_phy_addr: u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Write) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("store64 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.write_mem_8byte(guest_phy_addr, rs2_data);
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };
    return 0;}

    pub fn helper_func_store32(
        emu: &mut EmuEnv,
        rs2: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let rs2_data = emu.m_regs[rs2 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Write) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("store32 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.write_mem_4byte(guest_phy_addr, rs2_data as u32);
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };
        return 0;
    }

    pub fn helper_func_store16(
        emu: &mut EmuEnv,
        rs2: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let rs2_data = emu.m_regs[rs2 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Write) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("store16 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.write_mem_2byte(guest_phy_addr, rs2_data as u16);
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };
        return 0;
    }

    pub fn helper_func_store8(
        emu: &mut EmuEnv,
        rs2: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_regs[rs1 as usize];
        let rs2_data = emu.m_regs[rs2 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        #[allow(unused_assignments)]
        let mut guest_phy_addr:u64 = 0;
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Write) {
            Ok(addr) => { 
                guest_phy_addr = addr; 
                println!("store8 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                emu.write_mem_1byte(guest_phy_addr, rs2_data as u8);
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
            }
        };
        return 0;
    }

}
