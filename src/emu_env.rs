use mmap::{MapOption, MemoryMap};
use std::mem;

use crate::elf_loader::ELFLoader;
use crate::elf_loader::ProgramHeader;
use crate::elf_loader::SectionHeader;

use crate::riscv::{ExceptCode, PrivMode, TranslateRiscv};
use crate::riscv_csr::{CsrAddr, RiscvCsr};
use crate::riscv_csr_def;
use crate::riscv_decoder::decode_inst;
use crate::riscv_inst_id::RiscvInstId;

use crate::x86::TCGX86;

use crate::tcg::{TCGOp, TCG};

use crate::instr_info::InstrInfo;

pub struct EmuEnv {
    pub head: [u64; 1], // pointer of this struct. Do not move.

    m_regs: [u64; 32],
    m_pc: [u64; 1],

    m_csr: RiscvCsr<i64>, // CSR implementation

    helper_func: [fn(emu: &mut EmuEnv, dest: u32, source: u32, csr_addr: u32) -> usize; 16],

    // m_inst_vec: Vec<InstrInfo>,
    // m_tcg_vec: Vec<Box<tcg::TCGOp>>,
    m_tcg_vec: Vec<TCGOp>,
    m_tcg_raw_vec: Vec<u8>,
    m_tcg_tb_vec: Vec<u8>,

    pub m_prologue_epilogue_mem: MemoryMap,
    pub m_guest_text_mem: MemoryMap,
    pub m_guest_data_mem: MemoryMap,

    pub m_tb_text_mem: MemoryMap,

    pub m_host_prologue: [u8; 15],
    pub m_host_epilogue: [u8; 11],
}

impl EmuEnv {
    pub fn new() -> EmuEnv {
        EmuEnv {
            head: [0xdeadbeef; 1],
            m_regs: [0; 32],
            m_pc: [0x0; 1],
            m_csr: RiscvCsr::new(),

            helper_func: [
                Self::helper_func_csrrw,
                Self::helper_func_csrrs,
                Self::helper_func_csrrc,
                Self::helper_func_csrrwi,
                Self::helper_func_csrrsi,
                Self::helper_func_csrrci,
                Self::helper_func_mret,
                Self::helper_func_ecall,
                Self::dummy_helper,
                Self::dummy_helper,
                Self::dummy_helper,
                Self::dummy_helper,
                Self::dummy_helper,
                Self::dummy_helper,
                Self::dummy_helper,
                Self::dummy_helper,
            ],
            // m_inst_vec: vec![],

            m_tcg_vec: vec![],
            m_tcg_raw_vec: vec![],
            m_tcg_tb_vec: vec![],
            m_prologue_epilogue_mem: match MemoryMap::new(1, &[]) {
                Ok(m) => m,
                Err(e) => panic!("Error: {}", e),
            },
            m_tb_text_mem: match MemoryMap::new(1, &[]) {
                Ok(m) => m,
                Err(e) => panic!("Error: {}", e),
            },
            m_guest_text_mem: match MemoryMap::new(
                0x8000,
                &[
                    MapOption::MapReadable,
                    MapOption::MapWritable,
                    MapOption::MapExecutable,
                ],
            ) {
                Ok(m) => m,
                Err(e) => panic!("Error: {}", e),
            },
            m_guest_data_mem: match MemoryMap::new(
                0x4000,
                &[
                    MapOption::MapReadable,
                    MapOption::MapWritable,
                ],    
            ) {
                Ok(m) => m,
                Err(e) => panic!("Error: {}", e),
            },

            m_host_prologue: [
                0x55, // pushq %rbp
                0x54, // pushq %rsp
                0x51, // pushq %rcx
                0x48, 0x8b, 0xef, // movq     %rdi, %rbp
                0x48, 0x81, 0xc4, 0x80, 0xfb, 0xff, 0xff, // addq     $-0x488, %rsp
                0xff, 0xe6, //  jmpq     *%rsi
            ],
            m_host_epilogue: [
                0x48, 0x81, 0xc4, 0x80, 0x04, 0x00, 0x00, // addq     $0x488, %rsp
                0x59, // popq     %rcx
                0x5b, // popq     %rbx
                0x5d, // popq     %rbp
                0xc3, // retq
            ],
        }
    }

    fn dummy_helper(_emu: &mut EmuEnv, _dest: u32, _source: u32, _csr_addr: u32) -> usize {
        panic!("Illegal helper function called!");
    }

    fn helper_func_csrrw(emu: &mut EmuEnv, dest: u32, source: u32, csr_addr: u32) -> usize {
        println!(
            "helper_csrrw(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, source, csr_addr
        );
        let data = emu.m_regs[source as usize];
        let reg_data = emu
            .m_csr
            .csrrw(CsrAddr::from_u64(csr_addr as u64), data as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        emu.dump_gpr();
        return 0;
    }

    fn helper_func_csrrs(emu: &mut EmuEnv, dest: u32, source: u32, csr_addr: u32) -> usize {
        println!(
            "helper_csrrs(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, source, csr_addr
        );
        let data = emu.m_regs[source as usize];
        let reg_data = emu
            .m_csr
            .csrrs(CsrAddr::from_u64(csr_addr as u64), data as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        emu.dump_gpr();
        return 0;
    }

    fn helper_func_csrrc(emu: &mut EmuEnv, dest: u32, source: u32, csr_addr: u32) -> usize {
        println!(
            "helper_csrrc(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, source, csr_addr
        );
        let data = emu.m_regs[source as usize];
        let reg_data = emu
            .m_csr
            .csrrc(CsrAddr::from_u64(csr_addr as u64), data as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        emu.dump_gpr();
        return 0;
    }

    fn helper_func_csrrwi(emu: &mut EmuEnv, dest: u32, imm: u32, csr_addr: u32) -> usize {
        println!(
            "helper_csrrw(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, imm, csr_addr
        );
        let reg_data = emu
            .m_csr
            .csrrw(CsrAddr::from_u64(csr_addr as u64), imm as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        emu.dump_gpr();
        return 0;
    }

    fn helper_func_csrrsi(emu: &mut EmuEnv, dest: u32, imm: u32, csr_addr: u32) -> usize {
        println!(
            "helper_csrrs(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, imm, csr_addr
        );
        let reg_data = emu
            .m_csr
            .csrrs(CsrAddr::from_u64(csr_addr as u64), imm as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        emu.dump_gpr();
        return 0;
    }

    fn helper_func_csrrci(emu: &mut EmuEnv, dest: u32, imm: u32, csr_addr: u32) -> usize {
        println!(
            "helper_csrrc(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, imm, csr_addr
        );
        let reg_data = emu
            .m_csr
            .csrrc(CsrAddr::from_u64(csr_addr as u64), imm as i64);
        emu.m_regs[dest as usize] = reg_data as u64;
        emu.dump_gpr();
        return 0;
    }

    fn helper_func_mret(emu: &mut EmuEnv, dest: u32, imm: u32, csr_addr: u32) -> usize {
        println!(
            "helper_mret(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, imm, csr_addr
        );
        emu.m_pc[0] = emu.m_csr.csrrc(CsrAddr::Mepc, 0 as i64) as u64;
        print!("PC is set to 0x{:08x}\n", emu.m_pc[0]);
        return 0;
    }

    fn helper_func_ecall(emu: &mut EmuEnv, dest: u32, imm: u32, csr_addr: u32) -> usize {
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

    pub fn dump_gpr(&self) {
        for (i, reg) in self.m_regs.iter().enumerate() {
            print!("x{:02} = {:016x}  ", i, reg);
            if i % 4 == 3 {
                print!("\n");
            }
        }
        print!("PC = {:016x}\n", self.m_pc[0]);
    }

    pub fn get_gpr(&self) -> [u64; 32] {
        return self.m_regs;
    }

    pub fn run(&mut self, filename: &String) {
        let loader = match ELFLoader::new(filename) {
            Ok(loader) => loader,
            Err(error) => panic!("There was a problem opening the file: {:?}", error),
        };

        let elf_header = loader.get_elf_header();
        elf_header.dump();

        let mut ph_headers = Vec::new();
        for ph_idx in 0..elf_header.e_phnum {
            let phdr: ProgramHeader = loader.get_program_header(
                elf_header.e_phoff,
                elf_header.e_phentsize,
                ph_idx.into(),
            );
            ph_headers.push(phdr);
        }

        let mut sh_headers = Vec::new();
        for sh_idx in 0..elf_header.e_shnum {
            let shdr: SectionHeader = loader.get_section_header(
                elf_header.e_shoff,
                elf_header.e_shentsize,
                sh_idx.into(),
            );
            sh_headers.push(shdr);
        }

        for sh_header in sh_headers {
            println!("sh_flags = {:}", sh_header.sh_flags);
            if sh_header.sh_flags != 0 {
                sh_header.dump();
                if sh_header.sh_flags & 4 != 0 {
                    // Text section
                    loader.load_section(
                        0x8000_0000,
                        &mut self.m_guest_text_mem,
                        sh_header.sh_offset,
                        sh_header.sh_addr,
                        sh_header.sh_size,
                    );
                } else {
                    // Data section
                    loader.load_section(
                        0x8000_0000,
                        &mut self.m_guest_data_mem,
                        sh_header.sh_offset,
                        sh_header.sh_addr,
                        sh_header.sh_size,
                    );
                }
            }
        }

        // unsafe {
        //     self.gen_tcg();
        // }

        for _loop_idx in 0..100 {
            // for inst in &self.m_inst_vec {
            //     if inst.addr == self.m_pc[0] {
            //         break;
            //     }
            //     start_idx += 1;
            // }
            // self.m_tcg_vec.clear();
            // print!(
            //     "start_idx = {}. m_inst_vec.len = {}\n",
            //     start_idx,
            //     &self.m_inst_vec.len(),
            // );
            // let mut inst_idx = 0;
            let mut guest_pc = self.m_pc[0];
            
            // for inst in &self.m_inst_vec {
            self.m_tcg_vec.clear();
            #[allow(while_true)]
            while true {
                let guest_inst = unsafe { 
                    ((self.m_guest_text_mem.data().offset(guest_pc as isize + 0).read() as u32) <<  0) |
                    ((self.m_guest_text_mem.data().offset(guest_pc as isize + 1).read() as u32) <<  8) |
                    ((self.m_guest_text_mem.data().offset(guest_pc as isize + 2).read() as u32) << 16) |
                    ((self.m_guest_text_mem.data().offset(guest_pc as isize + 3).read() as u32) << 24) 
                };
                let id = match decode_inst(guest_inst) {
                    Some(id) => id,
                    _ => panic!("Decode Failed"),
                };
                let inst_info = InstrInfo {inst:guest_inst, addr:guest_pc};
                let mut tcg_inst = TranslateRiscv::translate(id, &inst_info);
                self.m_tcg_vec.append(&mut tcg_inst);
                print!("Address = {:08x} : {:08x}\n", inst_info.addr, inst_info.inst);
                if id == RiscvInstId::JALR
                    || id == RiscvInstId::JAL
                    || id == RiscvInstId::BEQ
                    || id == RiscvInstId::BNE
                    || id == RiscvInstId::BGE
                    || id == RiscvInstId::BGEU
                    || id == RiscvInstId::BLT
                    || id == RiscvInstId::BLTU
                    || id == RiscvInstId::ECALL
                    || id == RiscvInstId::MRET
                {
                    break;
                }
                guest_pc += 4;
            }

            // Emit Prologue
            for b in &self.m_host_prologue {
                self.m_tcg_raw_vec.push(*b);
            }

            // Emit Epilogue
            for b in &self.m_host_epilogue {
                self.m_tcg_raw_vec.push(*b);
            }

            // {
            //     for (i, b) in self.m_tcg_raw_vec.iter().enumerate() {
            //         print!("{:02x} ", b);
            //         if i % 16 == 15 {
            //             print!("\n");
            //         }
            //     }
            //     print!("\n");
            // }

            self.m_prologue_epilogue_mem = {
                let v = self.m_tcg_raw_vec.as_slice();
                Self::reflect(v)
            };

            // Make tb instruction region (temporary 1024byte)
            self.m_tb_text_mem = match MemoryMap::new(
                0x4000,
                &[
                    MapOption::MapReadable,
                    MapOption::MapWritable,
                    MapOption::MapExecutable,
                ],
            ) {
                Ok(m) => m,
                Err(e) => panic!("Error: {}", e),
            };

            let mut pc_address = 0;

            let tb_map_ptr = self.m_tb_text_mem.data() as *const u64;
            let pe_map_ptr = self.m_prologue_epilogue_mem.data() as *const u64;
            // let host_cod_ptr = self.m_guest_text_mem.as_ptr();

            println!("tb_address  = {:?}", tb_map_ptr);
            println!("pe_address  = {:?}", pe_map_ptr);
            //println!("self.m_guest_text_mem = {:?}", host_cod_ptr);

            self.m_tcg_tb_vec.clear();
            for tcg in &self.m_tcg_vec {
                println!("tcg_inst = {:?}", &tcg);

                let mut mc_byte = vec![];
                TCGX86::tcg_gen(&self, pc_address, tcg, &mut mc_byte);
                for be in &mc_byte {
                    let be_data = *be;
                    self.m_tcg_tb_vec.push(be_data);
                }
                pc_address += mc_byte.len() as u64;
            }

            unsafe {
                std::ptr::copy(
                    self.m_tcg_tb_vec.as_ptr(),
                    self.m_tb_text_mem.data(),
                    self.m_tcg_tb_vec.len(),
                );
            }

            for tcg in &self.m_tcg_vec {
                match tcg.op {
                    Some(_) => {}
                    None => {
                        println!("label found 2");
                        match &tcg.label {
                            Some(l) => {
                                let l = &mut *l.borrow_mut();
                                println!("label found. offset = {:x}", l.offset);
                                for v_off in &l.code_ptr_vec {
                                    let diff = l.offset as usize - v_off - 4;
                                    println!(
                                        "replacement target is {:x}, data = {:x}",
                                        v_off, diff
                                    );
                                    let s = self.m_tb_text_mem.data();
                                    unsafe {
                                        *s.offset(*v_off as isize) = (diff & 0xff) as u8;
                                    };
                                }
                            }
                            None => {}
                        }
                    }
                }
            }

            let s = self.m_tb_text_mem.data();
            for byte_idx in 0..256 {
                if byte_idx % 16 == 0 {
                    print!("{:08x} : ", byte_idx);
                }
                unsafe {
                    print!("{:02x} ", *s.offset(byte_idx as isize) as u8);
                }
                if byte_idx % 16 == 15 {
                    print!("\n");
                }
            }

            let emu_ptr: *const [u64; 1] = &self.head;

            unsafe {
                let func: unsafe extern "C" fn(emu_head: *const [u64; 1], tb_map: *mut u8) -> u32 =
                    mem::transmute(self.m_prologue_epilogue_mem.data());

                let tb_host_data = self.m_tb_text_mem.data();
                println!("reflect tb address = {:p}", tb_host_data);

                let ans = func(emu_ptr, tb_host_data);
                println!("ans = 0x{:x}", ans);
            }
            self.dump_gpr();
        }
    }

    fn reflect(prologue_epilogue: &[u8]) -> mmap::MemoryMap {
        let pe_map = match MemoryMap::new(
            prologue_epilogue.len(),
            &[
                // MapOption::MapAddr(0 as *mut u8),
                // MapOption::MapOffset(0),
                // MapOption::MapFd(fd),
                MapOption::MapReadable,
                MapOption::MapWritable,
                MapOption::MapExecutable,
                // MapOption::MapNonStandardFlags(libc::MAP_ANON),
                // MapOption::MapNonStandardFlags(libc::MAP_PRIVATE),
            ],
        ) {
            Ok(m) => m,
            Err(e) => panic!("Error: {}", e),
        };

        unsafe {
            std::ptr::copy(
                prologue_epilogue.as_ptr(),
                pe_map.data(),
                prologue_epilogue.len(),
            );
        }

        return pe_map;
    }

    // unsafe fn gen_tcg(&mut self) {
    //     let instructions = &self.m_guest_text_mem;
    //     let mut inst_32: u32 = 0;
    //     for (idx, inst) in instructions.iter().enumerate() {
    //         inst_32 = inst_32 | (*inst as u32) << (8 * (idx % 4));
    //         if idx % 4 == 3 {
    //             let inst_info = Box::new(InstrInfo {
    //                 inst: inst_32,
    //                 addr: (idx - 3) as u64,
    //             });
    //             self.m_inst_vec.push(*inst_info);
    //             print!("{:08x} ", inst_32);
    //             if idx % 32 == 32 - 1 {
    //                 print!("\n");
    //             }
    //             inst_32 = 0;
    //         }
    //     }
    //     print!("\n");
    // }

    pub fn calc_epilogue_address(&self) -> isize {
        let prologue_epilogue_ptr = self.m_prologue_epilogue_mem.data() as *const u64;
        let tb_ptr = self.m_tb_text_mem.data() as *const u64;
        let mut diff_from_epilogue = unsafe { prologue_epilogue_ptr.offset_from(tb_ptr) };
        diff_from_epilogue *= 8;
        diff_from_epilogue += self.m_host_prologue.len() as isize;
        diff_from_epilogue
    }

    pub fn calc_gpr_relat_address(&self, gpr_addr: u64) -> isize {
        let guestcode_ptr = self.m_regs.as_ptr() as *const u8;
        let self_ptr = self.head.as_ptr() as *const u8;
        let mut diff = unsafe { guestcode_ptr.offset_from(self_ptr) };
        diff += gpr_addr as isize * mem::size_of::<u64>() as isize;
        diff
    }

    pub fn calc_pc_address(&self) -> isize {
        let guestcode_ptr = self.m_pc.as_ptr() as *const u8;
        let self_ptr = self.head.as_ptr() as *const u8;
        let diff = unsafe { guestcode_ptr.offset_from(self_ptr) };
        diff
    }

    pub fn calc_guestcode_address(&self) -> usize {
        let guestcode_ptr = self.m_guest_text_mem.data();
        println!("guestcode_ptr = {:p}", guestcode_ptr);
        return guestcode_ptr as usize;
    }

    pub fn calc_helper_func_relat_address(&self, csr_helper_idx: usize) -> isize {
        let csr_helper_func_ptr =
            unsafe { self.helper_func.as_ptr().offset(csr_helper_idx as isize) as *const u8 };
        let self_ptr = self.head.as_ptr() as *const u8;
        let diff = unsafe { csr_helper_func_ptr.offset_from(self_ptr) };
        diff
    }

    pub fn extract_bit_field(hex: i64, left: u8, right: u8) -> i64 {
        let mask: i64 = (1 << (left - right + 1)) - 1;
        return (hex >> right) & mask;
    }

    pub fn set_bit_field(hex: i64, val: i64, left: u8, right: u8) -> i64 {
        let mask: i64 = (1 << (left - right + 1)) - 1;
        return (hex & !(mask << right)) | (val << right);
    }

    fn generate_exception(&mut self, code: ExceptCode, tval: i64) {
        println!(
            "<Info: Generate Exception Code={}, TVAL={:016x} PC={:016x}>",
            code as u32, tval, self.m_pc[0]
        );

        let epc: u64;
        epc = self.m_pc[0];

        // let curr_priv: PrivMode = self.m_priv;
        let curr_priv = PrivMode::Machine;

        let mut mstatus: i64;
        let mut sstatus: i64;
        let tvec: i64;
        let medeleg = self.m_csr.csrrs(CsrAddr::Medeleg, 0);
        let mut next_priv: PrivMode = PrivMode::Machine;

        // self.set_priv_mode(next_priv);

        if (medeleg & (1 << (code as u32))) != 0 {
            // Delegation
            self.m_csr.csrrw(CsrAddr::Sepc, epc as i64);
            self.m_csr.csrrw(CsrAddr::Scause, code as i64);
            self.m_csr.csrrw(CsrAddr::Stval, tval as i64);

            tvec = self.m_csr.csrrs(CsrAddr::Stvec, 0 as i64);
            next_priv = PrivMode::Supervisor;
        } else {
            self.m_csr.csrrw(CsrAddr::Mepc, epc as i64);
            self.m_csr.csrrw(CsrAddr::Mcause, code as i64);
            self.m_csr.csrrw(CsrAddr::Mtval, tval as i64);

            tvec = self.m_csr.csrrs(CsrAddr::Mtvec, 0 as i64);
            print!("tvec = {:016x}\n", tvec);
        }

        // Update status CSR
        if (medeleg & (1 << (code as u32))) != 0 {
            // Delegation
            sstatus = self.m_csr.csrrs(CsrAddr::Sstatus, 0 as i64);
            sstatus = Self::set_bit_field(
                sstatus,
                Self::extract_bit_field(
                    sstatus,
                    riscv_csr_def::SYSREG_SSTATUS_SIE_MSB,
                    riscv_csr_def::SYSREG_SSTATUS_SIE_LSB,
                ),
                riscv_csr_def::SYSREG_SSTATUS_SPIE_MSB,
                riscv_csr_def::SYSREG_SSTATUS_SPIE_LSB,
            );
            sstatus = Self::set_bit_field(
                sstatus,
                curr_priv as i64,
                riscv_csr_def::SYSREG_SSTATUS_SPP_MSB,
                riscv_csr_def::SYSREG_SSTATUS_SPP_LSB,
            );
            sstatus = Self::set_bit_field(
                sstatus,
                0,
                riscv_csr_def::SYSREG_SSTATUS_SIE_MSB,
                riscv_csr_def::SYSREG_SSTATUS_SIE_LSB,
            );
            self.m_csr.csrrw(CsrAddr::Sstatus, sstatus as i64);
        } else {
            mstatus = self.m_csr.csrrs(CsrAddr::Mstatus, 0);
            mstatus = Self::set_bit_field(
                mstatus,
                Self::extract_bit_field(
                    mstatus,
                    riscv_csr_def::SYSREG_MSTATUS_MIE_MSB,
                    riscv_csr_def::SYSREG_MSTATUS_MIE_LSB,
                ),
                riscv_csr_def::SYSREG_MSTATUS_MPIE_MSB,
                riscv_csr_def::SYSREG_MSTATUS_MPIE_LSB,
            );
            mstatus = Self::set_bit_field(
                mstatus,
                curr_priv as i64,
                riscv_csr_def::SYSREG_MSTATUS_MPP_MSB,
                riscv_csr_def::SYSREG_MSTATUS_MPP_LSB,
            );
            mstatus = Self::set_bit_field(
                mstatus,
                0,
                riscv_csr_def::SYSREG_MSTATUS_MIE_MSB,
                riscv_csr_def::SYSREG_MSTATUS_MIE_LSB,
            );

            self.m_csr.csrrw(CsrAddr::Mstatus, mstatus);
        }

        // self.set_priv_mode(next_priv);
        //
        // self.set_pc(tvec as u64);
        // self.set_update_pc(true);
        self.m_pc[0] = tvec as u64;

        println!(
            "<Info: Exception. ChangeMode from {} to {}>",
            curr_priv as u32, next_priv as u32
        );
        println!("<Info: Set Program Counter = 0x{:16x}>", tvec);

        return;
    }

    pub fn get_mem(&self, addr: u64) -> u32 {
        let mem = self.m_guest_text_mem.data();
        return unsafe { mem.offset(addr as isize).read() } as u32 ;
    }
}
