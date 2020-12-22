use std::sync::atomic::Ordering;

use crate::emu_env::{EmuEnv, MachineEnum};
use crate::target::riscv::mmu::{MemAccType, MemResult};
use crate::target::riscv::riscv::ExceptCode;

impl EmuEnv {
    pub fn helper_func_load64(emu: &mut EmuEnv,rd: u64,rs1: u64,imm: u64,guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);
        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                // Update TLB List
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.m_iregs[rd as usize] = emu.read_mem_8byte(guest_phy_addr) as u64;
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_load32(emu: &mut EmuEnv, rd: u64, rs1: u64, imm: u64, guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("load32 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                if emu.m_arg_config.machine == MachineEnum::RiscvSiFiveU && (guest_phy_addr & !0xfff) == 0x1001_0000 {
                    if emu.m_arg_config.debug {
                        println!("UART Access : {:08x}", guest_phy_addr);
                    }
                    // region : 0x1000_0000 - 0x1000_0fff
                    match guest_phy_addr {
                        0x1001_0000 => { emu.m_iregs[rd as usize] = 0; },       // txdata
                        0x1001_0004 => { emu.m_iregs[rd as usize] = 0; },       // rxdata
                        0x1001_0008 => { emu.m_iregs[rd as usize] = 0; },       // txctrl
                        0x1001_000c => { emu.m_iregs[rd as usize] = 0; },       // rxctrl
                        _ => {},
                    }
                    return MemResult::NoExcept as usize;
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.m_iregs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as i32 as u64;
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_load16(emu: &mut EmuEnv, rd: u64, rs1: u64, imm: u64, guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("load16 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.m_iregs[rd as usize] = emu.read_mem_2byte(guest_phy_addr) as i16 as u64;
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_load8(emu: &mut EmuEnv, rd: u64, rs1: u64, imm: u64, guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("load8 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.m_iregs[rd as usize] = emu.read_mem_1byte(guest_phy_addr) as i8 as u64;
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_loadu32(emu: &mut EmuEnv, rd: u64, rs1: u64, imm: u64, guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("loadu32 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.m_iregs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as u64;
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_loadu16(emu: &mut EmuEnv, rd: u64, rs1: u64, imm: u64, guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("loadu16 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.m_iregs[rd as usize] = emu.read_mem_2byte(guest_phy_addr) as u64;
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_loadu8(emu: &mut EmuEnv, rd: u64, rs1: u64, imm: u64, guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("loadu8 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.m_iregs[rd as usize] = emu.read_mem_1byte(guest_phy_addr) as u64;
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::StorePageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_store64(
        emu: &mut EmuEnv,
        rs2: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let rs2_data = emu.m_iregs[rs2 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Write) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("store64 : converted address: {:016x} --> {:016x} <= {:016x}", addr, guest_phy_addr, rs2_data);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.write_mem_8byte(guest_phy_addr, rs2_data);
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::StorePageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_store32(emu: &mut EmuEnv, rs2: u64, rs1: u64, imm: u64, guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let rs2_data = emu.m_iregs[rs2 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Write) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("store32 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                if emu.m_arg_config.machine == MachineEnum::RiscvSiFiveU && (guest_phy_addr & !0xfff) == 0x1001_0000 {
                    if emu.m_arg_config.debug {
                        println!("UART Access : {:08x}", guest_phy_addr);
                    }
                    // region : 0x1000_0000 - 0x1000_0fff
                    match guest_phy_addr {
                        0x1001_0000 => { eprint!("{}", (rs2_data & 0xff) as u8 as char) },       // txdata
                        0x1001_0004 => { },       // rxdata
                        0x1001_0008 => { },       // txctrl
                        0x1001_000c => { },       // rxctrl
                        _ => {},
                    }
                    return MemResult::NoExcept as usize;
                }
                if emu.m_arg_config.machine == MachineEnum::RiscvSiFiveU && (guest_phy_addr & !0xfff) == 0x10_0000 {
                    emu.m_notify_exit.store(true, Ordering::Relaxed);
                    return MemResult::NoExcept as usize;
                }

                if emu.m_arg_config.machine == MachineEnum::RiscvVirt && guest_phy_addr == 0x80001000 {
                    emu.m_notify_exit.store(true, Ordering::Relaxed);
                    return MemResult::NoExcept as usize;
                }

                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.write_mem_4byte(guest_phy_addr, rs2_data as u32);
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::StorePageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_store16(
        emu: &mut EmuEnv,
        rs2: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let rs2_data = emu.m_iregs[rs2 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Write) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("store16 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.write_mem_2byte(guest_phy_addr, rs2_data as u16);
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::StorePageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_store8(
        emu: &mut EmuEnv,
        rs2: u64,
        rs1: u64,
        imm: u64,
        guest_pc: u64,
    ) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let rs2_data = emu.m_iregs[rs2 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Write) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("store8 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.write_mem_1byte(guest_phy_addr, rs2_data as u8);
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::StorePageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_float_load64(emu: &mut EmuEnv,rd: u64,rs1: u64,imm: u64,guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("loadf64 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.m_fregs[rd as usize] = emu.read_mem_8byte(guest_phy_addr) as u64;
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_float_load32(emu: &mut EmuEnv, rd: u64, rs1: u64, imm: u64, guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Read) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("loadf32 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                if emu.m_arg_config.mmu_debug {
                    println!("update tlb_vec[{:}] = {:016x}", ((addr >> 12) & 0xfff) as usize, addr >> (12 + 12));
                }
                emu.m_fregs[rd as usize] = emu.read_mem_4byte(guest_phy_addr) as u64 | 0xffffffff00000000;  // NaN Boxing
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::LoadPageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_float_store64(emu: &mut EmuEnv, rs2: u64, rs1: u64, imm: u64, guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let rs2_data = emu.m_fregs[rs2 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Write) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("storef64 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                emu.write_mem_8byte(guest_phy_addr, rs2_data);
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::StorePageFault, addr as i64);
                return error as usize;
            }
        };
    }

    pub fn helper_func_float_store32(emu: &mut EmuEnv, rs2: u64, rs1: u64, imm: u64, guest_pc: u64) -> usize {
        let rs1_data = emu.m_iregs[rs1 as usize];
        let rs2_data = emu.m_fregs[rs2 as usize];
        let addr = rs1_data.wrapping_add(imm as i32 as u64);

        match emu.convert_physical_address(guest_pc, addr, MemAccType::Write) {
            Ok(guest_phy_addr) => {
                if emu.m_arg_config.mmu_debug {
                    println!("storef32 : converted address: {:016x} --> {:016x}", addr, guest_phy_addr);
                }
                emu.m_tlb_vec[((addr >> 12) & 0xfff) as usize] = addr >> (12 + 12);
                emu.m_tlb_addr_vec[((addr >> 12) & 0xfff) as usize] = guest_phy_addr & !0xfff;
                emu.write_mem_4byte(guest_phy_addr, rs2_data as u32);
                return MemResult::NoExcept as usize;
            }
            Err(error) => {
                print!("Read Error: {:?}\n", error);
                emu.generate_exception(guest_pc, ExceptCode::StorePageFault, addr as i64);
                return error as usize;
            }
        };
    }

}
