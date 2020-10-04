use softfloat_wrapper_riscv::{ExceptionFlags, Float, RoundingMode, F64};
use crate::target::riscv::riscv_csr::{CsrAddr};
use crate::emu_env::EmuEnv;

impl EmuEnv {
    pub fn helper_func_fadd_d(emu: &mut EmuEnv, fd: u32, fs1: u32, fs2: u32, _dummy: u32) -> usize {
        println!("fadd(emu, {:}, {:}, {:}) is called!", fd, fs1, fs2);
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data.add(fs2_data, RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
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

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
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

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
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

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
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

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
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

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
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

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
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

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
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

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
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

    pub fn helper_func_fmax_d(emu: &mut EmuEnv, rd: u32, fs1: u32, fs2: u32, _dummy: u32) -> usize {
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_fregs[rd as usize] = 
        if fs1_data.is_nan() && fs2_data.is_nan() { 
            F64::quiet_nan().bits() as u64
        } else if fs2_data.lt_quiet(fs1_data) || fs2_data.is_nan() || fs1_data.eq(fs2_data) && fs2_data.is_negative() {
            fs1_data.bits() as u64
        } else {
            fs2_data.bits() as u64
        };
        flag.get();
        let ret_flag = flag.bits();
        println!("fmax_d(emu, {:}, {:}, {:}) is called! => {:}", rd, fs1, fs2, ret_flag);
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }

    pub fn helper_func_fmin_d(emu: &mut EmuEnv, rd: u32, fs1: u32, fs2: u32, _dummy: u32) -> usize {
        let fs1_data = F64::from_bits(emu.m_fregs[fs1 as usize]);
        let fs2_data = F64::from_bits(emu.m_fregs[fs2 as usize]);
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_fregs[rd as usize] = 
        if fs1_data.is_nan() && fs2_data.is_nan() { 
            F64::quiet_nan().bits() as u64
        } else if fs1_data.lt_quiet(fs2_data) || fs2_data.is_nan() || fs1_data.eq(fs2_data) && fs1_data.is_negative() {
            fs1_data.bits() as u64
        } else {
            fs2_data.bits() as u64
        };
        flag.get();
        let ret_flag = flag.bits();
        println!("fmin_d(emu, {:}, {:}, {:}) is called!", rd, fs1, fs2);
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }
}
