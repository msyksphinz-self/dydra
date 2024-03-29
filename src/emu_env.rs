use mmap::{MapOption, MemoryMap};
// use std::collections::HashMap;
use arr_macro::arr;
// use fnv::FnvHashMap;
use std::mem;
use clap::{Parser, ValueEnum};

use crate::elf_loader::{ELFLoader};
use crate::elf_loader::ProgramHeader;
use crate::elf_loader::SectionHeader;

use crate::target::riscv::riscv::{ExceptCode, PrivMode, TranslateRiscv};
use crate::target::riscv::riscv_csr::{CsrAddr, RiscvCsr};
use crate::target::riscv::riscv_csr_def;
use crate::target::riscv::riscv_decoder::decode_inst;
use crate::target::riscv::riscv_inst_id::RiscvInstId;
use crate::target::riscv::mmu::{MemAccType};
use crate::target::riscv::riscv_disassemble::{disassemble_riscv};

use crate::tcg::tcg::{TCGOp, TCG, TCGOpcode};
use crate::tcg::x86::x86::{TCGX86, X86TargetRM};
use crate::tcg::x86::disassemble::{disassemble_x86};
use crate::instr_info::InstrInfo;

use std::time::{Instant, Duration};

const TCG_HASH_SIZE:usize = 1024;
#[inline]
fn calc_hash_func(addr: u64) -> usize {
    ((addr >> 1) & 0x3ff) as usize
}

const TLB_SIZE: usize = 4096;

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum MachineEnum {
    #[clap(name = "virt")]
    RiscvVirt,
    #[clap(name = "sifive_u")]
    RiscvSiFiveU,
}

#[derive(Parser, Debug, Clone)]
#[clap(
    name = "Hydra",
    author = "msyksphinz",
    version = "0.0.1",
    about = "Binary Translated Instruction Set Emulator"
)]
pub struct ArgConfig {
    #[arg(short = 'd', help = "Debug mode")]
    pub debug: bool, 

    #[arg(long = "dump-gpr", help = "Dump Integer Register by Each Block")]
    pub dump_gpr: bool, 

    #[arg(long = "dump-fpr", help = "Dump Floating Point Register by Each Block")]
    pub dump_fpr: bool, 

    #[arg(long = "dump-tcg", help = "TCG Translation, step execution")]
    pub dump_tcg: bool, 

    #[arg(short = 's', long = "step", help = "TCG Translation, step execution")]
    pub step: bool,

    #[arg(short = 'm', long = "mmu", help = "MMU debug log output")]
    pub mmu_debug: bool,

    #[arg(long = "dump-guest", help = "Dump Guest Instruction")]
    pub dump_guest: bool,

    #[arg(long = "dump-host", help = "Dump Host Instruction")]
    pub dump_host: bool,

    #[arg(value_enum, value_name = "Machine Name", long = "machine", help = "specify machine", required = true)]
    pub machine: MachineEnum,

    #[arg(long = "opt-reg-fwd", help = "Apply optimization : Register Forwarding")]
    pub opt_reg_fwd: bool,

    #[arg(long = "elf-file", required = true)]
    pub elf_file: String,
}

pub struct EmuEnv {
    pub head: [u64; 1], // pointer of this struct. Do not move.

    pub m_priv: PrivMode,

    pub m_iregs: [u64; 32],  // Integer Registers
    pub m_fregs: [u64; 32], // Floating Point Registers
    pub m_pc: [u64; 1],

    pub m_csr: RiscvCsr<i64>, // CSR implementation

    helper_func: [fn(emu: &mut EmuEnv, arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> usize; 59],

    pub m_riscv_trans: TranslateRiscv,

    m_tcg_raw_vec: Vec<u8>,
    m_tcg_tb_vec: Vec<u8>,

    pub m_prologue_epilogue_mem: MemoryMap,
    pub m_guest_mem: MemoryMap,

    pub m_curr_hash_key: usize,
    pub m_tb_text_hash_address: [u64; TCG_HASH_SIZE],
    pub m_tb_text_hash_inst_size: [usize; TCG_HASH_SIZE],
    pub m_tb_text_hash_memmap: [MemoryMap; TCG_HASH_SIZE],

    pub m_host_prologue: [u8; 15],
    pub m_host_epilogue: [u8; 11],

    m_updated_pc : bool,

    pub m_tlb_vec: [u64; TLB_SIZE],
    pub m_tlb_addr_vec: [u64; TLB_SIZE],
    // Configuration
    pub m_arg_config: ArgConfig,

    pub m_notify_exit: bool,

    pub m_gpr_usage_list: [Option<X86TargetRM>; 32],
    pub m_x86reg_usage_list: [Option<u64>; X86TargetRM::SENTINEL as usize],  // Sentinel
    loop_idx: usize,
}

impl EmuEnv {
    pub fn new(arg_config: ArgConfig) -> EmuEnv {
        EmuEnv {
            head: [0xdeadbeef; 1],
            m_priv: PrivMode::Machine,

            m_iregs: [0; 32],
            m_fregs: [0; 32],
            m_pc: [0x8000_0000; 1],
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
                Self::helper_func_fadd_d,
                Self::helper_func_fsub_d,
                Self::helper_func_fmul_d,
                Self::helper_func_fdiv_d,
                Self::helper_func_fmadd_d,
                Self::helper_func_fmsub_d,
                Self::helper_func_fnmsub_d,
                Self::helper_func_fnmadd_d,
                Self::helper_func_fsqrt_d,
                Self::helper_func_feq_d,
                Self::helper_func_flt_d,
                Self::helper_func_fle_d,
                Self::helper_func_fclass_d,
                Self::helper_func_fadd_s,
                Self::helper_func_fsub_s,
                Self::helper_func_fmul_s,
                Self::helper_func_fdiv_s,
                Self::helper_func_fmadd_s,
                Self::helper_func_fmsub_s,
                Self::helper_func_fnmsub_s,
                Self::helper_func_fnmadd_s,
                Self::helper_func_fsqrt_s,
                Self::helper_func_feq_s,
                Self::helper_func_flt_s,
                Self::helper_func_fle_s,
                Self::helper_func_fclass_s,
                Self::helper_func_fmax_d,
                Self::helper_func_fmin_d,
                Self::helper_func_fmax_s,
                Self::helper_func_fmin_s,
                Self::helper_func_fsgnj_s,
                Self::helper_func_fsgnjn_s,
                Self::helper_func_fsgnjx_s,
                Self::helper_func_sret,
                Self::helper_func_load64,
                Self::helper_func_load32,
                Self::helper_func_load16,
                Self::helper_func_load8,
                Self::helper_func_loadu32,
                Self::helper_func_loadu16,
                Self::helper_func_loadu8,
                Self::helper_func_store64,
                Self::helper_func_store32,
                Self::helper_func_store16,
                Self::helper_func_store8,
                Self::helper_func_float_load64,
                Self::helper_func_float_load32,
                Self::helper_func_float_store64,
                Self::helper_func_float_store32,
                Self::helper_func_sfence_vma,
                Self::helper_func_fcvt,
            ],
            m_riscv_trans: TranslateRiscv::new(),

            m_tcg_raw_vec: vec![],
            m_tcg_tb_vec: vec![],
            m_prologue_epilogue_mem: match MemoryMap::new(1, &[]) {
                Ok(m) => m,
                Err(e) => panic!("Error: {}", e),
            },
            // m_tb_text_hashmap: FnvHashMap::with_capacity_and_hasher (0, Default::default()),
            m_tb_text_hash_address: [0; TCG_HASH_SIZE],
            m_tb_text_hash_inst_size: [0; TCG_HASH_SIZE],
            m_tb_text_hash_memmap: arr![MemoryMap::new(0x2000, &[
                MapOption::MapReadable,
                MapOption::MapWritable,
                MapOption::MapExecutable,
            ]).unwrap(); 1024],

            m_guest_mem: match MemoryMap::new(
                0x80000,
                &[
                    MapOption::MapReadable,
                    MapOption::MapWritable,
                    MapOption::MapExecutable,
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
            m_updated_pc: false,

            m_curr_hash_key: 0,
            // TLB format
            m_tlb_vec: [0xdeadbeef_01234567; TLB_SIZE],
            m_tlb_addr_vec: [0x0; TLB_SIZE],
            m_arg_config: arg_config,

            loop_idx: 0,
            m_notify_exit: false,

            m_gpr_usage_list: [None; 32],
            m_x86reg_usage_list: [None; X86TargetRM::SENTINEL as usize],
        }
    }

    // fn dummy_helper(
    //     _emu: &mut EmuEnv,
    //     _dest: u32,
    //     _source: u32,
    //     _csr_addr: u32,
    //     _dummy: u32,
    // ) -> usize {
    //     panic!("Illegal helper function called!");
    // }


    pub fn dump_gpr(&self) {
        let abi_reg_name = ["zero ", "ra   ", "sp   ", "gp   ", "tp   ", "t0   ", "t1   ", "t2   ",
                            "s0/fp", "s1   ", "a0   ", "a1   ", "a2   ", "a3   ", "a4   ", "a5   ", 
                            "a6   ", "a7   ", "s2   ", "s3   ", "s4   ", "s5   ", "s6   ", "s7   ", 
                            "s8   ", "s9   ", "s10  ", "s11  ", "t3   ", "t4   ", "t5   ", "t6   "];
        for (i, reg) in self.m_iregs.iter().enumerate() {
            eprint!("x{:02}({:}) = {:016x}  ", i, abi_reg_name[i], reg);
            if i % 4 == 3 {
                eprint!("\n");
            }
        }
        eprint!("\n");
    }

    pub fn dump_fpr(&self) {
        let abi_reg_name = ["ft0  ", "ft1  ", "ft2  ", "ft3  ", "ft4  ", "ft5  ", "ft6  ", "ft7  ",
                                     "fs0  ", "fs1  ", "fa0  ", "fa1  ", "fa2  ", "fa3  ", "fa4  ", "fa5  ", 
                                     "fa6  ", "fa7  ", "fs2  ", "fs3  ", "fs4  ", "fs5  ", "fs6  ", "fs7  ", 
                                     "fs8  ", "fs9  ", "fs10 ", "fs11 ", "ft8  ", "ft9  ", "ft10 ", "ft11 "];
        for (i, reg) in self.m_fregs.iter().enumerate() {
            eprint!("f{:02}({:}) = {:016x}  ", i, abi_reg_name[i], reg);
            if i % 4 == 3 {
                eprint!("\n");
            }
        }
    }

    pub fn get_gpr(&self) -> [u64; 32] {
        return self.m_iregs;
    }

    pub fn run(&mut self) {
        let loader = match ELFLoader::new(&self.m_arg_config.elf_file) {
            Ok(loader) => loader,
            Err(error) => panic!("There was a problem opening the file: {:?}, {:}", error, &self.m_arg_config.elf_file),
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
            eprintln!("sh_flags = {:}", sh_header.sh_flags);
            if sh_header.sh_flags & 0x7 != 0 && sh_header.sh_type != 8 {   // SectionType = NOBITS => Skip
                sh_header.dump();
                if sh_header.sh_flags & 4 != 0 {
                    // Text section
                    loader.load_section(
                        0x8000_0000,
                        &mut self.m_guest_mem,
                        sh_header.sh_offset,
                        sh_header.sh_addr,
                        sh_header.sh_size,
                    );
                } else {
                    // Data section
                    loader.load_section(
                        0x8000_0000,
                        &mut self.m_guest_mem,
                        sh_header.sh_offset,
                        sh_header.sh_addr,
                        sh_header.sh_size,
                    );
                }
            }
        }

        // Emit Prologue
        for b in &self.m_host_prologue {
            self.m_tcg_raw_vec.push(*b);
        }

        // Emit Epilogue
        for b in &self.m_host_epilogue {
            self.m_tcg_raw_vec.push(*b);
        }

        self.m_prologue_epilogue_mem = {
            let v = self.m_tcg_raw_vec.as_slice();
            Self::reflect(v)
        };

        self.run_loop();
    }

    fn execute_func(&self, tb_text: *mut u8) {
        unsafe {
            let func: unsafe extern "C" fn(emu_head: *const [u64; 1], tb_map: *mut u8) -> u32 =
                mem::transmute(self.m_prologue_epilogue_mem.data());

            let emu_ptr: *const [u64; 1] = &self.head;
            let _ans = func(emu_ptr, tb_text);
            if self.m_arg_config.debug {
                println!("execute return value = {:08x}", _ans);
            }
        }
    }

    fn run_loop (&mut self) {
        let start = Instant::now();
        let loop_max = 10000000;
        self.loop_idx = 5;
        while self.loop_idx < loop_max {
            if self.m_arg_config.debug {
                eprintln!("========= BLOCK START =========");
            }

            assert!(self.m_pc[0] >= 0x8000_0000);
            self.m_curr_hash_key = calc_hash_func(self.m_pc[0]);
            if self.m_arg_config.debug {
                self.decode_and_run();
            } else {
                if self.m_tb_text_hash_address[self.m_curr_hash_key] == self.m_pc[0] {
                    let inst_size = self.m_tb_text_hash_inst_size[self.m_curr_hash_key];
                    // let mem_map = &self.m_tb_text_hash_memmap[hash_key];

                    self.m_pc[0] = self.m_pc[0] + inst_size as u64;
                } else {
                    self.decode_and_run();
                }
            };

            self.execute_func(self.m_tb_text_hash_memmap[self.m_curr_hash_key].data());

            // if self.m_arg_config.dump_gpr {
            //     self.dump_gpr();
            // }
            // if self.m_arg_config.dump_fpr {
            //     self.dump_fpr();
            // }
            // if self.m_arg_config.machine == MachineEnum::RiscvVirt && self.get_mem(0x1000) != 0 {
            //     if self.get_mem(0x1000) & 0x01 == 1 {
            //         break;
            //     }
            //     self.sys_write(self.read_mem_8byte(0x80001000));
            //     self.write_mem_4byte(0x80001000, 0);
            //     self.write_mem_4byte(0x80001040, 1);
            // }
            if self.m_arg_config.machine == MachineEnum::RiscvSiFiveU && self.m_notify_exit {
                break;
            }
            // if self.get_mem(0x3000) != 0 {
            //     if self.get_mem(0x3000) & 0x01 == 1 {
            //         eprintln!("0x3000 finished.");
            //         break;
            //     }
            // }
            // for reg in self.m_x86reg_usage_list.iter_mut() {
            //     *reg = None;
            // }
            // for reg in self.m_gpr_usage_list.iter_mut() {
            //     *reg = None;
            // }
        }
        let end = start.elapsed();
        eprintln!("{}.{:03} finished", end.as_secs(), end.subsec_nanos() / 1_000_000);
    }


    fn sys_write(&mut self, tohost: u64) {
        let _fd = self.read_mem_8byte(tohost + 8);
        let pbuf = self.read_mem_8byte(tohost + 16);
        let len = self.read_mem_8byte(tohost + 24);

        eprintln!("sys_write() = {:x} ,tohost = {:x}", pbuf, tohost);
        for idx in 0..len {
            eprint!("{}", self.read_mem_1byte(pbuf.wrapping_add(idx)) as char);
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
    //     let instructions = &self.m_guest_mem;
    //     let mut inst_32: u32 = 0;
    //     for (idx, inst) in instructions.iter().enumerate() {
    //         inst_32 = inst_32 | (*inst as u32) << (8 * (idx % 4));
    //         if idx % 4 == 3 {
    //             let inst_info = Box::new(InstrInfo {
    //                 inst: inst_32,
    //                 addr: (idx - 3) as u64,
    //             });
    //             self.m_inst_vec.push(*inst_info);
    //             eprint!("{:08x} ", inst_32);
    //             if idx % 32 == 32 - 1 {
    //                 eprint!("\n");
    //             }
    //             inst_32 = 0;
    //         }
    //     }
    //     eprint!("\n");
    // }

    pub fn calc_epilogue_address(&self) -> isize {
        let prologue_epilogue_ptr = self.m_prologue_epilogue_mem.data() as *const u64;
        let tb_ptr = self.m_tb_text_hash_memmap[self.m_curr_hash_key].data() as *const u64;
        let mut diff_from_epilogue = unsafe { prologue_epilogue_ptr.offset_from(tb_ptr) };
        diff_from_epilogue *= 8;
        diff_from_epilogue += self.m_host_prologue.len() as isize;
        diff_from_epilogue
    }

    pub fn calc_gpr_relat_address(&self, gpr_addr: u64) -> isize {
        let guestcode_ptr = self.m_iregs.as_ptr() as *const u8;
        let self_ptr = self.head.as_ptr() as *const u8;
        let mut diff = unsafe { guestcode_ptr.offset_from(self_ptr) };
        diff += gpr_addr as isize * mem::size_of::<u64>() as isize;
        diff
    }

    pub fn calc_fregs_relat_address(&self, gpr_addr: u64) -> isize {
        let guestcode_ptr = self.m_fregs.as_ptr() as *const u8;
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

    pub fn calc_guest_data_mem_address(&self) -> usize {
        let guestcode_ptr = self.m_guest_mem.data();
        return guestcode_ptr as usize;
    }

    pub fn calc_helper_func_relat_address(&self, csr_helper_idx: usize) -> isize {
        let csr_helper_func_ptr =
            unsafe { self.helper_func.as_ptr().offset(csr_helper_idx as isize) as *const u8 };
        let self_ptr = self.head.as_ptr() as *const u8;
        let diff = unsafe { csr_helper_func_ptr.offset_from(self_ptr) };
        diff
    }

    pub fn calc_tlb_relat_address(&self) -> isize {
        let tlb_ptr = self.m_tlb_vec.as_ptr() as *const u8;
        let self_ptr = self.head.as_ptr() as *const u8;
        let diff = unsafe { tlb_ptr.offset_from(self_ptr) };
        // eprintln!("calc_tlb_relat_address tlb_ptr = {:p}, self_ptr = {:p}, diff = {:08x}", tlb_ptr, self_ptr, diff);
        diff
    }

    pub fn calc_tlb_addr_relat_address(&self) -> isize {
        let tlb_ptr = self.m_tlb_addr_vec.as_ptr() as *const u8;
        let self_ptr = self.head.as_ptr() as *const u8;
        let diff = unsafe { tlb_ptr.offset_from(self_ptr) };
        // eprintln!("calc_tlb_vec_relat_address tlb_ptr = {:p}, self_ptr = {:p}, diff = {:08x}", tlb_ptr, self_ptr, diff);
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

    pub fn generate_exception(&mut self, guest_pc: u64, code: ExceptCode, tval: i64) {
        if self.m_arg_config.debug {
            eprintln!(
                "<Info: Generate Exception Code={}, TVAL={:016x} PC={:016x}>",
                code as u32, tval, guest_pc
            );
        }
        let epc = guest_pc;

        let curr_priv: PrivMode = self.m_priv;

        let mut mstatus: i64;
        let mut sstatus: i64;
        let tvec: i64;
        let medeleg = self.m_csr.csrrs(CsrAddr::Medeleg, 0);
        let mut next_priv: PrivMode = PrivMode::Machine;

        self.m_priv = next_priv;

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
            eprint!("tvec = {:016x}\n", tvec);
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
        self.m_priv = next_priv;
        // self.set_pc(tvec as u64);
        // self.set_update_pc(true);
        self.m_pc[0] = tvec as u64;

        if self.m_arg_config.debug {
            eprintln!(
                "<Info: Exception. ChangeMode from {} to {}>",
                curr_priv as u32, next_priv as u32
            );
            eprintln!("<Info: Set Program Counter = 0x{:16x}>", self.m_pc[0]);
        }
        self.m_updated_pc = true;

        return;
    }

    pub fn get_mem(&self, addr: u64) -> u32 {
        let mem = self.m_guest_mem.data();
        return unsafe { mem.offset(addr as isize).read() } as u32;
    }


    pub fn read_mem_1byte(&self, guest_phy_addr: u64) -> u8 {
        assert!(guest_phy_addr >= 0x8000_0000);
        let guest_phy_addr = guest_phy_addr - 0x8000_0000;
        unsafe {
            self.m_guest_mem.data().offset(guest_phy_addr as isize).read() 
        }
    }

    pub fn read_mem_2byte(&self, guest_phy_addr: u64) -> u16 {
        ((self.read_mem_1byte(guest_phy_addr + 1) as u16) << 8) | 
        ((self.read_mem_1byte(guest_phy_addr + 0) as u16) << 0)
    }


    pub fn read_mem_4byte(&self, guest_phy_addr: u64) -> u32 {
        ((self.read_mem_2byte(guest_phy_addr + 2) as u32) << 16) | 
        ((self.read_mem_2byte(guest_phy_addr + 0) as u32) <<  0)
    }


    pub fn read_mem_8byte(&self, guest_phy_addr: u64) -> u64 {
        ((self.read_mem_4byte(guest_phy_addr + 4) as u64) << 32) | 
        ((self.read_mem_4byte(guest_phy_addr + 0) as u64) <<  0)
    }

    pub fn write_mem_1byte(&self, guest_phy_addr: u64, data: u8) {
        assert!(guest_phy_addr >= 0x8000_0000);
        let guest_phy_addr = guest_phy_addr - 0x8000_0000;
        unsafe {
            self.m_guest_mem.data().offset(guest_phy_addr as isize + 0).write(((data >>  0) & 0xff) as u8);
        };
    }

    pub fn write_mem_2byte(&self, guest_phy_addr: u64, data: u16) {
        assert!(guest_phy_addr >= 0x8000_0000);
        let guest_phy_addr = guest_phy_addr - 0x8000_0000;
        unsafe {
            self.m_guest_mem.data().offset(guest_phy_addr as isize + 0).write(((data >>  0) & 0xff) as u8);
            self.m_guest_mem.data().offset(guest_phy_addr as isize + 1).write(((data >>  8) & 0xff) as u8);
        };
    }


    pub fn write_mem_4byte(&self, guest_phy_addr: u64, data: u32) {
        assert!(guest_phy_addr >= 0x8000_0000);
        let guest_phy_addr = guest_phy_addr - 0x8000_0000;
        unsafe {
            self.m_guest_mem.data().offset(guest_phy_addr as isize + 0).write(((data >>  0) & 0xff) as u8);
            self.m_guest_mem.data().offset(guest_phy_addr as isize + 1).write(((data >>  8) & 0xff) as u8);
            self.m_guest_mem.data().offset(guest_phy_addr as isize + 2).write(((data >> 16) & 0xff) as u8);
            self.m_guest_mem.data().offset(guest_phy_addr as isize + 3).write(((data >> 24) & 0xff) as u8);
        };
    }

    pub fn write_mem_8byte(&self, guest_phy_addr: u64, data: u64) {
        let data0 = data & 0xffff_ffff;
        let data1 = (data >> 32) & 0xffff_ffff;
        self.write_mem_4byte(guest_phy_addr + 0, data0 as u32);
        self.write_mem_4byte(guest_phy_addr + 4, data1 as u32);
    }

    fn decode_and_run(&mut self) {
        let mut tcg_vec = vec![];
        if self.m_arg_config.debug {
            eprint!("{:}: Guest PC Address = {:08x}\n", self.loop_idx, self.m_pc[0]);
        }

        let mut total_inst_byte = 0;
        let init_pc = self.m_pc[0];
        #[allow(while_true)]
        while true {
            self.loop_idx += 1;
            #[allow(unused_assignments)]
            let mut guest_phy_addr = 0;
            match self.convert_physical_address(self.m_pc[0], self.m_pc[0], MemAccType::Fetch) {
                Ok(addr) => guest_phy_addr = addr,
                Err(_error) => {
                    continue;
                }
            };
            if self.m_arg_config.mmu_debug {
                eprint!("  converted physical address = {:08x}\n", guest_phy_addr);
            }
            let guest_inst = self.read_mem_4byte(guest_phy_addr);
        
            let (id, inst_byte) = match decode_inst(guest_inst) {
                Some((id, inst_byte)) => (id, inst_byte),
                _ => panic!("Decode Failed. {:08x}", guest_inst),
            };
            let inst_info = InstrInfo {
                inst: guest_inst,
                addr: self.m_pc[0],
            };
            let mut tcg_inst = self.m_riscv_trans.translate(id, &inst_info);
            assert_eq!(self.m_riscv_trans.reg_bitmap.len(), 5);

            tcg_vec.append(&mut tcg_inst);
            if self.m_arg_config.step {
                let mut exit_tcg = vec![TCGOp::new_0op(TCGOpcode::EXIT_TB, None)];
                tcg_vec.append(&mut exit_tcg);
            }
            if self.m_arg_config.dump_guest {
                eprint!(" {:016x}:{:016x} Hostcode {:08x} : {}\n",  self.m_pc[0], guest_phy_addr, inst_info.inst, disassemble_riscv(guest_inst));
            }
            total_inst_byte += inst_byte;

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
                || id == RiscvInstId::SRET
                || id == RiscvInstId::C_J
                || id == RiscvInstId::C_JAL
                || id == RiscvInstId::C_JALR
                || id == RiscvInstId::C_JR
            {
                break;
            }
            self.m_pc[0] = self.m_pc[0] + inst_byte as u64;

            if id == RiscvInstId::FENCE_I {
                break;
            }
        
            if self.m_arg_config.step {
                break;      // When self.m_arg_config.debug Mode, break for each instruction
            }
        }
        
        let mut pc_address = 0;
        
        self.m_tcg_tb_vec.clear();

        for tcg in tcg_vec.iter() {
            if self.m_arg_config.dump_tcg {
                eprintln!("tcg_inst = {:?}", &tcg);
            }
        
            let mut mc_byte = vec![];
            TCGX86::tcg_gen(self, pc_address, tcg, &mut mc_byte);
            for be in &mc_byte {
                let be_data = *be;
                self.m_tcg_tb_vec.push(be_data);
            }
            pc_address += mc_byte.len() as u64;
        }
        
        let hash_key = calc_hash_func(init_pc);
        unsafe {
            std::ptr::copy(
                self.m_tcg_tb_vec.as_ptr(),
                self.m_tb_text_hash_memmap[hash_key].data(),
                self.m_tcg_tb_vec.len(),
            );
        }
        self.m_tb_text_hash_address[hash_key] = init_pc;
        self.m_tb_text_hash_inst_size[hash_key] = total_inst_byte;

        for tcg in tcg_vec.iter_mut() {
            match tcg.op {
                Some(_) => {}
                None => {
                    if self.m_arg_config.debug {
                        eprintln!("label found 2");
                    }
                    match &tcg.label {
                        Some(l) => {
                            let l = &mut *l.borrow_mut();
                            if self.m_arg_config.debug {
                                eprintln!("label found. offset = {:x}", l.offset);
                            }
                            for v_off in &l.code_ptr_vec {
                                let diff = l.offset as usize - v_off - 4;
                                if self.m_arg_config.debug {
                                    eprintln!(
                                        "replacement target is {:x}, data = {:x}",
                                        v_off, diff
                                    );
                                }
                                let s = self.m_tb_text_hash_memmap[hash_key].data();
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
        if self.m_arg_config.dump_host {
            unsafe {
                std::ptr::copy(
                    self.m_tb_text_hash_memmap[hash_key].data(),
                    self.m_tcg_tb_vec.as_mut_ptr(),
                    self.m_tcg_tb_vec.len(),
                );
            }
    
            disassemble_x86(self.m_tcg_tb_vec.as_slice(), self.m_tb_text_hash_memmap[hash_key].data());
        }
    }

}
