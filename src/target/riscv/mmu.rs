use num::iter::range;

use crate::emu_env::EmuEnv;
use crate::target::riscv::riscv_csr::{CsrAddr};
use crate::target::riscv::riscv_csr_def;
use crate::target::riscv::riscv::{ExceptCode, PrivMode};

#[derive(Copy, Clone)]
pub enum MemAccType {
    Fetch,
    Write,
    Read,
}

#[derive(PartialEq, Eq)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum MemResult {
    NoExcept = 0,
    MisAlign = 1 << 0,
    NotDefined = 1 << 1,
    NewRegion = 1 << 2,
    TlbError = 1 << 3,
}

#[derive(PartialEq, Eq)]
#[allow(dead_code)]
pub enum VMMode {
    Mbare = 0,
    Sv32 = 1,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
    Sv64 = 11,
}
impl VMMode {
    pub fn from(x: i64) -> VMMode {
        match x {
            0 => VMMode::Mbare,
            1 => VMMode::Sv32,
            8 => VMMode::Sv39,
            9 => VMMode::Sv48,
            10 => VMMode::Sv57,
            11 => VMMode::Sv64,
            _ => panic!("Intelnal Error: Unknown VMMode = {:}", x),
        }
    }
}



impl EmuEnv {

    pub fn convert_physical_address(&mut self, virtual_addr: u64, acc_type: MemAccType) -> Result<u64, MemResult> {
        let is_fetch_access = match acc_type {
            MemAccType::Fetch => true,
            _ => false,
        };

        let mstatus: i64 = self.m_csr.csrrs(CsrAddr::Mstatus, PrivMode::Machine as i64);
        let mprv: u8 =
            Self::extract_bit_field(mstatus, riscv_csr_def::SYSREG_MSTATUS_MPRV_MSB, riscv_csr_def::SYSREG_MSTATUS_MPRV_LSB)
                as u8;
        let mpp_u8: u8 =
            Self::extract_bit_field(mstatus, riscv_csr_def::SYSREG_MSTATUS_MPP_MSB, riscv_csr_def::SYSREG_MSTATUS_MPP_LSB) as u8;
        let mpp: PrivMode = PrivMode::from_u8(mpp_u8);

        let priv_mode: PrivMode = if !is_fetch_access && (mprv != 0) {
            mpp
        } else {
            self.m_priv
        };

        println!("<Convert_Virtual_Address. virtual_addr={:016x} : vm_mode = {}, priv_mode = {}>",
                 virtual_addr, self.get_vm_mode() as u32, priv_mode as u32);

        if self.get_vm_mode() == VMMode::Sv39
            && (priv_mode == PrivMode::Supervisor || priv_mode == PrivMode::User)
        {
            let ppn_idx: Vec<u8> = vec![12, 21, 30];
            let pte_len: Vec<u8> = vec![9, 9, 26];
            let pte_idx: Vec<u8> = vec![10, 19, 28];
            let vpn_len: Vec<u8> = vec![9, 9, 9];
            let vpn_idx: Vec<u8> = vec![12, 21, 30];
            let pagesize: u32 = 4096; // num::pow(2, 12);
            let ptesize: u32 = 8;

            return self.walk_page_table(
                virtual_addr, acc_type, 3, ppn_idx, pte_len, pte_idx, vpn_len, vpn_idx, pagesize, ptesize,
            );
        } else if self.get_vm_mode() == VMMode::Sv32
            && (priv_mode == PrivMode::Supervisor || priv_mode == PrivMode::User)
        {
            let ppn_idx: Vec<u8> = vec![12, 22];
            let pte_len: Vec<u8> = vec![10, 12];
            let pte_idx: Vec<u8> = vec![10, 20];
            let vpn_len: Vec<u8> = vec![10, 10];
            let vpn_idx: Vec<u8> = vec![12, 22];
            let pagesize: u32 = 4096; // num::pow(2, 12);
            let ptesize: u32 = 4;

            return self.walk_page_table(
                virtual_addr, acc_type, 2, ppn_idx, pte_len, pte_idx, vpn_len, vpn_idx, pagesize, ptesize,
            );
        } else {
            return Ok(virtual_addr);
        }

    }

    fn walk_page_table(
        &mut self,
        virtual_addr: u64,
        acc_type: MemAccType,
        init_level: u32,
        ppn_idx: Vec<u8>,
        pte_len: Vec<u8>,
        pte_idx: Vec<u8>,
        vpn_len: Vec<u8>,
        vpn_idx: Vec<u8>,
        pagesize: u32,
        ptesize: u32,
    ) -> Result<u64, MemResult> {
        let is_write_access = match acc_type {
            MemAccType::Write => true,
            _ => false,
        };

        //===================
        // Simple TLB Search
        //===================
        // let virtual_addr_vpn: u64 = (virtual_addr >> 12);
        // let virtual_addr_tag: u8 = virtual_addr_vpn & (tlb_width-1);
        // if (m_tlb_en[virtual_addr_tag] && m_tlb_tag[virtual_addr_tag] == virtual_addr_vpn) {
        //     let paddr:u64 = (m_tlb_addr[virtual_addr_tag] & !0x0fff) + (virtual_addr & 0x0fff);
        //     let pte_val:u64 = m_tlb_addr[virtual_addr_tag] & 0x0ff;
        //
        //     if (!is_allowed_access ((pte_val >> 1) & 0x0f, acc_type, self.m_priv)) {
        //         println! ("<Page Access Failed. Allowed Access Failed PTE_VAL=%016lx>", pte_val);
        //         return Err(MemResult::TlbError);
        //     }
        //     if (((pte_val & 0x40) == 0) || // PTE.A
        //         ((acc_type == MemAccType::Write) && (pte_val & 0x80) == 0)) { // PTE.D
        //         println!("<Access Fault : Page Permission Fault {:01x}>", (pte_val >> 1) & 0x0f);
        //         if (acc_type == MemAccType::Fetch) {
        //             generate_exception (self, ExceptCode::InstPageFault, virtual_addr as i64);
        //         }
        //         return Err(MemResult::TlbError);
        //     }
        //     return Ok(paddr);
        // }

        let satp = self.m_csr.csrrs(CsrAddr::Satp, 0) as i64;
        let pte_base = Self::extract_bit_field(satp, 43, 0);

        let mut pte_val: i64 = 0;
        let mut pte_addr: u64 = (pte_base * pagesize as i64) as u64;
        let level: usize = 0;

        for level in range(0, init_level).rev() {
            let va_vpn_i: u64 =
                (virtual_addr >> vpn_idx[level as usize]) & ((1 << vpn_len[level as usize]) - 1);
            pte_addr += (va_vpn_i * (ptesize as u64)) as u64;

            pte_val = self.read_mem_4byte(pte_addr) as i64;

            println!(
                "<Info: VAddr = 0x{:016x} PTEAddr = 0x{:016x} : PPTE = 0x{:08x}>",
                virtual_addr, pte_addr, pte_val
            );

            // 3. If pte:v = 0, or if pte:r = 0 and pte:w = 1, stop and raise a page-fault exception.
            if (pte_val & 0x01) == 0 || (((pte_val & 0x02) == 0) && ((pte_val & 0x04) == 0x04)) {
                // let bit_length: u32 = m_bit_mode == RiscvBitMode_t::Bit32 ? 8 : 16;
                println!("<Page Table Error : 0x{:016x} = 0x{:08x} is not valid Page Table. Generate Exception>",
                         pte_addr, pte_val);

                match acc_type {
                    MemAccType::Fetch => {
                        self.generate_exception(ExceptCode::InstPageFault, virtual_addr as i64);
                    }
                    MemAccType::Read => {
                        self.generate_exception(ExceptCode::LoadPageFault, virtual_addr as i64);
                    }
                    MemAccType::Write => {
                        self.generate_exception(ExceptCode::StorePageFault, virtual_addr as i64);
                    }
                };
                return Err(MemResult::TlbError);
            }

            // If pte:r = 1 or pte:x = 1, go to step 5. Otherwise, this PTE is a
            // pointer to the next level of the page table. Let i = i − 1. If i < 0, stop and raise a page-fault
            // exception. Otherwise, let a = pte:ppn × pagesize and go to step 2.
            if ((pte_val & 0x08) == 0x08) || ((pte_val & 0x02) == 0x02) {
                break;
            } else {
                if level == 0 {
                    println!(
                        "<Access Fault : Tried to Access to Page {:01x}>",
                        ((pte_val >> 1) & 0x0f)
                    );

                    match acc_type {
                        MemAccType::Fetch => {
                            self.generate_exception(ExceptCode::InstPageFault, virtual_addr as i64);
                        }
                        MemAccType::Read => {
                            self.generate_exception(ExceptCode::LoadPageFault, virtual_addr as i64);
                        }
                        MemAccType::Write => {
                            self.generate_exception(ExceptCode::StorePageFault, virtual_addr as i64);
                        }
                    };
                    return Err(MemResult::TlbError);
                }
            }
            let pte_ppn: u64 = Self::extract_bit_field(
                pte_val as i64,
                pte_len[(init_level - 1) as usize] + pte_idx[(init_level - 1) as usize] - 1,
                pte_idx[0],
            ) as u64;
            pte_addr = pte_ppn * (pagesize as u64);
        }

        let current_priv: PrivMode = self.m_priv.clone();
        if !self.is_allowed_access(
            ((pte_val >> 1) & 0x0f) as u8,
            acc_type.clone(),
            current_priv,
        ) {
            println!(
                "<Page Access Failed. Allowed Access Failed PTE_VAL={:016x}>",
                pte_val,
            );
            return Err(MemResult::TlbError);
        }

        if level != 0
            && Self::extract_bit_field(
                pte_val as i64,
                pte_len[level - 1] + pte_idx[level - 1] - 1,
                pte_idx[0],
            ) != 0
        {
            // 6. If i > 0 and pa:ppn[i−1:0] != 0, this is a misaligned superpage
            // stop and raise a page-fault exception.
            // println! ("<Page Access Failed. Last PTE != 0>");
            return Err(MemResult::TlbError);
        }

        if ((pte_val & 0x40) == 0) || // PTE.A
            (is_write_access && (pte_val & 0x80) == 0)
        {
            // PTE.D
            println!(
                "<Access Fault : Page Permission Fault {:01x}",
                ((pte_val >> 1) & 0x0f)
            );

            match acc_type {
                MemAccType::Fetch => {
                    self.generate_exception(ExceptCode::InstPageFault, virtual_addr as i64);
                }
                MemAccType::Read => {
                    self.generate_exception(ExceptCode::LoadPageFault, virtual_addr as i64);
                }
                MemAccType::Write => {
                    self.generate_exception(ExceptCode::StorePageFault, virtual_addr as i64);
                }
            };
            return Err(MemResult::TlbError);
        }

        let mut phy_addr: u64 = (Self::extract_bit_field(
            pte_val as i64,
            pte_len[(init_level - 1) as usize] + pte_idx[(init_level - 1) as usize] - 1,
            pte_idx[level],
        ) << ppn_idx[level]) as u64;

        // println!("Level = {}", level);

        for l in 0..(level + 1) {
            let virtual_addr_vpn: u64 = Self::extract_bit_field(
                virtual_addr as i64,
                vpn_len[level - l as usize] + vpn_idx[level - l as usize] - 1,
                vpn_idx[level - l as usize],
            ) as u64;
            phy_addr |= virtual_addr_vpn << ppn_idx[level as usize];
        }

        // Finally Add Page Offset
        phy_addr |= Self::extract_bit_field(virtual_addr as i64, vpn_idx[0] - 1, 0) as u64;

        //==========================
        // Update Simple TLB Search
        //==========================
        // println!(
        //     "<Info: TLB[{:d}] <= 0x{:016x}(0x{:016x})>",
        //     virtual_addr as i64_tag,
        //     virtual_addr as i64_vpn,
        //     *paddr & !0x0fff
        // );
        // m_tlb_en  [virtual_addr_tag] = true;
        // m_tlb_tag [virtual_addr_tag] = virtual_addr_vpn;
        // m_tlb_addr[virtual_addr_tag] = (*paddr & !0x0fff) | (pte_val & 0x0ff);

        // println!("<Converted Virtual Address = {:08x}>", phy_addr);
        return Ok(phy_addr);
    }

    fn is_allowed_access(&mut self, i_type: u8, acc_type: MemAccType, priv_mode: PrivMode) -> bool {
        let is_user_mode = match priv_mode {
            PrivMode::User => true,
            _ => false,
        };
        if is_user_mode && !((i_type & 0x08) != 0) {
            return false;
        }
        let allowed_access = match acc_type {
            MemAccType::Fetch => (i_type & 0x04) != 0,
            MemAccType::Write => ((i_type & 0x01) != 0) && ((i_type & 0x02) != 0),
            MemAccType::Read => {
                let mstatus: i64 = self.m_csr.csrrs(CsrAddr::Mstatus, 0);
                let mxr: u8 = Self::extract_bit_field(
                    mstatus,
                    riscv_csr_def::SYSREG_MSTATUS_MXR_MSB,
                    riscv_csr_def::SYSREG_MSTATUS_MXR_LSB,
                ) as u8;
                ((i_type & 0x01) != 0) | ((mxr & (i_type & 0x04)) != 0)
            }
        };
        return allowed_access;
    }

    fn get_vm_mode(&mut self) -> VMMode {
        let satp_val = self.m_csr.csrrs(CsrAddr::Satp, 0); // SATP
        let mode = Self::extract_bit_field(satp_val, 63, 60);
        return if self.m_priv == PrivMode::Machine {
            VMMode::Mbare
        } else {
            let v_mode = VMMode::from(mode);
            if v_mode == VMMode::Mbare || v_mode == VMMode::Sv32 ||
                v_mode == VMMode::Sv39 || v_mode == VMMode::Sv48 || v_mode == VMMode::Sv57 || v_mode == VMMode::Sv64 {
                return v_mode
            } else {
                panic!("Error: illegal VM Mode in SATP {:}", mode)
            }
        };
    }
}
