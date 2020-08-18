extern crate mmap;
use std::mem;
use std::rc::Rc;

use mmap::{MapOption, MemoryMap};

pub mod riscv_decoder;
pub mod riscv_inst_id;

use crate::riscv_decoder::decode_inst;
use crate::riscv_inst_id::RiscvInstId;

macro_rules! get_rs1_addr {
    ($inst:expr) => {
        ($inst >> 15) & 0x1f
    };
}

macro_rules! get_rs2_addr {
    ($inst:expr) => {
        ($inst >> 20) & 0x1f
    };
}

#[allow(unused_macros)]
macro_rules! get_rs3_addr {
    ($inst:expr) => {
        ($inst >> 27) & 0x1f
    };
}

macro_rules! get_rd_addr {
    ($inst:expr) => {
        ($inst >> 7) & 0x1f
    };
}

struct CPU {
    m_regs: [u64; 32],

    m_inst_vec: Vec<u32>,
    m_opcode_vec: Vec<Box<TCGOp>>,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            m_regs: [2; 32],
            m_inst_vec: Vec::new(),
            m_opcode_vec: Vec::new(),
        }
    }

    pub fn run(mut self) {
        let riscv_guestcode: [u8; 8] = [
            0x13, 0x05, 0xa0, 0x00, // addi a0,zero,10
            0x67, 0x80, 0x00, 0x00, // ret
        ];
        unsafe {
            self.gen_tcg(&riscv_guestcode);
        }

        // let x86_hostcode: [u8; 8] = [
        //     0x48, 0x83, 0xc7, 0x0a,  // add 0xa, %rdi
        //     0x48, 0x89, 0xf8,        // mov %rdi, %rax
        //     0xc3];                   // retq
        // unsafe {
        //     self.reflect(&x86_hostcode);
        // }

        for inst in &self.m_inst_vec {
            let riscv_id = match decode_inst(*inst) {
                Some(id) => id,
                _ => panic!("Decode Failed"),
            };

            let tcg_inst = match riscv_id {
                RiscvInstId::ADDI => Self::tcg_gen_addi(inst),
                RiscvInstId::SUB => Self::tcg_gen_sub(inst),
                RiscvInstId::JALR => Self::tcg_gen_jalr(inst),
                _ => panic!("Not supported these instructions."),
            };

            self.m_opcode_vec.push(tcg_inst);

            // println!("riscv_id = {:?}, tcg_inst = {:?}", riscv_id, tcg_inst);
            //
            // let raw_x = tcg_inst.arg0.value as *const u64;
            // println!("address0 = {:p}", raw_x);
            //
            // let raw_x = tcg_inst.arg1.value as *const u64;
            // println!("address1 = {:p}", raw_x);
        }

        for op in &self.m_opcode_vec {
            println!("tcg_inst = {:?}", op);

            let raw_x = op.arg0.value as *const u64;
            println!("address0 = {:p}", &raw_x);

            let raw_x = op.arg1.value as *const u64;
            println!("address1 = {:p}", &raw_x);

            // self.reflect(&op);
        }
    }

    fn tcg_gen_addi(inst: &u32) -> Box<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let imm_const: u64 = (*inst as u64) >> 20 & 0xfff;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = Box::new(TCGOp::new(TCGOpcode::ADD, *rd, *rs1, *imm));

        tcg_inst
    }

    fn tcg_gen_sub(inst: &u32) -> Box<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let rs2_addr: usize = get_rs2_addr!(*inst) as usize;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let rs2 = Box::new(TCGv::new_reg(rs2_addr as u64));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = Box::new(TCGOp::new(TCGOpcode::SUB, *rd, *rs1, *rs2));

        tcg_inst
    }

    fn tcg_gen_jalr(inst: &u32) -> Box<TCGOp> {
        let rs1_addr: usize = get_rs1_addr!(*inst) as usize;
        let imm_const: u64 = (*inst as u64) >> 20 & 0xfff;
        let rd_addr: usize = get_rd_addr!(*inst) as usize;

        let rs1 = Box::new(TCGv::new_reg(rs1_addr as u64));
        let imm = Box::new(TCGv::new_imm(imm_const));
        let rd = Box::new(TCGv::new_reg(rd_addr as u64));

        let tcg_inst = Box::new(TCGOp::new(TCGOpcode::JMP, *rd, *rs1, *imm));

        tcg_inst
    }

    // unsafe fn reflect(&mut self, instructions: &[u8]) {
    unsafe fn reflect(&mut self, instructions: &[u8]) {
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

        let func: unsafe extern "C" fn() -> u8 = mem::transmute(map.data());

        let ans = func();
        println!("ans = {:x}", ans);
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

#[derive(Debug, Copy, Clone)]
enum TCGOpcode {
    ADD = 0,
    SUB = 1,
    JMP = 2,
}

#[derive(Debug, Copy, Clone)]
enum TCGvType {
    Register = 0,
    Immediate = 1,
}

#[derive(Debug, Copy, Clone)]
struct TCGOp {
    op: TCGOpcode,
    arg0: TCGv,
    arg1: TCGv,
    arg2: TCGv,
}

impl TCGOp {
    pub fn new(opcode: TCGOpcode, a1: TCGv, a2: TCGv, a3: TCGv) -> TCGOp {
        TCGOp {
            op: opcode,
            arg0: a1,
            arg1: a2,
            arg2: a3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct TCGv {
    t: TCGvType,
    value: u64,
}

impl TCGv {
    pub fn new_reg(val: u64) -> TCGv {
        TCGv {
            t: TCGvType::Register,
            value: val,
        }
    }

    pub fn new_imm(val: u64) -> TCGv {
        TCGv {
            t: TCGvType::Immediate,
            value: val,
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.run();
}
