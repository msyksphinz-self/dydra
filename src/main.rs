extern crate mmap;
use std::mem;

use mmap::{MapOption, MemoryMap};

pub mod riscv_decoder;
pub mod riscv_inst_id;

use crate::riscv_decoder::decode_inst;

unsafe fn reflect(instructions: &[u8]) {

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

unsafe fn gen_tcg(instructions: &[u8])
{
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

        let inst = ((*map_data.offset(byte_idx as isize + 0) as u32) << 0) |
        ((*map_data.offset(byte_idx as isize + 1) as u32) <<  8) |
        ((*map_data.offset(byte_idx as isize + 2) as u32) << 16) |
        ((*map_data.offset(byte_idx as isize + 3) as u32) << 24);

        println!("inst = {:08x}", inst);

        let riscv_id = match decode_inst(inst) {
            Some(id) => id,
            _ => panic!("Decode Failed"),
        };

        println!("riscv_id = {:?}", riscv_id);
    }

}


fn main() {

    let riscv_guestcode: [u8; 8] = [
        0x13, 0x05, 0xa0, 0x00,  // addi a0,zero,10
        0x67, 0x80, 0x00, 0x00   // ret
    ];
    unsafe {
        gen_tcg(&riscv_guestcode);
    }


    let x86_hostcode: [u8; 8] = [
        0x48, 0x83, 0xc7, 0x0a,  // add 0xa, %rdi
        0x48, 0x89, 0xf8,        // mov %rdi, %rax
        0xc3];                   // retq
    unsafe {
        reflect(&x86_hostcode);
    }
}
