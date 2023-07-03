use super::riscv_inst_id::RiscvInstId;

pub fn decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00000_rd_00001_op_00000(
    _inst: u32,
) -> Option<(RiscvInstId, usize)> {
    // if (m_pe_thread->GetBitMode() == RiscvBitMode_t::Bit32) {
    //   return Some((RiscvInstId::C_FLWSP;
    // } else if (m_pe_thread->GetBitMode() == RiscvBitMode_t::Bit64) {
    return Some((RiscvInstId::C_LDSP, 2));
    // } else {
    //   return Some((RiscvInstId::SENTINEL_MAX;
    // }
}

pub fn decode_inst_ld_10_f3_110_r3_00000_f2_00_r2_00000_r1_00001_rd_00000_op_00000(
    _inst: u32,
) -> Option<(RiscvInstId, usize)> {
    // if (m_pe_thread->GetBitMode() == RiscvBitMode_t::Bit32) {
    //   return Some((RiscvInstId::C_FSWSP;
    // } else if (m_pe_thread->GetBitMode() == RiscvBitMode_t::Bit64) {
    return Some((RiscvInstId::C_SDSP, 2));
    // } else {
    //   return Some((RiscvInstId::SENTINEL_MAX;
    // }
}

pub fn decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00000_f3_110_rd_00000_op_00000(
    _inst: u32,
) -> Option<(RiscvInstId, usize)> {
    // if (m_pe_thread->GetBitMode() == RiscvBitMode_t::Bit32) {
    //   return Some((RiscvInstId::C_FLW;
    // } else if (m_pe_thread->GetBitMode() == RiscvBitMode_t::Bit64) {
    return Some((RiscvInstId::C_LD, 2));
    // } else {
    //   return Some((RiscvInstId::SENTINEL_MAX;
    // }
}

pub fn decode_inst_ld_00_r3_00000_f2_00_r2_00000_r1_00001_f3_110_rd_00000_op_00000(
    _inst: u32,
) -> Option<(RiscvInstId, usize)> {
    // if (m_pe_thread->GetBitMode() == RiscvBitMode_t::Bit32) {
    //   return Some((RiscvInstId::C_FSW;
    // } else if (m_pe_thread->GetBitMode() == RiscvBitMode_t::Bit64) {
    return Some((RiscvInstId::C_SD, 2));
    // } else {
    //   return Some((RiscvInstId::SENTINEL_MAX;
    // }
}

pub fn decode_inst_ld_01_f3_010_r3_00000_f2_00_r2_00000_r1_00000_rd_00000_op_00000(
    _inst: u32,
) -> Option<(RiscvInstId, usize)> {
    // if (m_pe_thread->GetBitMode() == RiscvBitMode_t::Bit32) {
    //   return Some((RiscvInstId::C_JAL;
    // } else if (m_pe_thread->GetBitMode() == RiscvBitMode_t::Bit64) {
    return Some((RiscvInstId::C_ADDIW, 2));
    // } else {
    //   return Some((RiscvInstId::SENTINEL_MAX;
    // }
}
