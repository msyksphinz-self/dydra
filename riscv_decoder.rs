/* CAUTION! THIS SOURCE CODE IS GENERATED AUTOMATICALLY. DON'T MODIFY BY HAND. */


use super::riscv_inst_id::RiscvInstId;


fn decode_inst (inst: u32) -> Option<RiscvInstId> {
    let field_ld = ((inst as u64) >> 0) & (((1 as u64) << 2) - 1);
    return match field_ld {
        0x03 => 
        // Remaining Instruction is 164
        // lui        r[11:7],h[31:12]
        // auipc      r[11:7],h[31:12]
        // jal        r[11:7],uj[31:12]
        // jalr       r[11:7],r[19:15],h[31:20]
        // beq        r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // bne        r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // blt        r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // bge        r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // bltu       r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // bgeu       r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // lb         r[11:7],h[31:20](r[19:15])
        // lh         r[11:7],h[31:20](r[19:15])
        // lw         r[11:7],h[31:20](r[19:15])
        // lbu        r[11:7],h[31:20](r[19:15])
        // lhu        r[11:7],h[31:20](r[19:15])
        // sb         r[24:20],h[31:25]|h[11:7](r[19:15])
        // sh         r[24:20],h[31:25]|h[11:7](r[19:15])
        // sw         r[24:20],h[31:25]|h[11:7](r[19:15])
        // addi       r[11:7],r[19:15],h[31:20]
        // slti       r[11:7],r[19:15],h[31:20]
        // sltiu      r[11:7],r[19:15],h[31:20]
        // xori       r[11:7],r[19:15],h[31:20]
        // ori        r[11:7],r[19:15],h[31:20]
        // andi       r[11:7],r[19:15],h[31:20]
        // slli       r[11:7],r[19:15],h[25:20]
        // srli       r[11:7],r[19:15],h[24:20]
        // srai       r[11:7],r[19:15],h[24:20]
        // add        r[11:7],r[19:15],r[24:20]
        // sub        r[11:7],r[19:15],r[24:20]
        // sll        r[11:7],r[19:15],r[24:20]
        // slt        r[11:7],r[19:15],r[24:20]
        // sltu       r[11:7],r[19:15],r[24:20]
        // xor        r[11:7],r[19:15],r[24:20]
        // srl        r[11:7],r[19:15],r[24:20]
        // sra        r[11:7],r[19:15],r[24:20]
        // or         r[11:7],r[19:15],r[24:20]
        // and        r[11:7],r[19:15],r[24:20]
        // fence
        // fence.i
        // mul        r[11:7],r[19:15],r[24:20]
        // mulh       r[11:7],r[19:15],r[24:20]
        // mulhsu     r[11:7],r[19:15],r[24:20]
        // mulhu      r[11:7],r[19:15],r[24:20]
        // div        r[11:7],r[19:15],r[24:20]
        // divu       r[11:7],r[19:15],r[24:20]
        // rem        r[11:7],r[19:15],r[24:20]
        // remu       r[11:7],r[19:15],r[24:20]
        // lr.w       r[11:7],r[19:15]
        // sc.w       r[11:7],r[19:15],r[24:20]
        // amoswap.w  r[11:7],r[24:20],(r[19:15])
        // amoadd.w   r[11:7],r[24:20],(r[19:15])
        // amoxor.w   r[11:7],r[24:20],(r[19:15])
        // amoand.w   r[11:7],r[24:20],(r[19:15])
        // amoor.w    r[11:7],r[24:20],(r[19:15])
        // amomin.w   r[11:7],r[24:20],(r[19:15])
        // amomax.w   r[11:7],r[24:20],(r[19:15])
        // amominu.w  r[11:7],r[24:20],(r[19:15])
        // amomaxu.w  r[11:7],r[24:20],(r[19:15])
        // flw        f[11:7],h[31:20](r[19:15])
        // fsw        f[24:20],h[31:25]|h[11:7](r[19:15])
        // fmadd.s    f[11:7],f[19:15],f[24:20],f[31:27]
        // fmsub.s    f[11:7],f[19:15],f[24:20],f[31:27]
        // fnmsub.s   f[11:7],f[19:15],f[24:20],f[31:27]
        // fnmadd.s   f[11:7],f[19:15],f[24:20],f[31:27]
        // fadd.s     f[11:7],f[19:15],f[24:20]
        // fsub.s     f[11:7],f[19:15],f[24:20]
        // fmul.s     f[11:7],f[19:15],f[24:20]
        // fdiv.s     f[11:7],f[19:15],f[24:20]
        // fsqrt.s    f[11:7],f[19:15]
        // fsgnj.s    f[11:7],f[19:15],f[24:20]
        // fsgnjn.s   f[11:7],f[19:15],f[24:20]
        // fsgnjx.s   f[11:7],f[19:15],f[24:20]
        // fmin.s     f[11:7],f[19:15],f[24:20]
        // fmax.s     f[11:7],f[19:15],f[24:20]
        // fcvt.w.s   r[11:7],f[19:15]
        // fcvt.wu.s  r[11:7],f[19:15]
        // fmv.x.w    r[11:7],f[19:15]
        // feq.s      r[11:7],f[19:15],f[24:20]
        // flt.s      r[11:7],f[19:15],f[24:20]
        // fle.s      r[11:7],f[19:15],f[24:20]
        // fclass.s   f[11:7],f[19:15]
        // fcvt.s.w   f[11:7],r[19:15]
        // fcvt.s.wu  f[11:7],r[19:15]
        // fmv.w.x    f[11:7],r[19:15]
        // fld        f[11:7],r[19:15],h[31:20]
        // fsd        f[24:20],h[31:25]|h[11:7](r[19:15])
        // fmadd.d    f[11:7],f[19:15],f[24:20],f[31:27]
        // fmsub.d    f[11:7],f[19:15],f[24:20],f[31:27]
        // fnmsub.d   f[11:7],f[19:15],f[24:20],f[31:27]
        // fnmadd.d   f[11:7],f[19:15],f[24:20],f[31:27]
        // fadd.d     f[11:7],f[19:15],f[24:20]
        // fsub.d     f[11:7],f[19:15],f[24:20]
        // fmul.d     f[11:7],f[19:15],f[24:20]
        // fdiv.d     f[11:7],f[19:15],f[24:20]
        // fsqrt.d    f[11:7],f[19:15]
        // fsgnj.d    f[11:7],f[19:15],f[24:20]
        // fsgnjn.d   f[11:7],f[19:15],f[24:20]
        // fsgnjx.d   f[11:7],f[19:15],f[24:20]
        // fmin.d     f[11:7],f[19:15],f[24:20]
        // fmax.d     f[11:7],f[19:15],f[24:20]
        // fcvt.s.d   f[11:7],f[19:15]
        // fcvt.d.s   f[11:7],f[19:15]
        // feq.d      r[11:7],f[19:15],f[24:20]
        // flt.d      r[11:7],f[19:15],f[24:20]
        // fle.d      r[11:7],f[19:15],f[24:20]
        // fclass.d   r[11:7],f[19:15]
        // fcvt.w.d   r[11:7],f[19:15]
        // fcvt.wu.d  r[11:7],f[19:15]
        // fcvt.d.w   f[11:7],r[19:15]
        // fcvt.d.wu  f[11:7],r[19:15]
        // csrrw      r[11:7],h[31:20],r[19:15]
        // csrrs      r[11:7],h[31:20],r[19:15]
        // csrrc      r[11:7],h[31:20],r[19:15]
        // csrrwi     r[11:7],h[31:20],h[19:15]
        // csrrsi     r[11:7],h[31:20],h[19:15]
        // csrrci     r[11:7],h[31:20],h[19:15]
        // ecall
        // ebreak
        // uret
        // sret
        // hret
        // mret
        // mrts
        // mrth
        // wfi
        // sfence.vma r[19:15],r[24:20]
        // lwu        r[11:7],h[31:20](r[19:15])
        // ld         r[11:7],h[31:20](r[19:15])
        // sd         r[24:20],h[31:25]|h[11:7](r[19:15])
        // addiw      r[11:7],r[19:15],h[31:20]
        // slliw      r[11:7],r[19:15],r[24:20]
        // srliw      r[11:7],r[19:15],r[24:20]
        // sraiw      r[11:7],r[19:15],r[24:20]
        // addw       r[11:7],r[19:15],r[24:20]
        // subw       r[11:7],r[19:15],r[24:20]
        // sllw       r[11:7],r[19:15],r[24:20]
        // srlw       r[11:7],r[19:15],r[24:20]
        // sraw       r[11:7],r[19:15],r[24:20]
        // mulw       r[11:7],r[19:15],r[24:20]
        // divw       r[11:7],r[19:15],r[24:20]
        // divuw      r[11:7],r[19:15],r[24:20]
        // remw       r[11:7],r[19:15],r[24:20]
        // remuw      r[11:7],r[19:15],r[24:20]
        // lr.d       r[11:7],r[19:15]
        // sc.d       r[11:7],r[19:15],r[24:20]
        // amoswap.d  r[11:7],r[24:20],(r[19:15])
        // amoadd.d   r[11:7],r[24:20],(r[19:15])
        // amoxor.d   r[11:7],r[24:20],(r[19:15])
        // amoand.d   r[11:7],r[24:20],(r[19:15])
        // amoor.d    r[11:7],r[24:20],(r[19:15])
        // amomin.d   r[11:7],r[24:20],(r[19:15])
        // amomax.d   r[11:7],r[24:20],(r[19:15])
        // amominu.d  r[11:7],r[24:20],(r[19:15])
        // amomaxu.d  r[11:7],r[24:20],(r[19:15])
        // fcvt.l.s   r[11:7],f[19:15]
        // fcvt.lu.s  r[11:7],f[19:15]
        // fcvt.s.l   f[11:7],r[19:15]
        // fcvt.s.lu  f[11:7],r[19:15]
        // fcvt.l.d   r[11:7],f[19:15]
        // fcvt.lu.d  r[11:7],f[19:15]
        // fmv.x.d    r[11:7],f[19:15]
        // fcvt.d.l   f[11:7],r[19:15]
        // fcvt.d.lu  f[11:7],r[19:15]
        // fmv.d.x    f[11:7],r[19:15]
        decode_inst_ld_11 (inst),
        0x00 => 
        // Remaining Instruction is 9
        // c.addi4spn cr[4:2],u[12:5]
        // c.fld      cf[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.lw       cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.flw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.ld       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.fsd      cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.sw       cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.fsw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.sd       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00 (inst),
        0x01 => 
        // Remaining Instruction is 21
        // c.nop     
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.jal      u[12:12]|u[7:7]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        // c.addiw    r[11:7],h[12:12]|h[6:2]
        // c.li       r[11:7],u[12:12]|h[6:2]
        // c.addi16sp cr[4:2],u[12:5]
        // c.lui      r[11:7],u[12:12]|h[6:2]
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        // c.srli64   cr[9:7],u[12:12]|h[6:2]
        // c.srai     cr[9:7],u[12:12]|h[6:2]
        // c.srai64   cr[9:7],u[12:12]|h[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        // c.sub      cr[9:7],cr[4:2]
        // c.xor      cr[9:7],cr[4:2]
        // c.or       cr[9:7],cr[4:2]
        // c.and      cr[9:7],cr[4:2]
        // c.subw     cr[9:7],r[6:2]
        // c.addw     cr[9:7],r[6:2]
        // c.j        u[12:12]|u[8:8]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        // c.beqz     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        // c.bnez     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01 (inst),
        0x02 => 
        // Remaining Instruction is 14
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.fldsp    r[11:7],u[4:2]|u[12:12]|u[6:5]<<3 
        // c.lwsp     r[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.flwsp    f[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.ldsp     r[11:7],u[4:2]|u[12:12]|u[6:5]<<3
        // c.jr       r[11:7]
        // c.mv       r[11:7],r[6:2]
        // c.ebreak  
        // c.jalr     r[11:7]
        // c.add      r[11:7],r[6:2]
        // c.fsdsp    f[6:2],u[9:7]|u[12:10]<<3
        // c.swsp     r[6:2],u[8:7]|u[12:9]<<2
        // c.fswsp    f[6:2],u[9:7]|u[12:10]<<3
        // c.sdsp     r[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x0d => 
        Some(RiscvInstId::LUI),
        0x05 => 
        Some(RiscvInstId::AUIPC),
        0x1b => 
        Some(RiscvInstId::JAL),
        0x19 => 
        Some(RiscvInstId::JALR),
        0x18 => 
        // Remaining Instruction is 6
        // beq        r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // bne        r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // blt        r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // bge        r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // bltu       r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        // bgeu       r[19:15],r[24:20],u[31:31]|u[7:7]|u[30:25]|u[11:8]<<1
        decode_inst_ld_11_op_11000 (inst),
        0x00 => 
        // Remaining Instruction is 7
        // lb         r[11:7],h[31:20](r[19:15])
        // lh         r[11:7],h[31:20](r[19:15])
        // lw         r[11:7],h[31:20](r[19:15])
        // lbu        r[11:7],h[31:20](r[19:15])
        // lhu        r[11:7],h[31:20](r[19:15])
        // lwu        r[11:7],h[31:20](r[19:15])
        // ld         r[11:7],h[31:20](r[19:15])
        decode_inst_ld_11_op_00000 (inst),
        0x08 => 
        // Remaining Instruction is 4
        // sb         r[24:20],h[31:25]|h[11:7](r[19:15])
        // sh         r[24:20],h[31:25]|h[11:7](r[19:15])
        // sw         r[24:20],h[31:25]|h[11:7](r[19:15])
        // sd         r[24:20],h[31:25]|h[11:7](r[19:15])
        decode_inst_ld_11_op_01000 (inst),
        0x04 => 
        // Remaining Instruction is 9
        // addi       r[11:7],r[19:15],h[31:20]
        // slti       r[11:7],r[19:15],h[31:20]
        // sltiu      r[11:7],r[19:15],h[31:20]
        // xori       r[11:7],r[19:15],h[31:20]
        // ori        r[11:7],r[19:15],h[31:20]
        // andi       r[11:7],r[19:15],h[31:20]
        // slli       r[11:7],r[19:15],h[25:20]
        // srli       r[11:7],r[19:15],h[24:20]
        // srai       r[11:7],r[19:15],h[24:20]
        decode_inst_ld_11_op_00100 (inst),
        0x0c => 
        // Remaining Instruction is 18
        // add        r[11:7],r[19:15],r[24:20]
        // sub        r[11:7],r[19:15],r[24:20]
        // sll        r[11:7],r[19:15],r[24:20]
        // slt        r[11:7],r[19:15],r[24:20]
        // sltu       r[11:7],r[19:15],r[24:20]
        // xor        r[11:7],r[19:15],r[24:20]
        // srl        r[11:7],r[19:15],r[24:20]
        // sra        r[11:7],r[19:15],r[24:20]
        // or         r[11:7],r[19:15],r[24:20]
        // and        r[11:7],r[19:15],r[24:20]
        // mul        r[11:7],r[19:15],r[24:20]
        // mulh       r[11:7],r[19:15],r[24:20]
        // mulhsu     r[11:7],r[19:15],r[24:20]
        // mulhu      r[11:7],r[19:15],r[24:20]
        // div        r[11:7],r[19:15],r[24:20]
        // divu       r[11:7],r[19:15],r[24:20]
        // rem        r[11:7],r[19:15],r[24:20]
        // remu       r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01100 (inst),
        0x03 => 
        // Remaining Instruction is 2
        // fence
        // fence.i
        decode_inst_ld_11_op_00011 (inst),
        0x0b => 
        // Remaining Instruction is 22
        // lr.w       r[11:7],r[19:15]
        // sc.w       r[11:7],r[19:15],r[24:20]
        // amoswap.w  r[11:7],r[24:20],(r[19:15])
        // amoadd.w   r[11:7],r[24:20],(r[19:15])
        // amoxor.w   r[11:7],r[24:20],(r[19:15])
        // amoand.w   r[11:7],r[24:20],(r[19:15])
        // amoor.w    r[11:7],r[24:20],(r[19:15])
        // amomin.w   r[11:7],r[24:20],(r[19:15])
        // amomax.w   r[11:7],r[24:20],(r[19:15])
        // amominu.w  r[11:7],r[24:20],(r[19:15])
        // amomaxu.w  r[11:7],r[24:20],(r[19:15])
        // lr.d       r[11:7],r[19:15]
        // sc.d       r[11:7],r[19:15],r[24:20]
        // amoswap.d  r[11:7],r[24:20],(r[19:15])
        // amoadd.d   r[11:7],r[24:20],(r[19:15])
        // amoxor.d   r[11:7],r[24:20],(r[19:15])
        // amoand.d   r[11:7],r[24:20],(r[19:15])
        // amoor.d    r[11:7],r[24:20],(r[19:15])
        // amomin.d   r[11:7],r[24:20],(r[19:15])
        // amomax.d   r[11:7],r[24:20],(r[19:15])
        // amominu.d  r[11:7],r[24:20],(r[19:15])
        // amomaxu.d  r[11:7],r[24:20],(r[19:15])
        decode_inst_ld_11_op_01011 (inst),
        0x01 => 
        // Remaining Instruction is 2
        // flw        f[11:7],h[31:20](r[19:15])
        // fld        f[11:7],r[19:15],h[31:20]
        decode_inst_ld_11_op_00001 (inst),
        0x09 => 
        // Remaining Instruction is 2
        // fsw        f[24:20],h[31:25]|h[11:7](r[19:15])
        // fsd        f[24:20],h[31:25]|h[11:7](r[19:15])
        decode_inst_ld_11_op_01001 (inst),
        0x10 => 
        // Remaining Instruction is 2
        // fmadd.s    f[11:7],f[19:15],f[24:20],f[31:27]
        // fmadd.d    f[11:7],f[19:15],f[24:20],f[31:27]
        decode_inst_ld_11_op_10000 (inst),
        0x11 => 
        // Remaining Instruction is 2
        // fmsub.s    f[11:7],f[19:15],f[24:20],f[31:27]
        // fmsub.d    f[11:7],f[19:15],f[24:20],f[31:27]
        decode_inst_ld_11_op_10001 (inst),
        0x12 => 
        // Remaining Instruction is 2
        // fnmsub.s   f[11:7],f[19:15],f[24:20],f[31:27]
        // fnmsub.d   f[11:7],f[19:15],f[24:20],f[31:27]
        decode_inst_ld_11_op_10010 (inst),
        0x13 => 
        // Remaining Instruction is 2
        // fnmadd.s   f[11:7],f[19:15],f[24:20],f[31:27]
        // fnmadd.d   f[11:7],f[19:15],f[24:20],f[31:27]
        decode_inst_ld_11_op_10011 (inst),
        0x14 => 
        // Remaining Instruction is 50
        // fadd.s     f[11:7],f[19:15],f[24:20]
        // fsub.s     f[11:7],f[19:15],f[24:20]
        // fmul.s     f[11:7],f[19:15],f[24:20]
        // fdiv.s     f[11:7],f[19:15],f[24:20]
        // fsqrt.s    f[11:7],f[19:15]
        // fsgnj.s    f[11:7],f[19:15],f[24:20]
        // fsgnjn.s   f[11:7],f[19:15],f[24:20]
        // fsgnjx.s   f[11:7],f[19:15],f[24:20]
        // fmin.s     f[11:7],f[19:15],f[24:20]
        // fmax.s     f[11:7],f[19:15],f[24:20]
        // fcvt.w.s   r[11:7],f[19:15]
        // fcvt.wu.s  r[11:7],f[19:15]
        // fmv.x.w    r[11:7],f[19:15]
        // feq.s      r[11:7],f[19:15],f[24:20]
        // flt.s      r[11:7],f[19:15],f[24:20]
        // fle.s      r[11:7],f[19:15],f[24:20]
        // fclass.s   f[11:7],f[19:15]
        // fcvt.s.w   f[11:7],r[19:15]
        // fcvt.s.wu  f[11:7],r[19:15]
        // fmv.w.x    f[11:7],r[19:15]
        // fadd.d     f[11:7],f[19:15],f[24:20]
        // fsub.d     f[11:7],f[19:15],f[24:20]
        // fmul.d     f[11:7],f[19:15],f[24:20]
        // fdiv.d     f[11:7],f[19:15],f[24:20]
        // fsqrt.d    f[11:7],f[19:15]
        // fsgnj.d    f[11:7],f[19:15],f[24:20]
        // fsgnjn.d   f[11:7],f[19:15],f[24:20]
        // fsgnjx.d   f[11:7],f[19:15],f[24:20]
        // fmin.d     f[11:7],f[19:15],f[24:20]
        // fmax.d     f[11:7],f[19:15],f[24:20]
        // fcvt.s.d   f[11:7],f[19:15]
        // fcvt.d.s   f[11:7],f[19:15]
        // feq.d      r[11:7],f[19:15],f[24:20]
        // flt.d      r[11:7],f[19:15],f[24:20]
        // fle.d      r[11:7],f[19:15],f[24:20]
        // fclass.d   r[11:7],f[19:15]
        // fcvt.w.d   r[11:7],f[19:15]
        // fcvt.wu.d  r[11:7],f[19:15]
        // fcvt.d.w   f[11:7],r[19:15]
        // fcvt.d.wu  f[11:7],r[19:15]
        // fcvt.l.s   r[11:7],f[19:15]
        // fcvt.lu.s  r[11:7],f[19:15]
        // fcvt.s.l   f[11:7],r[19:15]
        // fcvt.s.lu  f[11:7],r[19:15]
        // fcvt.l.d   r[11:7],f[19:15]
        // fcvt.lu.d  r[11:7],f[19:15]
        // fmv.x.d    r[11:7],f[19:15]
        // fcvt.d.l   f[11:7],r[19:15]
        // fcvt.d.lu  f[11:7],r[19:15]
        // fmv.d.x    f[11:7],r[19:15]
        decode_inst_ld_11_op_10100 (inst),
        0x1c => 
        // Remaining Instruction is 16
        // csrrw      r[11:7],h[31:20],r[19:15]
        // csrrs      r[11:7],h[31:20],r[19:15]
        // csrrc      r[11:7],h[31:20],r[19:15]
        // csrrwi     r[11:7],h[31:20],h[19:15]
        // csrrsi     r[11:7],h[31:20],h[19:15]
        // csrrci     r[11:7],h[31:20],h[19:15]
        // ecall
        // ebreak
        // uret
        // sret
        // hret
        // mret
        // mrts
        // mrth
        // wfi
        // sfence.vma r[19:15],r[24:20]
        decode_inst_ld_11_op_11100 (inst),
        0x06 => 
        // Remaining Instruction is 4
        // addiw      r[11:7],r[19:15],h[31:20]
        // slliw      r[11:7],r[19:15],r[24:20]
        // srliw      r[11:7],r[19:15],r[24:20]
        // sraiw      r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_00110 (inst),
        0x0e => 
        // Remaining Instruction is 10
        // addw       r[11:7],r[19:15],r[24:20]
        // subw       r[11:7],r[19:15],r[24:20]
        // sllw       r[11:7],r[19:15],r[24:20]
        // srlw       r[11:7],r[19:15],r[24:20]
        // sraw       r[11:7],r[19:15],r[24:20]
        // mulw       r[11:7],r[19:15],r[24:20]
        // divw       r[11:7],r[19:15],r[24:20]
        // divuw      r[11:7],r[19:15],r[24:20]
        // remw       r[11:7],r[19:15],r[24:20]
        // remuw      r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01110 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::BEQ),
        0x01 => 
        Some(RiscvInstId::BNE),
        0x04 => 
        Some(RiscvInstId::BLT),
        0x05 => 
        Some(RiscvInstId::BGE),
        0x06 => 
        Some(RiscvInstId::BLTU),
        0x07 => 
        Some(RiscvInstId::BGEU),
      _ => None,
    }
}
fn decode_inst_ld_11_op_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::LB),
        0x01 => 
        Some(RiscvInstId::LH),
        0x02 => 
        Some(RiscvInstId::LW),
        0x04 => 
        Some(RiscvInstId::LBU),
        0x05 => 
        Some(RiscvInstId::LHU),
        0x06 => 
        Some(RiscvInstId::LWU),
        0x03 => 
        Some(RiscvInstId::LD),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::SB),
        0x01 => 
        Some(RiscvInstId::SH),
        0x02 => 
        Some(RiscvInstId::SW),
        0x03 => 
        Some(RiscvInstId::SD),
      _ => None,
    }
}
fn decode_inst_ld_11_op_00100 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::ADDI),
        0x02 => 
        Some(RiscvInstId::SLTI),
        0x03 => 
        Some(RiscvInstId::SLTIU),
        0x04 => 
        Some(RiscvInstId::XORI),
        0x06 => 
        Some(RiscvInstId::ORI),
        0x07 => 
        Some(RiscvInstId::ANDI),
        0x01 => 
        Some(RiscvInstId::SLLI),
        0x05 => 
        // Remaining Instruction is 2
        // srli       r[11:7],r[19:15],h[24:20]
        // srai       r[11:7],r[19:15],h[24:20]
        decode_inst_ld_11_op_00100_f3_101 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_00100_f3_101 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 => 
        Some(RiscvInstId::SRLI),
        0x08 => 
        Some(RiscvInstId::SRAI),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01100 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 => 
        // Remaining Instruction is 16
        // add        r[11:7],r[19:15],r[24:20]
        // sll        r[11:7],r[19:15],r[24:20]
        // slt        r[11:7],r[19:15],r[24:20]
        // sltu       r[11:7],r[19:15],r[24:20]
        // xor        r[11:7],r[19:15],r[24:20]
        // srl        r[11:7],r[19:15],r[24:20]
        // or         r[11:7],r[19:15],r[24:20]
        // and        r[11:7],r[19:15],r[24:20]
        // mul        r[11:7],r[19:15],r[24:20]
        // mulh       r[11:7],r[19:15],r[24:20]
        // mulhsu     r[11:7],r[19:15],r[24:20]
        // mulhu      r[11:7],r[19:15],r[24:20]
        // div        r[11:7],r[19:15],r[24:20]
        // divu       r[11:7],r[19:15],r[24:20]
        // rem        r[11:7],r[19:15],r[24:20]
        // remu       r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01100_r3_00000 (inst),
        0x08 => 
        // Remaining Instruction is 2
        // sub        r[11:7],r[19:15],r[24:20]
        // sra        r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01100_r3_01000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01100_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 8
        // add        r[11:7],r[19:15],r[24:20]
        // sll        r[11:7],r[19:15],r[24:20]
        // slt        r[11:7],r[19:15],r[24:20]
        // sltu       r[11:7],r[19:15],r[24:20]
        // xor        r[11:7],r[19:15],r[24:20]
        // srl        r[11:7],r[19:15],r[24:20]
        // or         r[11:7],r[19:15],r[24:20]
        // and        r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01100_r3_00000_f2_00 (inst),
        0x01 => 
        // Remaining Instruction is 8
        // mul        r[11:7],r[19:15],r[24:20]
        // mulh       r[11:7],r[19:15],r[24:20]
        // mulhsu     r[11:7],r[19:15],r[24:20]
        // mulhu      r[11:7],r[19:15],r[24:20]
        // div        r[11:7],r[19:15],r[24:20]
        // divu       r[11:7],r[19:15],r[24:20]
        // rem        r[11:7],r[19:15],r[24:20]
        // remu       r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01100_r3_00000_f2_01 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01100_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::ADD),
        0x01 => 
        Some(RiscvInstId::SLL),
        0x02 => 
        Some(RiscvInstId::SLT),
        0x03 => 
        Some(RiscvInstId::SLTU),
        0x04 => 
        Some(RiscvInstId::XOR),
        0x05 => 
        Some(RiscvInstId::SRL),
        0x06 => 
        Some(RiscvInstId::OR),
        0x07 => 
        Some(RiscvInstId::AND),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01100_r3_00000_f2_01 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::MUL),
        0x01 => 
        Some(RiscvInstId::MULH),
        0x02 => 
        Some(RiscvInstId::MULHSU),
        0x03 => 
        Some(RiscvInstId::MULHU),
        0x04 => 
        Some(RiscvInstId::DIV),
        0x05 => 
        Some(RiscvInstId::DIVU),
        0x06 => 
        Some(RiscvInstId::REM),
        0x07 => 
        Some(RiscvInstId::REMU),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01100_r3_01000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 2
        // sub        r[11:7],r[19:15],r[24:20]
        // sra        r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01100_r3_01000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01100_r3_01000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::SUB),
        0x05 => 
        Some(RiscvInstId::SRA),
      _ => None,
    }
}
fn decode_inst_ld_11_op_00011 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 => 
        // Remaining Instruction is 2
        // fence
        // fence.i
        decode_inst_ld_11_op_00011_r1_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_00011_r1_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::FENCE),
        0x01 => 
        Some(RiscvInstId::FENCE_I),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x02 => 
        // Remaining Instruction is 2
        // lr.w       r[11:7],r[19:15]
        // lr.d       r[11:7],r[19:15]
        decode_inst_ld_11_op_01011_r3_00010 (inst),
        0x03 => 
        // Remaining Instruction is 2
        // sc.w       r[11:7],r[19:15],r[24:20]
        // sc.d       r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01011_r3_00011 (inst),
        0x01 => 
        // Remaining Instruction is 2
        // amoswap.w  r[11:7],r[24:20],(r[19:15])
        // amoswap.d  r[11:7],r[24:20],(r[19:15])
        decode_inst_ld_11_op_01011_r3_00001 (inst),
        0x00 => 
        // Remaining Instruction is 2
        // amoadd.w   r[11:7],r[24:20],(r[19:15])
        // amoadd.d   r[11:7],r[24:20],(r[19:15])
        decode_inst_ld_11_op_01011_r3_00000 (inst),
        0x04 => 
        // Remaining Instruction is 2
        // amoxor.w   r[11:7],r[24:20],(r[19:15])
        // amoxor.d   r[11:7],r[24:20],(r[19:15])
        decode_inst_ld_11_op_01011_r3_00100 (inst),
        0x0c => 
        // Remaining Instruction is 2
        // amoand.w   r[11:7],r[24:20],(r[19:15])
        // amoand.d   r[11:7],r[24:20],(r[19:15])
        decode_inst_ld_11_op_01011_r3_01100 (inst),
        0x08 => 
        // Remaining Instruction is 2
        // amoor.w    r[11:7],r[24:20],(r[19:15])
        // amoor.d    r[11:7],r[24:20],(r[19:15])
        decode_inst_ld_11_op_01011_r3_01000 (inst),
        0x10 => 
        // Remaining Instruction is 2
        // amomin.w   r[11:7],r[24:20],(r[19:15])
        // amomin.d   r[11:7],r[24:20],(r[19:15])
        decode_inst_ld_11_op_01011_r3_10000 (inst),
        0x14 => 
        // Remaining Instruction is 2
        // amomax.w   r[11:7],r[24:20],(r[19:15])
        // amomax.d   r[11:7],r[24:20],(r[19:15])
        decode_inst_ld_11_op_01011_r3_10100 (inst),
        0x18 => 
        // Remaining Instruction is 2
        // amominu.w  r[11:7],r[24:20],(r[19:15])
        // amominu.d  r[11:7],r[24:20],(r[19:15])
        decode_inst_ld_11_op_01011_r3_11000 (inst),
        0x1c => 
        // Remaining Instruction is 2
        // amomaxu.w  r[11:7],r[24:20],(r[19:15])
        // amomaxu.d  r[11:7],r[24:20],(r[19:15])
        decode_inst_ld_11_op_01011_r3_11100 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_00010 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 => 
        // Remaining Instruction is 2
        // lr.w       r[11:7],r[19:15]
        // lr.d       r[11:7],r[19:15]
        decode_inst_ld_11_op_01011_r3_00010_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_00010_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::LR_W),
        0x03 => 
        Some(RiscvInstId::LR_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_00011 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::SC_W),
        0x03 => 
        Some(RiscvInstId::SC_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::AMOSWAP_W),
        0x03 => 
        Some(RiscvInstId::AMOSWAP_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::AMOADD_W),
        0x03 => 
        Some(RiscvInstId::AMOADD_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_00100 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::AMOXOR_W),
        0x03 => 
        Some(RiscvInstId::AMOXOR_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_01100 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::AMOAND_W),
        0x03 => 
        Some(RiscvInstId::AMOAND_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_01000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::AMOOR_W),
        0x03 => 
        Some(RiscvInstId::AMOOR_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_10000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::AMOMIN_W),
        0x03 => 
        Some(RiscvInstId::AMOMIN_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_10100 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::AMOMAX_W),
        0x03 => 
        Some(RiscvInstId::AMOMAX_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_11000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::AMOMINU_W),
        0x03 => 
        Some(RiscvInstId::AMOMINU_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01011_r3_11100 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::AMOMAXU_W),
        0x03 => 
        Some(RiscvInstId::AMOMAXU_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::FLW),
        0x03 => 
        Some(RiscvInstId::FLD),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01001 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::FSW),
        0x03 => 
        Some(RiscvInstId::FSD),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FMADD_S),
        0x01 => 
        Some(RiscvInstId::FMADD_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10001 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FMSUB_S),
        0x01 => 
        Some(RiscvInstId::FMSUB_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10010 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FNMSUB_S),
        0x01 => 
        Some(RiscvInstId::FNMSUB_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10011 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FNMADD_S),
        0x01 => 
        Some(RiscvInstId::FNMADD_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 => 
        // Remaining Instruction is 2
        // fadd.s     f[11:7],f[19:15],f[24:20]
        // fadd.d     f[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_00000 (inst),
        0x01 => 
        // Remaining Instruction is 2
        // fsub.s     f[11:7],f[19:15],f[24:20]
        // fsub.d     f[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_00001 (inst),
        0x02 => 
        // Remaining Instruction is 2
        // fmul.s     f[11:7],f[19:15],f[24:20]
        // fmul.d     f[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_00010 (inst),
        0x03 => 
        // Remaining Instruction is 2
        // fdiv.s     f[11:7],f[19:15],f[24:20]
        // fdiv.d     f[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_00011 (inst),
        0x0b => 
        // Remaining Instruction is 2
        // fsqrt.s    f[11:7],f[19:15]
        // fsqrt.d    f[11:7],f[19:15]
        decode_inst_ld_11_op_10100_r3_01011 (inst),
        0x04 => 
        // Remaining Instruction is 6
        // fsgnj.s    f[11:7],f[19:15],f[24:20]
        // fsgnjn.s   f[11:7],f[19:15],f[24:20]
        // fsgnjx.s   f[11:7],f[19:15],f[24:20]
        // fsgnj.d    f[11:7],f[19:15],f[24:20]
        // fsgnjn.d   f[11:7],f[19:15],f[24:20]
        // fsgnjx.d   f[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_00100 (inst),
        0x05 => 
        // Remaining Instruction is 4
        // fmin.s     f[11:7],f[19:15],f[24:20]
        // fmax.s     f[11:7],f[19:15],f[24:20]
        // fmin.d     f[11:7],f[19:15],f[24:20]
        // fmax.d     f[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_00101 (inst),
        0x18 => 
        // Remaining Instruction is 8
        // fcvt.w.s   r[11:7],f[19:15]
        // fcvt.wu.s  r[11:7],f[19:15]
        // fcvt.w.d   r[11:7],f[19:15]
        // fcvt.wu.d  r[11:7],f[19:15]
        // fcvt.l.s   r[11:7],f[19:15]
        // fcvt.lu.s  r[11:7],f[19:15]
        // fcvt.l.d   r[11:7],f[19:15]
        // fcvt.lu.d  r[11:7],f[19:15]
        decode_inst_ld_11_op_10100_r3_11000 (inst),
        0x1c => 
        // Remaining Instruction is 4
        // fmv.x.w    r[11:7],f[19:15]
        // fclass.s   f[11:7],f[19:15]
        // fclass.d   r[11:7],f[19:15]
        // fmv.x.d    r[11:7],f[19:15]
        decode_inst_ld_11_op_10100_r3_11100 (inst),
        0x14 => 
        // Remaining Instruction is 6
        // feq.s      r[11:7],f[19:15],f[24:20]
        // flt.s      r[11:7],f[19:15],f[24:20]
        // fle.s      r[11:7],f[19:15],f[24:20]
        // feq.d      r[11:7],f[19:15],f[24:20]
        // flt.d      r[11:7],f[19:15],f[24:20]
        // fle.d      r[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_10100 (inst),
        0x1a => 
        // Remaining Instruction is 8
        // fcvt.s.w   f[11:7],r[19:15]
        // fcvt.s.wu  f[11:7],r[19:15]
        // fcvt.d.w   f[11:7],r[19:15]
        // fcvt.d.wu  f[11:7],r[19:15]
        // fcvt.s.l   f[11:7],r[19:15]
        // fcvt.s.lu  f[11:7],r[19:15]
        // fcvt.d.l   f[11:7],r[19:15]
        // fcvt.d.lu  f[11:7],r[19:15]
        decode_inst_ld_11_op_10100_r3_11010 (inst),
        0x1e => 
        // Remaining Instruction is 2
        // fmv.w.x    f[11:7],r[19:15]
        // fmv.d.x    f[11:7],r[19:15]
        decode_inst_ld_11_op_10100_r3_11110 (inst),
        0x08 => 
        // Remaining Instruction is 2
        // fcvt.s.d   f[11:7],f[19:15]
        // fcvt.d.s   f[11:7],f[19:15]
        decode_inst_ld_11_op_10100_r3_01000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FADD_S),
        0x01 => 
        Some(RiscvInstId::FADD_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FSUB_S),
        0x01 => 
        Some(RiscvInstId::FSUB_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_00010 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FMUL_S),
        0x01 => 
        Some(RiscvInstId::FMUL_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_00011 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FDIV_S),
        0x01 => 
        Some(RiscvInstId::FDIV_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_01011 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FSQRT_S),
        0x01 => 
        Some(RiscvInstId::FSQRT_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_00100 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 3
        // fsgnj.s    f[11:7],f[19:15],f[24:20]
        // fsgnjn.s   f[11:7],f[19:15],f[24:20]
        // fsgnjx.s   f[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_00100_f2_00 (inst),
        0x01 => 
        // Remaining Instruction is 3
        // fsgnj.d    f[11:7],f[19:15],f[24:20]
        // fsgnjn.d   f[11:7],f[19:15],f[24:20]
        // fsgnjx.d   f[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_00100_f2_01 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_00100_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::FSGNJ_S),
        0x01 => 
        Some(RiscvInstId::FSGNJN_S),
        0x02 => 
        Some(RiscvInstId::FSGNJX_S),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_00100_f2_01 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::FSGNJ_D),
        0x01 => 
        Some(RiscvInstId::FSGNJN_D),
        0x02 => 
        Some(RiscvInstId::FSGNJX_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_00101 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 2
        // fmin.s     f[11:7],f[19:15],f[24:20]
        // fmax.s     f[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_00101_f2_00 (inst),
        0x01 => 
        // Remaining Instruction is 2
        // fmin.d     f[11:7],f[19:15],f[24:20]
        // fmax.d     f[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_00101_f2_01 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_00101_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::FMIN_S),
        0x01 => 
        Some(RiscvInstId::FMAX_S),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_00101_f2_01 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::FMIN_D),
        0x01 => 
        Some(RiscvInstId::FMAX_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 4
        // fcvt.w.s   r[11:7],f[19:15]
        // fcvt.wu.s  r[11:7],f[19:15]
        // fcvt.l.s   r[11:7],f[19:15]
        // fcvt.lu.s  r[11:7],f[19:15]
        decode_inst_ld_11_op_10100_r3_11000_f2_00 (inst),
        0x01 => 
        // Remaining Instruction is 4
        // fcvt.w.d   r[11:7],f[19:15]
        // fcvt.wu.d  r[11:7],f[19:15]
        // fcvt.l.d   r[11:7],f[19:15]
        // fcvt.lu.d  r[11:7],f[19:15]
        decode_inst_ld_11_op_10100_r3_11000_f2_01 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 => 
        Some(RiscvInstId::FCVT_W_S),
        0x01 => 
        Some(RiscvInstId::FCVT_WU_S),
        0x02 => 
        Some(RiscvInstId::FCVT_L_S),
        0x03 => 
        Some(RiscvInstId::FCVT_LU_S),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11000_f2_01 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 => 
        Some(RiscvInstId::FCVT_W_D),
        0x01 => 
        Some(RiscvInstId::FCVT_WU_D),
        0x02 => 
        Some(RiscvInstId::FCVT_L_D),
        0x03 => 
        Some(RiscvInstId::FCVT_LU_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11100 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 2
        // fmv.x.w    r[11:7],f[19:15]
        // fclass.s   f[11:7],f[19:15]
        decode_inst_ld_11_op_10100_r3_11100_f2_00 (inst),
        0x01 => 
        // Remaining Instruction is 2
        // fclass.d   r[11:7],f[19:15]
        // fmv.x.d    r[11:7],f[19:15]
        decode_inst_ld_11_op_10100_r3_11100_f2_01 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11100_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 => 
        // Remaining Instruction is 2
        // fmv.x.w    r[11:7],f[19:15]
        // fclass.s   f[11:7],f[19:15]
        decode_inst_ld_11_op_10100_r3_11100_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11100_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::FMV_X_W),
        0x01 => 
        Some(RiscvInstId::FCLASS_S),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11100_f2_01 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 => 
        // Remaining Instruction is 2
        // fclass.d   r[11:7],f[19:15]
        // fmv.x.d    r[11:7],f[19:15]
        decode_inst_ld_11_op_10100_r3_11100_f2_01_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11100_f2_01_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x01 => 
        Some(RiscvInstId::FCLASS_D),
        0x00 => 
        Some(RiscvInstId::FMV_X_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_10100 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 3
        // feq.s      r[11:7],f[19:15],f[24:20]
        // flt.s      r[11:7],f[19:15],f[24:20]
        // fle.s      r[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_10100_f2_00 (inst),
        0x01 => 
        // Remaining Instruction is 3
        // feq.d      r[11:7],f[19:15],f[24:20]
        // flt.d      r[11:7],f[19:15],f[24:20]
        // fle.d      r[11:7],f[19:15],f[24:20]
        decode_inst_ld_11_op_10100_r3_10100_f2_01 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_10100_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::FEQ_S),
        0x01 => 
        Some(RiscvInstId::FLT_S),
        0x00 => 
        Some(RiscvInstId::FLE_S),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_10100_f2_01 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 => 
        Some(RiscvInstId::FEQ_D),
        0x01 => 
        Some(RiscvInstId::FLT_D),
        0x00 => 
        Some(RiscvInstId::FLE_D),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11010 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 4
        // fcvt.s.w   f[11:7],r[19:15]
        // fcvt.s.wu  f[11:7],r[19:15]
        // fcvt.s.l   f[11:7],r[19:15]
        // fcvt.s.lu  f[11:7],r[19:15]
        decode_inst_ld_11_op_10100_r3_11010_f2_00 (inst),
        0x01 => 
        // Remaining Instruction is 4
        // fcvt.d.w   f[11:7],r[19:15]
        // fcvt.d.wu  f[11:7],r[19:15]
        // fcvt.d.l   f[11:7],r[19:15]
        // fcvt.d.lu  f[11:7],r[19:15]
        decode_inst_ld_11_op_10100_r3_11010_f2_01 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11010_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 => 
        Some(RiscvInstId::FCVT_S_W),
        0x01 => 
        Some(RiscvInstId::FCVT_S_WU),
        0x02 => 
        Some(RiscvInstId::FCVT_S_L),
        0x03 => 
        Some(RiscvInstId::FCVT_S_LU),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11010_f2_01 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 => 
        Some(RiscvInstId::FCVT_D_W),
        0x01 => 
        Some(RiscvInstId::FCVT_D_WU),
        0x02 => 
        Some(RiscvInstId::FCVT_D_L),
        0x03 => 
        Some(RiscvInstId::FCVT_D_LU),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_11110 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FMV_W_X),
        0x01 => 
        Some(RiscvInstId::FMV_D_X),
      _ => None,
    }
}
fn decode_inst_ld_11_op_10100_r3_01000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        Some(RiscvInstId::FCVT_S_D),
        0x01 => 
        Some(RiscvInstId::FCVT_D_S),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11100 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x01 => 
        Some(RiscvInstId::CSRRW),
        0x02 => 
        Some(RiscvInstId::CSRRS),
        0x03 => 
        Some(RiscvInstId::CSRRC),
        0x05 => 
        Some(RiscvInstId::CSRRWI),
        0x06 => 
        Some(RiscvInstId::CSRRSI),
        0x07 => 
        Some(RiscvInstId::CSRRCI),
        0x00 => 
        // Remaining Instruction is 10
        // ecall
        // ebreak
        // uret
        // sret
        // hret
        // mret
        // mrts
        // mrth
        // wfi
        // sfence.vma r[19:15],r[24:20]
        decode_inst_ld_11_op_11100_f3_000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11100_f3_000 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 => 
        // Remaining Instruction is 3
        // ecall
        // ebreak
        // uret
        decode_inst_ld_11_op_11100_f3_000_r3_00000 (inst),
        0x02 => 
        // Remaining Instruction is 3
        // sret
        // wfi
        // sfence.vma r[19:15],r[24:20]
        decode_inst_ld_11_op_11100_f3_000_r3_00010 (inst),
        0x04 => 
        // Remaining Instruction is 2
        // hret
        // mrth
        decode_inst_ld_11_op_11100_f3_000_r3_00100 (inst),
        0x06 => 
        // Remaining Instruction is 2
        // mret
        // mrts
        decode_inst_ld_11_op_11100_f3_000_r3_00110 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11100_f3_000_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 3
        // ecall
        // ebreak
        // uret
        decode_inst_ld_11_op_11100_f3_000_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11100_f3_000_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 => 
        Some(RiscvInstId::ECALL),
        0x01 => 
        Some(RiscvInstId::EBREAK),
        0x02 => 
        Some(RiscvInstId::URET),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11100_f3_000_r3_00010 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 2
        // sret
        // wfi
        decode_inst_ld_11_op_11100_f3_000_r3_00010_f2_00 (inst),
        0x01 => 
        Some(RiscvInstId::SFENCE_VMA),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11100_f3_000_r3_00010_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x02 => 
        Some(RiscvInstId::SRET),
        0x05 => 
        Some(RiscvInstId::WFI),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11100_f3_000_r3_00100 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 2
        // hret
        // mrth
        decode_inst_ld_11_op_11100_f3_000_r3_00100_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11100_f3_000_r3_00100_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x02 => 
        Some(RiscvInstId::HRET),
        0x06 => 
        Some(RiscvInstId::MRTH),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11100_f3_000_r3_00110 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 2
        // mret
        // mrts
        decode_inst_ld_11_op_11100_f3_000_r3_00110_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_11100_f3_000_r3_00110_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x02 => 
        Some(RiscvInstId::MRET),
        0x05 => 
        Some(RiscvInstId::MRTS),
      _ => None,
    }
}
fn decode_inst_ld_11_op_00110 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::ADDIW),
        0x01 => 
        Some(RiscvInstId::SLLIW),
        0x05 => 
        // Remaining Instruction is 2
        // srliw      r[11:7],r[19:15],r[24:20]
        // sraiw      r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_00110_f3_101 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_00110_f3_101 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 => 
        Some(RiscvInstId::SRLIW),
        0x08 => 
        Some(RiscvInstId::SRAIW),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01110 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 => 
        // Remaining Instruction is 8
        // addw       r[11:7],r[19:15],r[24:20]
        // sllw       r[11:7],r[19:15],r[24:20]
        // srlw       r[11:7],r[19:15],r[24:20]
        // mulw       r[11:7],r[19:15],r[24:20]
        // divw       r[11:7],r[19:15],r[24:20]
        // divuw      r[11:7],r[19:15],r[24:20]
        // remw       r[11:7],r[19:15],r[24:20]
        // remuw      r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01110_r3_00000 (inst),
        0x08 => 
        // Remaining Instruction is 2
        // subw       r[11:7],r[19:15],r[24:20]
        // sraw       r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01110_r3_01000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01110_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 3
        // addw       r[11:7],r[19:15],r[24:20]
        // sllw       r[11:7],r[19:15],r[24:20]
        // srlw       r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01110_r3_00000_f2_00 (inst),
        0x01 => 
        // Remaining Instruction is 5
        // mulw       r[11:7],r[19:15],r[24:20]
        // divw       r[11:7],r[19:15],r[24:20]
        // divuw      r[11:7],r[19:15],r[24:20]
        // remw       r[11:7],r[19:15],r[24:20]
        // remuw      r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01110_r3_00000_f2_01 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01110_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::ADDW),
        0x01 => 
        Some(RiscvInstId::SLLW),
        0x05 => 
        Some(RiscvInstId::SRLW),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01110_r3_00000_f2_01 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::MULW),
        0x04 => 
        Some(RiscvInstId::DIVW),
        0x05 => 
        Some(RiscvInstId::DIVUW),
        0x06 => 
        Some(RiscvInstId::REMW),
        0x07 => 
        Some(RiscvInstId::REMUW),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01110_r3_01000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 => 
        // Remaining Instruction is 2
        // subw       r[11:7],r[19:15],r[24:20]
        // sraw       r[11:7],r[19:15],r[24:20]
        decode_inst_ld_11_op_01110_r3_01000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_11_op_01110_r3_01000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        Some(RiscvInstId::SUBW),
        0x05 => 
        Some(RiscvInstId::SRAW),
      _ => None,
    }
}
fn decode_inst_ld_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 9
        // c.addi4spn cr[4:2],u[12:5]
        // c.fld      cf[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.lw       cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.flw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.ld       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.fsd      cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.sw       cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.fsw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.sd       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_00_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 9
        // c.addi4spn cr[4:2],u[12:5]
        // c.fld      cf[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.lw       cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.flw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.ld       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.fsd      cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.sw       cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.fsw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.sd       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_00_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 9
        // c.addi4spn cr[4:2],u[12:5]
        // c.fld      cf[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.lw       cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.flw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.ld       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.fsd      cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.sw       cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.fsw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.sd       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_00_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        // Remaining Instruction is 5
        // c.addi4spn cr[4:2],u[12:5]
        // c.fld      cf[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.lw       cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.flw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.ld       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00000 (inst),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        // Remaining Instruction is 4
        // c.fsd      cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        // c.sw       cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.fsw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.sd       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00001 (inst),
      _ => None,
    }
}
fn decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 | 0x01 =>
        Some(RiscvInstId::C_ADDI4SPN),
        0x02 | 0x03 =>
        Some(RiscvInstId::C_FLD),
        0x04 | 0x05 =>
        Some(RiscvInstId::C_LW),
        0x06 | 0x07 =>
        // Remaining Instruction is 2
        // c.flw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.ld       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00000_f3_110 (inst),
      _ => None,
    }
}
fn decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00000_f3_110 (inst: u32) -> Option<RiscvInstId> {
    let field_rd = ((inst as u64) >> 7) & (((1 as u64) << 5) - 1);
    return match field_rd {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.flw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.ld       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00000_f3_110_rd_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00000_f3_110_rd_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.flw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.ld       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00000_f3_110_rd_00000_op_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x02 | 0x03 =>
        Some(RiscvInstId::C_FSD),
        0x04 | 0x05 =>
        Some(RiscvInstId::C_SW),
        0x06 | 0x07 =>
        // Remaining Instruction is 2
        // c.fsw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.sd       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00001_f3_110 (inst),
      _ => None,
    }
}
fn decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00001_f3_110 (inst: u32) -> Option<RiscvInstId> {
    let field_rd = ((inst as u64) >> 7) & (((1 as u64) << 5) - 1);
    return match field_rd {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.fsw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.sd       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00001_f3_110_rd_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00001_f3_110_rd_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.fsw      cr[4:2],cr[9:7],u[5:5]|u[12:10]|u[6:6]<<2
        // c.sd       cr[4:2],cr[9:7],u[6:5]|u[12:10]<<3
        decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00001_f3_110_rd_00000_op_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        // Remaining Instruction is 9
        // c.nop     
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        // c.srai     cr[9:7],u[12:12]|h[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        // c.sub      cr[9:7],cr[4:2]
        // c.xor      cr[9:7],cr[4:2]
        // c.or       cr[9:7],cr[4:2]
        // c.and      cr[9:7],cr[4:2]
        decode_inst_ld_01_f3_000 (inst),
        0x01 => 
        // Remaining Instruction is 6
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srli64   cr[9:7],u[12:12]|h[6:2]
        // c.srai64   cr[9:7],u[12:12]|h[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        // c.subw     cr[9:7],r[6:2]
        // c.addw     cr[9:7],r[6:2]
        decode_inst_ld_01_f3_001 (inst),
        0x02 | 0x03 =>
        // Remaining Instruction is 3
        // c.jal      u[12:12]|u[7:7]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        // c.addiw    r[11:7],h[12:12]|h[6:2]
        // c.j        u[12:12]|u[8:8]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        decode_inst_ld_01_f3_010 (inst),
        0x04 | 0x05 =>
        // Remaining Instruction is 2
        // c.li       r[11:7],u[12:12]|h[6:2]
        // c.beqz     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_100 (inst),
        0x06 | 0x07 =>
        // Remaining Instruction is 3
        // c.addi16sp cr[4:2],u[12:5]
        // c.lui      r[11:7],u[12:12]|h[6:2]
        // c.bnez     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_110 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000 (inst: u32) -> Option<RiscvInstId> {
    let field_rd = ((inst as u64) >> 7) & (((1 as u64) << 5) - 1);
    return match field_rd {
        0x00 => 
        // Remaining Instruction is 2
        // c.nop     
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_00000 (inst),
        0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_00001 (inst),
        0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srai     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_01000 (inst),
        0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_10000 (inst),
        0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 5
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.sub      cr[9:7],cr[4:2]
        // c.xor      cr[9:7],cr[4:2]
        // c.or       cr[9:7],cr[4:2]
        // c.and      cr[9:7],cr[4:2]
        decode_inst_ld_01_f3_000_rd_11000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x00 => 
        // Remaining Instruction is 2
        // c.nop     
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_00000_op_00000 (inst),
        0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        Some(RiscvInstId::C_SRLI),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_00000_op_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.nop     
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_00000_op_00000_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_00000_op_00000_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.nop     
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_00000_op_00000_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_00000_op_00000_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.nop     
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_00000_op_00000_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_00000_op_00000_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_NOP),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_SRLI),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_00001_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_00001_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_00001_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_00001_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srli     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_00001_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_00001_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_ADDI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_SRLI),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_01000 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srai     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_01000_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_01000_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srai     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_01000_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_01000_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srai     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_01000_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_01000_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_ADDI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_SRAI),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_10000 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_10000_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_10000_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_10000_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_10000_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        decode_inst_ld_01_f3_000_rd_10000_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_10000_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_ADDI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_ANDI),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_11000 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 5
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.sub      cr[9:7],cr[4:2]
        // c.xor      cr[9:7],cr[4:2]
        // c.or       cr[9:7],cr[4:2]
        // c.and      cr[9:7],cr[4:2]
        decode_inst_ld_01_f3_000_rd_11000_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_11000_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 5
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.sub      cr[9:7],cr[4:2]
        // c.xor      cr[9:7],cr[4:2]
        // c.or       cr[9:7],cr[4:2]
        // c.and      cr[9:7],cr[4:2]
        decode_inst_ld_01_f3_000_rd_11000_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_11000_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 5
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.sub      cr[9:7],cr[4:2]
        // c.xor      cr[9:7],cr[4:2]
        // c.or       cr[9:7],cr[4:2]
        // c.and      cr[9:7],cr[4:2]
        decode_inst_ld_01_f3_000_rd_11000_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_11000_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_ADDI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        // Remaining Instruction is 4
        // c.sub      cr[9:7],cr[4:2]
        // c.xor      cr[9:7],cr[4:2]
        // c.or       cr[9:7],cr[4:2]
        // c.and      cr[9:7],cr[4:2]
        decode_inst_ld_01_f3_000_rd_11000_r3_00000_f2_00_r2_00000_r1_00001 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_000_rd_11000_r3_00000_f2_00_r2_00000_r1_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 =>
        Some(RiscvInstId::C_SUB),
        0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f =>
        Some(RiscvInstId::C_XOR),
        0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 =>
        Some(RiscvInstId::C_OR),
        0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        Some(RiscvInstId::C_AND),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_001 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 6
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srli64   cr[9:7],u[12:12]|h[6:2]
        // c.srai64   cr[9:7],u[12:12]|h[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        // c.subw     cr[9:7],r[6:2]
        // c.addw     cr[9:7],r[6:2]
        decode_inst_ld_01_f3_001_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_001_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 6
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srli64   cr[9:7],u[12:12]|h[6:2]
        // c.srai64   cr[9:7],u[12:12]|h[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        // c.subw     cr[9:7],r[6:2]
        // c.addw     cr[9:7],r[6:2]
        decode_inst_ld_01_f3_001_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_001_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 6
        // c.addi     r[11:7],u[12:12]|u[6:2]
        // c.srli64   cr[9:7],u[12:12]|h[6:2]
        // c.srai64   cr[9:7],u[12:12]|h[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        // c.subw     cr[9:7],r[6:2]
        // c.addw     cr[9:7],r[6:2]
        decode_inst_ld_01_f3_001_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_001_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_ADDI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        // Remaining Instruction is 5
        // c.srli64   cr[9:7],u[12:12]|h[6:2]
        // c.srai64   cr[9:7],u[12:12]|h[6:2]
        // c.andi     cr[9:7],u[12:12]|h[6:2]
        // c.subw     cr[9:7],r[6:2]
        // c.addw     cr[9:7],r[6:2]
        decode_inst_ld_01_f3_001_r3_00000_f2_00_r2_00000_r1_00001 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_001_r3_00000_f2_00_r2_00000_r1_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_rd = ((inst as u64) >> 7) & (((1 as u64) << 5) - 1);
    return match field_rd {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 =>
        Some(RiscvInstId::C_SRLI64),
        0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f =>
        Some(RiscvInstId::C_SRAI64),
        0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 =>
        Some(RiscvInstId::C_ANDI),
        0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.subw     cr[9:7],r[6:2]
        // c.addw     cr[9:7],r[6:2]
        decode_inst_ld_01_f3_001_r3_00000_f2_00_r2_00000_r1_00001_rd_11000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_001_r3_00000_f2_00_r2_00000_r1_00001_rd_11000 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 =>
        Some(RiscvInstId::C_SUBW),
        0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f =>
        Some(RiscvInstId::C_ADDW),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_010 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 3
        // c.jal      u[12:12]|u[7:7]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        // c.addiw    r[11:7],h[12:12]|h[6:2]
        // c.j        u[12:12]|u[8:8]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        decode_inst_ld_01_f3_010_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_010_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 3
        // c.jal      u[12:12]|u[7:7]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        // c.addiw    r[11:7],h[12:12]|h[6:2]
        // c.j        u[12:12]|u[8:8]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        decode_inst_ld_01_f3_010_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_010_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 3
        // c.jal      u[12:12]|u[7:7]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        // c.addiw    r[11:7],h[12:12]|h[6:2]
        // c.j        u[12:12]|u[8:8]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        decode_inst_ld_01_f3_010_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_010_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        // Remaining Instruction is 2
        // c.jal      u[12:12]|u[7:7]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        // c.addiw    r[11:7],h[12:12]|h[6:2]
        decode_inst_ld_01_f3_010_r3_00000_f2_00_r2_00000_r1_00000 (inst),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_J),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_010_r3_00000_f2_00_r2_00000_r1_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_rd = ((inst as u64) >> 7) & (((1 as u64) << 5) - 1);
    return match field_rd {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.jal      u[12:12]|u[7:7]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        // c.addiw    r[11:7],h[12:12]|h[6:2]
        decode_inst_ld_01_f3_010_r3_00000_f2_00_r2_00000_r1_00000_rd_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_010_r3_00000_f2_00_r2_00000_r1_00000_rd_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.jal      u[12:12]|u[7:7]|u[10:9]|u[6:6]|u[7:7]|u[2:2]|u[11:11]|u[5:3]<<1
        // c.addiw    r[11:7],h[12:12]|h[6:2]
        decode_inst_ld_01_f3_010_r3_00000_f2_00_r2_00000_r1_00000_rd_00000_op_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_100 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.li       r[11:7],u[12:12]|h[6:2]
        // c.beqz     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_100_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_100_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.li       r[11:7],u[12:12]|h[6:2]
        // c.beqz     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_100_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_100_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.li       r[11:7],u[12:12]|h[6:2]
        // c.beqz     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_100_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_100_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_LI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_BEQZ),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_110 (inst: u32) -> Option<RiscvInstId> {
    let field_rd = ((inst as u64) >> 7) & (((1 as u64) << 5) - 1);
    return match field_rd {
        0x02 => 
        // Remaining Instruction is 2
        // c.addi16sp cr[4:2],u[12:5]
        // c.bnez     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_110_rd_00010 (inst),
        0x00 | 0x01 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.lui      r[11:7],u[12:12]|h[6:2]
        // c.bnez     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_110_rd_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_110_rd_00010 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.addi16sp cr[4:2],u[12:5]
        // c.bnez     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_110_rd_00010_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_110_rd_00010_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.addi16sp cr[4:2],u[12:5]
        // c.bnez     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_110_rd_00010_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_110_rd_00010_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.addi16sp cr[4:2],u[12:5]
        // c.bnez     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_110_rd_00010_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_110_rd_00010_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_ADDI16SP),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_BNEZ),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_110_rd_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.lui      r[11:7],u[12:12]|h[6:2]
        // c.bnez     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_110_rd_00000_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_110_rd_00000_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.lui      r[11:7],u[12:12]|h[6:2]
        // c.bnez     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_110_rd_00000_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_110_rd_00000_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.lui      r[11:7],u[12:12]|h[6:2]
        // c.bnez     cr[9:7],u[12:12]|u[6:5]|u[2:2]|u[11:10]|u[4:3]<<1
        decode_inst_ld_01_f3_110_rd_00000_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_01_f3_110_rd_00000_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_LUI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_BNEZ),
      _ => None,
    }
}
fn decode_inst_ld_10 (inst: u32) -> Option<RiscvInstId> {
    let field_f3 = ((inst as u64) >> 12) & (((1 as u64) << 3) - 1);
    return match field_f3 {
        0x00 => 
        // Remaining Instruction is 3
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.jr       r[11:7]
        // c.mv       r[11:7],r[6:2]
        decode_inst_ld_10_f3_000 (inst),
        0x01 => 
        // Remaining Instruction is 4
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.ebreak  
        // c.jalr     r[11:7]
        // c.add      r[11:7],r[6:2]
        decode_inst_ld_10_f3_001 (inst),
        0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.fldsp    r[11:7],u[4:2]|u[12:12]|u[6:5]<<3 
        // c.fsdsp    f[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_010 (inst),
        0x04 | 0x05 =>
        // Remaining Instruction is 2
        // c.lwsp     r[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.swsp     r[6:2],u[8:7]|u[12:9]<<2
        decode_inst_ld_10_f3_100 (inst),
        0x06 | 0x07 =>
        // Remaining Instruction is 4
        // c.flwsp    f[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.ldsp     r[11:7],u[4:2]|u[12:12]|u[6:5]<<3
        // c.fswsp    f[6:2],u[9:7]|u[12:10]<<3
        // c.sdsp     r[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_110 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_000 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x00 => 
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.jr       r[11:7]
        decode_inst_ld_10_f3_000_op_00000 (inst),
        0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.mv       r[11:7],r[6:2]
        decode_inst_ld_10_f3_000_op_00001 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_000_op_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.jr       r[11:7]
        decode_inst_ld_10_f3_000_op_00000_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_000_op_00000_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.jr       r[11:7]
        decode_inst_ld_10_f3_000_op_00000_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_000_op_00000_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.jr       r[11:7]
        decode_inst_ld_10_f3_000_op_00000_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_000_op_00000_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_SLLI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_JR),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_000_op_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.mv       r[11:7],r[6:2]
        decode_inst_ld_10_f3_000_op_00001_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_000_op_00001_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.mv       r[11:7],r[6:2]
        decode_inst_ld_10_f3_000_op_00001_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_000_op_00001_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.mv       r[11:7],r[6:2]
        decode_inst_ld_10_f3_000_op_00001_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_000_op_00001_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_SLLI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_MV),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_001 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x00 => 
        // Remaining Instruction is 3
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.ebreak  
        // c.jalr     r[11:7]
        decode_inst_ld_10_f3_001_op_00000 (inst),
        0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.add      r[11:7],r[6:2]
        decode_inst_ld_10_f3_001_op_00001 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_001_op_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_rd = ((inst as u64) >> 7) & (((1 as u64) << 5) - 1);
    return match field_rd {
        0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.jalr     r[11:7]
        decode_inst_ld_10_f3_001_op_00000_rd_00001 (inst),
        0x00 => 
        Some(RiscvInstId::C_EBREAK),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_001_op_00000_rd_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.jalr     r[11:7]
        decode_inst_ld_10_f3_001_op_00000_rd_00001_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_001_op_00000_rd_00001_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.jalr     r[11:7]
        decode_inst_ld_10_f3_001_op_00000_rd_00001_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_001_op_00000_rd_00001_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.jalr     r[11:7]
        decode_inst_ld_10_f3_001_op_00000_rd_00001_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_001_op_00000_rd_00001_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_SLLI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_JALR),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_001_op_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.add      r[11:7],r[6:2]
        decode_inst_ld_10_f3_001_op_00001_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_001_op_00001_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.add      r[11:7],r[6:2]
        decode_inst_ld_10_f3_001_op_00001_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_001_op_00001_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.slli     r[11:7],u[12:12]|u[6:2]
        // c.add      r[11:7],r[6:2]
        decode_inst_ld_10_f3_001_op_00001_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_001_op_00001_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_SLLI),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_ADD),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_010 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.fldsp    r[11:7],u[4:2]|u[12:12]|u[6:5]<<3 
        // c.fsdsp    f[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_010_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_010_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.fldsp    r[11:7],u[4:2]|u[12:12]|u[6:5]<<3 
        // c.fsdsp    f[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_010_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_010_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.fldsp    r[11:7],u[4:2]|u[12:12]|u[6:5]<<3 
        // c.fsdsp    f[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_010_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_010_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_FLDSP),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_FSDSP),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_100 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.lwsp     r[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.swsp     r[6:2],u[8:7]|u[12:9]<<2
        decode_inst_ld_10_f3_100_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_100_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 2
        // c.lwsp     r[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.swsp     r[6:2],u[8:7]|u[12:9]<<2
        decode_inst_ld_10_f3_100_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_100_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.lwsp     r[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.swsp     r[6:2],u[8:7]|u[12:9]<<2
        decode_inst_ld_10_f3_100_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_100_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        Some(RiscvInstId::C_LWSP),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        Some(RiscvInstId::C_SWSP),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_110 (inst: u32) -> Option<RiscvInstId> {
    let field_r3 = ((inst as u64) >> 27) & (((1 as u64) << 5) - 1);
    return match field_r3 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 4
        // c.flwsp    f[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.ldsp     r[11:7],u[4:2]|u[12:12]|u[6:5]<<3
        // c.fswsp    f[6:2],u[9:7]|u[12:10]<<3
        // c.sdsp     r[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_110_r3_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_110_r3_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_f2 = ((inst as u64) >> 25) & (((1 as u64) << 2) - 1);
    return match field_f2 {
        0x00 | 0x01 | 0x02 | 0x03 =>
        // Remaining Instruction is 4
        // c.flwsp    f[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.ldsp     r[11:7],u[4:2]|u[12:12]|u[6:5]<<3
        // c.fswsp    f[6:2],u[9:7]|u[12:10]<<3
        // c.sdsp     r[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_110_r3_00000_f2_00 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_110_r3_00000_f2_00 (inst: u32) -> Option<RiscvInstId> {
    let field_r2 = ((inst as u64) >> 20) & (((1 as u64) << 5) - 1);
    return match field_r2 {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 4
        // c.flwsp    f[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.ldsp     r[11:7],u[4:2]|u[12:12]|u[6:5]<<3
        // c.fswsp    f[6:2],u[9:7]|u[12:10]<<3
        // c.sdsp     r[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_r1 = ((inst as u64) >> 15) & (((1 as u64) << 5) - 1);
    return match field_r1 {
        0x00 | 0x02 | 0x04 | 0x06 | 0x08 | 0x0a | 0x0c | 0x0e | 0x10 | 0x12 | 0x14 | 0x16 | 0x18 | 0x1a | 0x1c | 0x1e =>
        // Remaining Instruction is 2
        // c.flwsp    f[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.ldsp     r[11:7],u[4:2]|u[12:12]|u[6:5]<<3
        decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00000 (inst),
        0x01 | 0x03 | 0x05 | 0x07 | 0x09 | 0x0b | 0x0d | 0x0f | 0x11 | 0x13 | 0x15 | 0x17 | 0x19 | 0x1b | 0x1d | 0x1f =>
        // Remaining Instruction is 2
        // c.fswsp    f[6:2],u[9:7]|u[12:10]<<3
        // c.sdsp     r[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00001 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_rd = ((inst as u64) >> 7) & (((1 as u64) << 5) - 1);
    return match field_rd {
        0x00 => 
        Some(RiscvInstId::C_FLWSP),
        0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.flwsp    f[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.ldsp     r[11:7],u[4:2]|u[12:12]|u[6:5]<<3
        decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00000_rd_00001 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00000_rd_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.flwsp    f[11:7],u[3:2]|u[12:12]|u[6:4]<<2
        // c.ldsp     r[11:7],u[4:2]|u[12:12]|u[6:5]<<3
        decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00000_rd_00001_op_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00001 (inst: u32) -> Option<RiscvInstId> {
    let field_rd = ((inst as u64) >> 7) & (((1 as u64) << 5) - 1);
    return match field_rd {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.fswsp    f[6:2],u[9:7]|u[12:10]<<3
        // c.sdsp     r[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00001_rd_00000 (inst),
      _ => None,
    }
}
fn decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00001_rd_00000 (inst: u32) -> Option<RiscvInstId> {
    let field_op = ((inst as u64) >> 2) & (((1 as u64) << 5) - 1);
    return match field_op {
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d | 0x0e | 0x0f | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f =>
        // Remaining Instruction is 2
        // c.fswsp    f[6:2],u[9:7]|u[12:10]<<3
        // c.sdsp     r[6:2],u[9:7]|u[12:10]<<3
        decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00001_rd_00000_op_00000 (inst),
      _ => None,
    }
}
