extern crate mmap;
use mmap::{MapOption, MemoryMap};
use std::env;
use std::mem;

pub mod elf_loader;
pub mod riscv;
pub mod riscv_decoder;
pub mod riscv_inst_id;
pub mod tcg;
pub mod x86;

use elf_loader::ELFLoader;
use elf_loader::ProgramHeader;
use elf_loader::SectionHeader;

use riscv_inst_id::RiscvInstId;

use riscv_decoder::decode_inst;

use riscv::TranslateRiscv;
use tcg::*;
use x86::TCGX86;

struct CPU {
    m_regs: [u64; 32],
    m_pc: u64,

    m_inst_vec: Vec<u32>,
    // m_tcg_vec: Vec<Box<tcg::TCGOp>>,
    m_tcg_vec: Vec<tcg::TCGOp>,
    m_tcg_raw_vec: Vec<u8>,
    m_tcg_tb_vec: Vec<u8>,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            m_regs: [0; 32],
            m_pc: 0x0,
            m_inst_vec: vec![],
            m_tcg_vec: vec![],
            m_tcg_raw_vec: vec![],
            m_tcg_tb_vec: vec![],
        }
    }

    fn dump_gpr(self) {
        for (i, reg) in self.m_regs.iter().enumerate() {
            print!("x{:02} = {:016x}  ", i, reg);
            if i % 4 == 3 {
                print!("\n");
            }
        }
        print!("PC = {:016x}\n", self.m_pc);
    }

    pub fn run(mut self, filename: &String) {
        let loader = match ELFLoader::new(filename) {
            Ok(loader) => loader,
            Err(error) => panic!("There was a problem opening the file: {:?}", error),
        };

        // let elf_header = loader.get_elf_header();

        let elf_header = loader.get_elf_header();
        elf_header.dump();

        let mut ph_headers = Vec::new();
        for ph_idx in 0..elf_header.e_phnum {
            let phdr: ProgramHeader = loader.get_program_header(
                &elf_header,
                elf_header.e_phoff,
                elf_header.e_phentsize,
                ph_idx.into(),
            );
            ph_headers.push(phdr);
        }

        let mut sh_headers = Vec::new();
        for sh_idx in 0..elf_header.e_shnum {
            let shdr: SectionHeader = loader.get_section_header(
                &elf_header,
                elf_header.e_shoff,
                elf_header.e_shentsize,
                sh_idx.into(),
            );
            sh_headers.push(shdr);
        }

        let mut riscv_guestcode: Vec<u8> = Vec::new();

        // Dump All Section Headers
        for sh_header in sh_headers {
            if sh_header.sh_flags == 6 {
                sh_header.dump();
                loader.load_section(&mut riscv_guestcode, sh_header.sh_offset, sh_header.sh_size);
            }
        }

        // let riscv_guestcode: [u8; 8] = [
        //     0x13, 0x05, 0x40, 0x06, // addi a0,zero,100
        //     0x67, 0x80, 0x00, 0x00, // ret
        // ];
        let host_prologue = [
            0x55, // pushq %rbp
            0x54, // pushq %rsp
            0x48, 0x8b, 0xef, // movq     %rdi, %rbp
            0x48, 0x81, 0xc4, 0x80, 0xfb, 0xff, 0xff, // addq     $-0x488, %rsp
            0x48, 0x89, 0x75, 0x00, //  mov    %rsi,0x0(%rbp)
            0x48, 0x8b, 0x06, // mov    (%rsi),%rax
            0x48, 0x89, 0x45, 0x08, //  mov    %rax,0x8(%rbp)
            0x48, 0x8b, 0x06, // mov    (%rsi),%rax
            0x48, 0x89, 0x65, 0x10, //  mov    %rsp,0x10(%rbp)
            0xff, 0xe6, //  jmpq     *%rsi
        ];
        let host_epilogue = [
            0x48, 0x81, 0xc4, 0x80, 0x04, 0x00, 0x00, // addq     $0x488, %rsp
            0x5b, // popq     %rbx
            0x5d, // popq     %rbp
            0xc3, // retq
        ];

        unsafe {
            self.gen_tcg(&riscv_guestcode);
        }

        for inst in &self.m_inst_vec {
            let riscv_id = match decode_inst(*inst) {
                Some(id) => id,
                _ => panic!("Decode Failed"),
            };

            let mut tcg_inst = match riscv_id {
                RiscvInstId::ADDI => TranslateRiscv::translate_addi(inst),
                RiscvInstId::ADD => TranslateRiscv::translate_add(inst),
                RiscvInstId::SUB => TranslateRiscv::translate_sub(inst),
                RiscvInstId::AND => TranslateRiscv::translate_and(inst),
                RiscvInstId::OR => TranslateRiscv::translate_or(inst),
                RiscvInstId::XOR => TranslateRiscv::translate_xor(inst),
                RiscvInstId::ANDI => TranslateRiscv::translate_andi(inst),
                RiscvInstId::ORI => TranslateRiscv::translate_ori(inst),
                RiscvInstId::XORI => TranslateRiscv::translate_xori(inst),
                RiscvInstId::JALR => TranslateRiscv::translate_jalr(inst),
                RiscvInstId::LUI => TranslateRiscv::translate_lui(inst),
                RiscvInstId::BEQ => TranslateRiscv::translate_beq(inst),
                other_id => panic!("InstID={:?} : Not supported these instructions.", other_id),
            };

            self.m_tcg_vec.append(&mut tcg_inst);
        }

        // Emit Prologue
        for b in &host_prologue {
            self.m_tcg_raw_vec.push(*b);
        }

        // Emit Epilogue
        for b in &host_epilogue {
            self.m_tcg_raw_vec.push(*b);
        }

        {
            for (i, b) in self.m_tcg_raw_vec.iter().enumerate() {
                print!("{:02x} ", b);
                if i % 16 == 15 {
                    print!("\n");
                }
            }
            print!("\n");
        }

        let pe_map = {
            let v = self.m_tcg_raw_vec.as_slice();
            Self::reflect(v)
        };

        // Make tb instruction region (temporary 1024byte)
        let tb_map = match MemoryMap::new(
            1024,
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

        let tb_map_ptr = tb_map.data() as *const u64;
        let pe_map_ptr = pe_map.data() as *const u64;

        println!("tb_address = {:?}", tb_map_ptr);
        println!("pe_address = {:?}", pe_map_ptr);

        for tcg in &self.m_tcg_vec {
            println!("tcg_inst = {:?}", tcg);

            let mut diff_from_epilogue = unsafe { pe_map_ptr.offset_from(tb_map_ptr) };
            diff_from_epilogue *= 8;
            diff_from_epilogue += 32;

            let mut mc_byte = vec![];
            TCGX86::tcg_gen(diff_from_epilogue, pc_address, &tcg, &mut mc_byte);
            for be in &mc_byte {
                let be_data = *be;
                self.m_tcg_tb_vec.push(be_data);
            }
            pc_address += mc_byte.len() as u64;
        }

        unsafe {
            std::ptr::copy(
                self.m_tcg_tb_vec.as_ptr(),
                tb_map.data(),
                self.m_tcg_tb_vec.len(),
            );
        }

        let reg_ptr: *const [u64; 32] = &self.m_regs;

        unsafe {
            let func: unsafe extern "C" fn(gpr_base: *const [u64; 32], tb_map: *mut u8) -> u32 =
                mem::transmute(pe_map.data());

            let tb_data = tb_map.data();
            println!("reflect tb address = {:p}", tb_data);
            let ans = func(reg_ptr, tb_data);
            println!("ans = {:x}", ans);
        }
        self.dump_gpr();
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

    unsafe fn gen_tcg(&mut self, instructions: &[u8]) {
        let map = match MemoryMap::new(
            instructions.len(),
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

        std::ptr::copy(instructions.as_ptr(), map.data(), instructions.len());

        for byte_idx in (0..instructions.len()).step_by(4) {
            let map_data = map.data();
            // let map_raw = match map_data {
            //     Some(m) => m,
            //     _ => panic!("Decode Failed"),
            // };

            let inst = ((*map_data.offset(byte_idx as isize + 0) as u32) << 0)
                | ((*map_data.offset(byte_idx as isize + 1) as u32) << 8)
                | ((*map_data.offset(byte_idx as isize + 2) as u32) << 16)
                | ((*map_data.offset(byte_idx as isize + 3) as u32) << 24);

            println!("inst = {:08x}", inst);

            self.m_inst_vec.push(inst);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let cpu = CPU::new();
    cpu.run(&filename);

    return;
}
