extern crate mmap;
use std::mem;

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
    m_tcg_vec: Vec<Box<TCGOp>>,
    m_tcg_raw_vec: Vec<u8>,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            m_regs: [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31,
            ],
            m_inst_vec: vec![],
            m_tcg_vec: vec![],
            m_tcg_raw_vec: vec![],
        }
    }

    pub fn run(mut self) {
        let riscv_guestcode: [u8; 8] = [
            0x13, 0x05, 0x40, 0x06, // addi a0,zero,100
            0x67, 0x80, 0x00, 0x00, // ret
        ];
        let host_prologue = [
            0x55, // pushq %rbp
            0x54, // pushq %rbx
            0x48, 0x8b, 0xef, // movq     %rdi, %rbp
            0x48, 0x81, 0xc4, 0x78, 0xfb, 0xff, 0xff, // addq     $-0x488, %rsp
        ];
        let host_epilogue = [
            0x48, 0x81, 0xc4, 0x88, 0x04, 0x00, 0x00, // addq     $0x488, %rsp
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

            let tcg_inst = match riscv_id {
                RiscvInstId::ADDI => Self::tcg_gen_addi(inst),
                RiscvInstId::SUB => Self::tcg_gen_sub(inst),
                RiscvInstId::JALR => Self::tcg_gen_jalr(inst),
                _ => panic!("Not supported these instructions."),
            };

            self.m_tcg_vec.push(tcg_inst);

            // println!("riscv_id = {:?}, tcg_inst = {:?}", riscv_id, tcg_inst);

            // let raw_x = tcg_inst.arg0.value as *const u64;
            // println!("address0 = {:p}", raw_x);

            // let raw_x = tcg_inst.arg1.value as *const u64;
            // println!("address1 = {:p}", raw_x);
        }

        // Emit Prologue
        for b in &host_prologue {
            self.m_tcg_raw_vec.push(*b);
        }

        for tcg in self.m_tcg_vec {
            println!("tcg_inst = {:?}", tcg);

            let raw_x = tcg.arg0.value as *const u64;
            println!("address0 = {:p}", &raw_x);

            let raw_x = tcg.arg1.value as *const u64;
            println!("address1 = {:p}", &raw_x);

            let (mc_raw, mc_byte) = Self::translate(&tcg);

            for (i, be) in mc_raw.to_be_bytes().iter().enumerate() {
                if i < 8 - mc_byte {
                    continue;
                }
                self.m_tcg_raw_vec.push(*be);
            }
        }

        // Emit Epilogue
        for b in &host_epilogue {
            self.m_tcg_raw_vec.push(*b);
        }

        {
            for b in &self.m_tcg_raw_vec {
                print!("{:02x} ", b);
            }
            print!("\n");
        }

        unsafe {
            let v = self.m_tcg_raw_vec.as_slice();
            let reg_ptr: *const [u64; 32] = &self.m_regs;
            Self::reflect(v, reg_ptr);
        }
    }

    fn translate(tcg: &TCGOp) -> (u64, usize) {
        match tcg.op {
            TCGOpcode::ADD => Self::translate_addi(tcg),
            TCGOpcode::SUB => Self::translate_sub(tcg),
            TCGOpcode::JMP => Self::translate_ret(tcg),
        }
    }

    fn translate_addi(tcg: &TCGOp) -> (u64, usize) {
        assert_eq!(tcg.arg0.t, TCGvType::Register);
        assert_eq!(tcg.arg1.t, TCGvType::Register);
        assert_eq!(tcg.arg2.t, TCGvType::Immediate);

        if tcg.arg0.value == 0 {
            // if destination is x0, skip generate host machine code.
            return (0, 0);
        }
        if tcg.arg1.value == 0 {
            // if source register is x0, just generate immediate value.
            let revert_bytes = (tcg.arg2.value as u32).swap_bytes();
            // movl   imm,reg_addr(%rbp)
            let raw_mc: u64 =
                0xc745_00_00000000 | (revert_bytes as u64) | ((tcg.arg0.value * 8 as u64) << 32);
            return (raw_mc, 56 / 8);
        }

        panic!("This function is not supported!");
    }

    fn translate_sub(tcg: &TCGOp) -> (u64, usize) {
        panic!("This function is not supported!");
    }

    fn translate_ret(tcg: &TCGOp) -> (u64, usize) {
        assert_eq!(tcg.op, TCGOpcode::JMP);
        if tcg.arg0.t == TCGvType::Register
            && tcg.arg0.value == 0
            && tcg.arg1.t == TCGvType::Register
            && tcg.arg1.value == 1
        {
            // mov 0x50(%rbp), eax  0x50 is location of x10
            let raw_mc: u64 = 0x8b45_50;
            return (raw_mc, 3);
        }
        panic!("This function is not supported!")
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

    unsafe fn reflect(instructions: &[u8], gpr_base: *const [u64; 32]) {
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

        let func: unsafe extern "C" fn(gpr_base: *const [u64; 32]) -> u32 =
            mem::transmute(map.data());

        let ans = func(gpr_base);
        println!("ans = {:}", ans);
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

#[derive(Debug, Copy, Clone, PartialEq)]
enum TCGOpcode {
    ADD = 0,
    SUB = 1,
    JMP = 2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
    let cpu = CPU::new();
    cpu.run();

    return;
}
