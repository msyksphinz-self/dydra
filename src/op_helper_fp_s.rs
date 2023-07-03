use crate::emu_env::EmuEnv;
use crate::target::riscv::riscv_csr::CsrAddr;
use softfloat_wrapper::{ExceptionFlags, Float, RoundingMode, F32};

impl EmuEnv {
    #[inline]
    fn convert_nan_boxing(i: u64) -> u32 {
        if i & 0xffffffff_00000000 == 0xffffffff_00000000 {
            (i & 0xffffffff) as u32
        } else {
            0x7fc00000
        }
    }

    pub fn helper_func_fadd_s(emu: &mut EmuEnv, fd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        println!("fadd(emu, {:}, {:}, {:}) is called!", fd, fs1, fs2);
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data.add(fs2_data, RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        return 0;
    }

    pub fn helper_func_fsub_s(emu: &mut EmuEnv, fd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
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

    pub fn helper_func_fmul_s(emu: &mut EmuEnv, fd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
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

    pub fn helper_func_fdiv_s(emu: &mut EmuEnv, fd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
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

    pub fn helper_func_fmadd_s(emu: &mut EmuEnv, fd: u64, fs1: u64, fs2: u64, fs3: u64) -> usize {
        println!(
            "fmadd(emu, {:}, {:}, {:}, {:}) is called!",
            fd, fs1, fs2, fs3
        );
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
        let fs3_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs3 as usize]) as u32);
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

    pub fn helper_func_fmsub_s(emu: &mut EmuEnv, fd: u64, fs1: u64, fs2: u64, fs3: u64) -> usize {
        println!(
            "fmsub(emu, {:}, {:}, {:}, {:}) is called!",
            fd, fs1, fs2, fs3
        );
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
        let fs3_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs3 as usize]) as u32);
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

    pub fn helper_func_fnmsub_s(emu: &mut EmuEnv, fd: u64, fs1: u64, fs2: u64, fs3: u64) -> usize {
        println!(
            "fnmsub(emu, {:}, {:}, {:}, {:}) is called!",
            fd, fs1, fs2, fs3
        );
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
        let fs3_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs3 as usize]) as u32);
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

    pub fn helper_func_fnmadd_s(emu: &mut EmuEnv, fd: u64, fs1: u64, fs2: u64, fs3: u64) -> usize {
        println!(
            "fnmadd(emu, {:}, {:}, {:}, {:}) is called!",
            fd, fs1, fs2, fs3
        );
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
        let fs3_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs3 as usize]) as u32);
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

    pub fn helper_func_fsqrt_s(emu: &mut EmuEnv, fd: u64, fs1: u64, _: u64, _: u64) -> usize {
        println!("fsqrt(emu, {:}, {:}) is called!", fd, fs1);

        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let mut flag = ExceptionFlags::default();
        flag.set();
        let fd_data = fs1_data.sqrt(RoundingMode::TiesToEven);
        flag.get();
        let ret_flag = flag.bits();

        emu.m_fregs[fd as usize] = fd_data.bits() as u64;
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);

        return 0;
    }

    pub fn helper_func_feq_s(emu: &mut EmuEnv, rd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_iregs[rd as usize] = fs1_data.eq(fs2_data) as u64;
        flag.get();
        let ret_flag = flag.bits();
        println!(
            "feq(emu, {:}, {:}, {:}) => {:} is called!",
            rd, fs1, fs2, ret_flag
        );
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }

    pub fn helper_func_flt_s(emu: &mut EmuEnv, rd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_iregs[rd as usize] = fs1_data.lt(fs2_data) as u64;
        flag.get();
        let ret_flag = flag.bits();
        println!(
            "flt(emu, {:}, {:}, {:}) is called! => {:}",
            rd, fs1, fs2, ret_flag
        );
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }

    pub fn helper_func_fle_s(emu: &mut EmuEnv, rd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        println!("fle(emu, {:}, {:}, {:}) is called!", rd, fs1, fs2);
        let fs1_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32);
        let fs2_data = F32::from_bits(Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32);
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_iregs[rd as usize] = fs1_data.le(fs2_data) as u64;
        flag.get();
        let ret_flag = flag.bits();
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }

    pub fn helper_func_fclass_s(emu: &mut EmuEnv, rd: u64, fs1: u64, _fs2: u64, _: u64) -> usize {
        println!("fclass_s(emu, {:}, {:}) is called!", rd, fs1);
        let fs1_data = F32::from_bits((emu.m_fregs[fs1 as usize] & 0x0ffffffff) as u32);
        #[allow(unused_assignments)]
        let mut result = 0;
        if fs1_data.is_negative_infinity() {
            result = 1 << 0;
        } else if fs1_data.is_positive_infinity() {
            result = 1 << 7;
        } else if fs1_data.is_negative_zero() {
            result = 1 << 3;
        } else if fs1_data.is_positive_zero() {
            result = 1 << 4;
        } else if fs1_data.is_negative_zero() || fs1_data.is_negative_subnormal() {
            result = 1 << 2;
        } else if fs1_data.is_positive_zero() || fs1_data.is_positive_subnormal() {
            result = 1 << 5;
        } else if fs1_data.is_nan() {
            if (fs1_data.exponent() == F32::EXPONENT_BIT)
                && (fs1_data.fraction() & (1 << (F32::EXPONENT_POS - 1))) != 0
            {
                result = 1 << 9;
            } else {
                result = 1 << 8;
            }
        } else if fs1_data.is_negative() {
            result = 1 << 1;
        } else {
            result = 1 << 6;
        }
        emu.m_iregs[rd as usize] = result as u64;
        return 0;
    }

    pub fn helper_func_fmax_s(emu: &mut EmuEnv, rd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        let fs1_data = F32::from_bits(emu.m_fregs[fs1 as usize] as u32);
        let fs2_data = F32::from_bits(emu.m_fregs[fs2 as usize] as u32);
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_fregs[rd as usize] = if fs1_data.is_nan() && fs2_data.is_nan() {
            F32::quiet_nan().bits() as u64
        } else if fs2_data.lt_quiet(fs1_data)
            || fs2_data.is_nan()
            || fs1_data.eq(fs2_data) && fs2_data.is_negative()
        {
            fs1_data.bits() as u64
        } else {
            fs2_data.bits() as u64
        };
        flag.get();
        let ret_flag = flag.bits();
        println!(
            "fmax_d(emu, {:}, {:}, {:}) is called! => {:}",
            rd, fs1, fs2, ret_flag
        );
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }

    pub fn helper_func_fmin_s(emu: &mut EmuEnv, rd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        let fs1_data = F32::from_bits(emu.m_fregs[fs1 as usize] as u32);
        let fs2_data = F32::from_bits(emu.m_fregs[fs2 as usize] as u32);
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_fregs[rd as usize] = if fs1_data.is_nan() && fs2_data.is_nan() {
            F32::quiet_nan().bits() as u64
        } else if fs1_data.lt_quiet(fs2_data)
            || fs2_data.is_nan()
            || fs1_data.eq(fs2_data) && fs1_data.is_negative()
        {
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

    pub fn helper_func_fsgnj_s(emu: &mut EmuEnv, rd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        let fs1_data = Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32;
        let fs2_data = Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32;
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_fregs[rd as usize] =
            (fs1_data & 0x7fffffff | fs2_data & 0x80000000) as u64 | 0xffffffff_00000000;
        flag.get();
        let ret_flag = flag.bits();
        println!("fsgnj_s(emu, {:}, {:}, {:}) is called!", rd, fs1, fs2);
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }

    pub fn helper_func_fsgnjn_s(emu: &mut EmuEnv, rd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        let fs1_data = Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32;
        let fs2_data = Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32;
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_fregs[rd as usize] =
            (fs1_data & 0x7fffffff | !fs2_data & 0x80000000) as u64 | 0xffffffff_00000000;
        flag.get();
        let ret_flag = flag.bits();
        println!("fsgnjn_s(emu, {:}, {:}, {:}) is called!", rd, fs1, fs2);
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }

    pub fn helper_func_fsgnjx_s(emu: &mut EmuEnv, rd: u64, fs1: u64, fs2: u64, _: u64) -> usize {
        let fs1_data = Self::convert_nan_boxing(emu.m_fregs[fs1 as usize]) as u32;
        let fs2_data = Self::convert_nan_boxing(emu.m_fregs[fs2 as usize]) as u32;
        let mut flag = ExceptionFlags::default();
        flag.set();
        emu.m_fregs[rd as usize] = (fs1_data & 0x7fffffff | (fs1_data ^ fs2_data) & 0x80000000)
            as u64
            | 0xffffffff_00000000;
        flag.get();
        let ret_flag = flag.bits();
        println!("fsgnjx_s(emu, {:}, {:}, {:}) is called!", rd, fs1, fs2);
        emu.m_csr.csrrw(CsrAddr::FFlags, ret_flag as i64);
        return 0;
    }
}
