use softfloat_wrapper_riscv::{ExceptionFlags, Float, RoundingMode, F64};
use crate::target::riscv::riscv_csr::{CsrAddr};
use crate::emu_env::EmuEnv;
use crate::target::riscv::riscv::{ExceptCode};

impl EmuEnv {
    pub fn helper_func_csrrw(
        emu: &mut EmuEnv,
        dest: u32,
        source: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
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

    pub fn helper_func_csrrs(
        emu: &mut EmuEnv,
        dest: u32,
        source: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
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

    pub fn helper_func_csrrc(
        emu: &mut EmuEnv,
        dest: u32,
        source: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
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

    pub fn helper_func_csrrwi(
        emu: &mut EmuEnv,
        dest: u32,
        imm: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
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

    pub fn helper_func_csrrsi(
        emu: &mut EmuEnv,
        dest: u32,
        imm: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
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

    pub fn helper_func_csrrci(
        emu: &mut EmuEnv,
        dest: u32,
        imm: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
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

    pub fn helper_func_mret(
        emu: &mut EmuEnv,
        dest: u32,
        imm: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
        println!(
            "helper_mret(emu, {:}, {:}, 0x{:03x}) is called!",
            dest, imm, csr_addr
        );
        emu.m_pc[0] = emu.m_csr.csrrc(CsrAddr::Mepc, 0 as i64) as u64;
        print!("PC is set to 0x{:08x}\n", emu.m_pc[0]);
        return 0;
    }

    pub fn helper_func_ecall(
        emu: &mut EmuEnv,
        dest: u32,
        imm: u32,
        csr_addr: u32,
        _dummy: u32,
    ) -> usize {
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

    pub fn helper_func_fadd_d(emu: &mut EmuEnv, fd: u32, fs1: u32, fs2: u32, _dummy: u32) -> usize {
        println!("fadd(emu, {:}, {:}, {:}) is called!", fd, fs1, fs2);
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data.add(fs2_data, RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        return 0;
    }

    pub fn helper_func_fsub_d(emu: &mut EmuEnv, fd: u32, fs1: u32, fs2: u32, _dummy: u32) -> usize {
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data.sub(fs2_data, RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        println!(
            "fsub({:?}, {:?}, {:?}) is called!",
            fs1_data, fs2_data, fd_data
        );
        return 0;
    }

    pub fn helper_func_fmul_d(emu: &mut EmuEnv, fd: u32, fs1: u32, fs2: u32, _dummy: u32) -> usize {
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data.mul(fs2_data, RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        println!("fmul(emu, {:}, {:}, 0x{:03x}) is called!", fd, fs1, fs2);

        return 0;
    }

    pub fn helper_func_fdiv_d(emu: &mut EmuEnv, fd: u32, fs1: u32, fs2: u32, _dummy: u32) -> usize {
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data.div(fs2_data, RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        println!("fdiv(emu, {:}, {:}, 0x{:03x}) is called!", fd, fs1, fs2);

        return 0;
    }

    pub fn helper_func_fmadd_d(emu: &mut EmuEnv, fd: u32, fs1: u32, fs2: u32, fs3: u32) -> usize {
        println!(
            "fmadd(emu, {:}, {:}, {:}, {:}) is called!",
            fd, fs1, fs2, fs3
        );
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let fs3_data = F64::from_bits(emu.m_fregs[fs3 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data
            .mul(fs2_data, RoundingMode::TiesToEven)
            .add(fs3_data, RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        return 0;
    }

    pub fn helper_func_fmsub_d(emu: &mut EmuEnv, fd: u32, fs1: u32, fs2: u32, fs3: u32) -> usize {
        println!(
            "fmsub(emu, {:}, {:}, {:}, {:}) is called!",
            fd, fs1, fs2, fs3
        );
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let fs3_data = F64::from_bits(emu.m_fregs[fs3 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data
            .mul(fs2_data, RoundingMode::TiesToEven)
            .sub(fs3_data, RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        return 0;
    }

    pub fn helper_func_fnmsub_d(emu: &mut EmuEnv, fd: u32, fs1: u32, fs2: u32, fs3: u32) -> usize {
        println!(
            "fnmsub(emu, {:}, {:}, {:}, {:}) is called!",
            fd, fs1, fs2, fs3
        );
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let fs3_data = F64::from_bits(emu.m_fregs[fs3 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data
            .mul(fs2_data, RoundingMode::TiesToEven)
            .neg()
            .add(fs3_data, RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        return 0;
    }

    pub fn helper_func_fnmadd_d(emu: &mut EmuEnv, fd: u32, fs1: u32, fs2: u32, fs3: u32) -> usize {
        println!(
            "fnmadd(emu, {:}, {:}, {:}, {:}) is called!",
            fd, fs1, fs2, fs3
        );
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let fs3_data = F64::from_bits(emu.m_fregs[fs3 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data
            .mul(fs2_data, RoundingMode::TiesToEven)
            .neg()
            .sub(fs3_data, RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        return 0;
    }

    pub fn helper_func_fsqrt_d(emu: &mut EmuEnv, fd: u32, fs1: u32, _: u32, _: u32) -> usize {
        println!("fsqrt(emu, {:}, {:}) is called!", fd, fs1);

        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data.sqrt(RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        return 0;
    }

    pub fn helper_func_feq_d(emu: &mut EmuEnv, rd: u32, fs1: u32, fs2: u32, _dummy: u32) -> usize {
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_regs[rd as usize] = fs1_data.eq(fs2_data) as u64;
        flag.get();
        let ret_flag = flag.bits();
        println!("feq(emu, {:}, {:}, {:}) => {:} is called!", rd, fs1, fs2, ret_flag);
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }

    pub fn helper_func_flt_d(emu: &mut EmuEnv, rd: u32, fs1: u32, fs2: u32, _dummy: u32) -> usize {
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_regs[rd as usize] = fs1_data.lt(fs2_data) as u64;
        flag.get();
        let ret_flag = flag.bits();
        println!("flt(emu, {:}, {:}, {:}) is called! => {:}", rd, fs1, fs2, ret_flag);
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }

    pub fn helper_func_fle_d(emu: &mut EmuEnv, rd: u32, fs1: u32, fs2: u32, _dummy: u32) -> usize {
        println!("fle(emu, {:}, {:}, {:}) is called!", rd, fs1, fs2);
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_regs[rd as usize] = fs1_data.le(fs2_data) as u64;
        flag.get();
        let ret_flag = flag.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }

    pub fn helper_func_fclass_d(emu: &mut EmuEnv, rd: u32, fs1: u32, _fs2: u32, _dummy: u32) -> usize {
        println!("fclass(emu, {:}, {:}) is called!", rd, fs1);
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        #[allow(unused_assignments)]
        let mut result = 0;
        if fs1_data.is_negative_infinity() {
            result= 1 << 0;
        } else if fs1_data.is_positive_infinity() {
            result= 1 << 7;
        } else if fs1_data.is_negative_zero() {
            result= 1 << 3;
        } else if fs1_data.is_positive_zero() {
            result= 1 << 4;
        } else if fs1_data.is_negative_zero() || fs1_data.is_negative_subnormal(){
            result = 1 << 2;
        } else if fs1_data.is_positive_zero () || fs1_data.is_positive_subnormal() {
            result = 1 << 5;
        } else if fs1_data.is_nan() {
            if fs1_data.is_quiet_nan() {
                result = 1 << 9;
            } else {
                result = 1 << 8;
            }
        } else if fs1_data.is_negative() {
            result = 1 << 1;
        } else {
            result = 1 << 6;
        }
        emu.m_regs[rd as usize] = result as u64;
        return 0;
    }
}
