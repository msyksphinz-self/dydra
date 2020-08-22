use memmap::Mmap;
use num::traits::FromPrimitive;
use std::env;
use std::fs::File;

// pub mod riscv_decoder;
// pub mod riscv_inst_id;
// pub mod riscv_inst_mnemonic;
// pub mod riscv_inst_operand;

// 0x7f 'E' 'L' 'F'
const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

/* 64bit architectures */

#[allow(non_camel_case_types)]
pub enum EType {
    ET_NONE = 0, /* No file type */
    ET_REL = 1,  /* Relocatable file */
    ET_EXEC = 2, /* Executable file */
    ET_DYN = 3,  /* Shared object file */
    ET_CORE = 4, /* Core file */
}

impl FromPrimitive for EType {
    fn from_i64(n: i64) -> Option<EType> {
        match n {
            0 => Some(EType::ET_NONE),
            1 => Some(EType::ET_REL),
            2 => Some(EType::ET_EXEC),
            3 => Some(EType::ET_DYN),
            4 => Some(EType::ET_CORE),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<EType> {
        match n {
            0 => Some(EType::ET_NONE),
            1 => Some(EType::ET_REL),
            2 => Some(EType::ET_EXEC),
            3 => Some(EType::ET_DYN),
            4 => Some(EType::ET_CORE),
            _ => None,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Phdr_Type {
    PT_NULL = 0,                  /* Program header table entry unused */
    PT_LOAD = 1,                  /* Loadable program segment */
    PT_DYNAMIC = 2,               /* Dynamic linking information */
    PT_INTERP = 3,                /* Program interpreter */
    PT_NOTE = 4,                  /* Auxiliary information */
    PT_SHLIB = 5,                 /* Reserved */
    PT_PHDR = 6,                  /* Entry for header table itself */
    PT_TLS = 7,                   /* Thread-local storage segment */
    PT_NUM = 8,                   /* Number of defined types */
    PT_LOOS = 0x60000000,         /* Start of OS-specific */
    PT_GNU_EH_FRAME = 0x6474e550, /* GCC .eh_frame_hdr segment */
    PT_GNU_STACK = 0x6474e551,    /* Indicates stack executability */
    PT_GNU_RELRO = 0x6474e552,    /* Read-only after relocation */
    // PT_LOSUNW    = 0x6ffffffa,
    PT_SUNWBSS = 0x6ffffffa,   /* Sun Specific segment */
    PT_SUNWSTACK = 0x6ffffffb, /* Stack segment */
    // PT_HISUNW    = 0x6fffffff,
    PT_HIOS = 0x6fffffff,   /* End of OS-specific */
    PT_LOPROC = 0x70000000, /* Start of processor-specific */
    PT_HIPROC = 0x7fffffff, /* End of processor-specific */
}

impl FromPrimitive for Phdr_Type {
    fn from_i64(n: i64) -> Option<Phdr_Type> {
        match n {
            0 => Some(Phdr_Type::PT_NULL),
            1 => Some(Phdr_Type::PT_LOAD),
            2 => Some(Phdr_Type::PT_DYNAMIC),
            3 => Some(Phdr_Type::PT_INTERP),
            4 => Some(Phdr_Type::PT_NOTE),
            5 => Some(Phdr_Type::PT_SHLIB),
            6 => Some(Phdr_Type::PT_PHDR),
            7 => Some(Phdr_Type::PT_TLS),
            8 => Some(Phdr_Type::PT_NUM),
            0x60000000 => Some(Phdr_Type::PT_LOOS),
            0x6474e550 => Some(Phdr_Type::PT_GNU_EH_FRAME),
            0x6474e551 => Some(Phdr_Type::PT_GNU_STACK),
            0x6474e552 => Some(Phdr_Type::PT_GNU_RELRO),
            0x6ffffffa => Some(Phdr_Type::PT_SUNWBSS),
            0x6ffffffb => Some(Phdr_Type::PT_SUNWSTACK),
            0x6fffffff => Some(Phdr_Type::PT_HIOS),
            0x70000000 => Some(Phdr_Type::PT_LOPROC),
            0x7fffffff => Some(Phdr_Type::PT_HIPROC),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Phdr_Type> {
        match n {
            0 => Some(Phdr_Type::PT_NULL),
            1 => Some(Phdr_Type::PT_LOAD),
            2 => Some(Phdr_Type::PT_DYNAMIC),
            3 => Some(Phdr_Type::PT_INTERP),
            4 => Some(Phdr_Type::PT_NOTE),
            5 => Some(Phdr_Type::PT_SHLIB),
            6 => Some(Phdr_Type::PT_PHDR),
            7 => Some(Phdr_Type::PT_TLS),
            8 => Some(Phdr_Type::PT_NUM),
            0x60000000 => Some(Phdr_Type::PT_LOOS),
            0x6474e550 => Some(Phdr_Type::PT_GNU_EH_FRAME),
            0x6474e551 => Some(Phdr_Type::PT_GNU_STACK),
            0x6474e552 => Some(Phdr_Type::PT_GNU_RELRO),
            0x6ffffffa => Some(Phdr_Type::PT_SUNWBSS),
            0x6ffffffb => Some(Phdr_Type::PT_SUNWSTACK),
            0x6fffffff => Some(Phdr_Type::PT_HIOS),
            0x70000000 => Some(Phdr_Type::PT_LOPROC),
            0x7fffffff => Some(Phdr_Type::PT_HIPROC),
            _ => None,
        }
    }
}

pub struct ELFHeader {
    pub e_type: EType,    /* Object file type */
    pub e_machine: u16,   /* Architecture */
    pub e_version: u32,   /* Object file version */
    pub e_entry: u64,     /* Entry point virtual address */
    pub e_phoff: u64,     /* Program header table file offset */
    pub e_shoff: u64,     /* Section header table file offset */
    pub e_flags: u32,     /* Processor-specific flags */
    pub e_ehsize: u16,    /* ELF header size in bytes */
    pub e_phentsize: u16, /* Program header table entry size */
    pub e_phnum: u16,     /* Program header table entry count */
    pub e_shentsize: u16, /* Section header table entry size */
    pub e_shnum: u16,     /* Section header table entry count */
    pub e_shstrndx: u16,  /* Section header string table index */
}

impl ELFHeader {
    pub fn new(
        e_type: u16,
        e_machine: u16,
        e_version: u32,
        e_entry: u64,
        e_phoff: u64,
        e_shoff: u64,
        e_flags: u32,
        e_ehsize: u16,
        e_phentsize: u16,
        e_phnum: u16,
        e_shentsize: u16,
        e_shnum: u16,
        e_shstrndx: u16,
    ) -> ELFHeader {
        let e_type_enum = match EType::from_u64(e_type as u64) {
            Some(e_type) => e_type,
            None => panic!("Unknown EI Type"),
        };

        ELFHeader {
            e_type: e_type_enum,
            e_machine: e_machine,
            e_version: e_version,
            e_entry: e_entry,
            e_phoff: e_phoff,
            e_shoff: e_shoff,
            e_flags: e_flags,
            e_ehsize: e_ehsize,
            e_phentsize: e_phentsize,
            e_phnum: e_phnum,
            e_shentsize: e_shentsize,
            e_shnum: e_shnum,
            e_shstrndx: e_shstrndx,
        }
    }

    pub fn dump(&self) {
        println!("\n");
        println!("E_TYPE      = {}", self.get_e_type_string());
        println!("E_MACHINE   = {}", self.get_e_machine_string());
        println!("E_VERSION   = {}", self.e_version);
        println!("E_ENTRY     = 0x{:x}", self.e_entry);
        println!("E_PHOFF     = 0x{:x}", self.e_phoff);
        println!("E_SHOFF     = 0x{:x}", self.e_shoff);
        println!("E_FLAGS     = {}", self.e_flags);
        println!("E_EHSIZE    = {}", self.e_ehsize);
        println!("E_PHENTSIZE = {}", self.e_phentsize);
        println!("E_PHNUM     = {}", self.e_phnum);
        println!("E_SHENTSIZE = {}", self.e_shentsize);
        println!("E_SHNUM     = {}", self.e_shnum);
        println!("E_SHSTRNDX  = {}", self.e_shstrndx);
    }

    fn get_e_type_string(&self) -> String {
        match self.e_type {
            EType::ET_NONE => String::from("ET_NONE"),
            EType::ET_REL => String::from("ET_REL"),
            EType::ET_EXEC => String::from("ET_EXEC"),
            EType::ET_DYN => String::from("ET_DYN"),
            EType::ET_CORE => String::from("ET_CORE"),
        }
    }

    fn get_e_machine_string(&self) -> String {
        match self.e_machine {
            EM_RISCV => String::from("RISCV"),
            EM_X86_64 => String::from("X86_64"),
            _ => panic!("Unknown ELF type"),
        }
    }
}

pub struct ProgramHeader<'a> {
    elf_header: &'a ELFHeader,

    pub p_type: Phdr_Type, /* entry type */
    pub p_flags: u32,      /* flags */
    pub p_offset: u64,     /* offset */
    pub p_vaddr: u64,      /* virtual address */
    pub p_paddr: u64,      /* physical address */
    pub p_filesz: u64,     /* file size */
    pub p_memsz: u64,      /* memory size */
    pub p_align: u64,      /* memory & file alignment */
}

impl<'a> ProgramHeader<'a> {
    pub fn new(
        elf_header: &'a ELFHeader,
        p_type: Phdr_Type,
        p_flags: u32,
        p_offset: u64,
        p_vaddr: u64,
        p_paddr: u64,
        p_filesz: u64,
        p_memsz: u64,
        p_align: u64,
    ) -> ProgramHeader {
        ProgramHeader {
            elf_header: elf_header,
            p_type: p_type,
            p_flags: p_flags,
            p_offset: p_offset,
            p_vaddr: p_vaddr,
            p_paddr: p_paddr,
            p_filesz: p_filesz,
            p_memsz: p_memsz,
            p_align: p_align,
        }
    }

    pub fn get_type_string(&self) -> String {
        match self.p_type {
            Phdr_Type::PT_NULL => String::from("PT_NULL"),
            Phdr_Type::PT_LOAD => String::from("PT_LOAD"),
            Phdr_Type::PT_DYNAMIC => String::from("PT_DYNAMIC"),
            Phdr_Type::PT_INTERP => String::from("PT_INTERP"),
            Phdr_Type::PT_NOTE => String::from("PT_NOTE"),
            Phdr_Type::PT_SHLIB => String::from("PT_SHLIB"),
            Phdr_Type::PT_PHDR => String::from("PT_PHDR"),
            Phdr_Type::PT_TLS => String::from("PT_TLS"),
            Phdr_Type::PT_NUM => String::from("PT_NUM"),
            Phdr_Type::PT_LOOS => String::from("PT_LOOS"),
            Phdr_Type::PT_GNU_EH_FRAME => String::from("PT_GNU_EH_FRAME"),
            Phdr_Type::PT_GNU_STACK => String::from("PT_GNU_STACK"),
            Phdr_Type::PT_GNU_RELRO => String::from("PT_GNU_RELRO"),
            // Phdr_Type::PT_LOSUNW        => String::from("PT_LOSUNW"),
            Phdr_Type::PT_SUNWBSS => String::from("PT_SUNWBSS"),
            Phdr_Type::PT_SUNWSTACK => String::from("PT_SUNWSTACK"),
            // Phdr_Type::PT_HISUNW        => String::from("PT_HISUNW"),
            Phdr_Type::PT_HIOS => String::from("PT_HIOS"),
            Phdr_Type::PT_LOPROC => String::from("PT_LOPROC"),
            Phdr_Type::PT_HIPROC => String::from("PT_HIPROC"),
        }
    }

    pub fn dump(&self) {
        println!("== Program Header Dump ==");

        println!("  Entry Type  : {}", self.get_type_string());
        println!("  Flags       : 0x{:x}", self.p_flags);
        println!("  Offset      : 0x{:x}", self.p_offset);
        println!("  VAddr       : 0x{:x}", self.p_vaddr);
        println!("  PAddr       : 0x{:x}", self.p_paddr);
        println!("  File Size   : 0x{:x}", self.p_filesz);
        println!("  Memory Size : 0x{:x}", self.p_memsz);
        println!("  Alignment   : 0x{:x}", self.p_align);
    }
}

pub struct SectionHeader<'a> {
    elf_header: &'a ELFHeader,

    pub sh_name: u32,      /* Section name (string tbl index) */
    pub sh_type: u32,      /* Section type */
    pub sh_flags: u64,     /* Section flags */
    pub sh_addr: u64,      /* Section virtual addr at execution */
    pub sh_offset: u64,    /* Section file offset */
    pub sh_size: u64,      /* Section size in bytes */
    pub sh_link: u32,      /* Link to another section */
    pub sh_info: u32,      /* Additional section information */
    pub sh_addralign: u64, /* Section alignment */
    pub sh_entsize: u64,   /* Entry size if section holds table */
}

impl<'a> SectionHeader<'a> {
    pub fn new(
        elf_header: &'a ELFHeader,
        sh_name: u32,
        sh_type: u32,
        sh_flags: u64,
        sh_addr: u64,
        sh_offset: u64,
        sh_size: u64,
        sh_link: u32,
        sh_info: u32,
        sh_addralign: u64,
        sh_entsize: u64,
    ) -> SectionHeader {
        SectionHeader {
            elf_header: elf_header,
            sh_name: sh_name,
            sh_type: sh_type,
            sh_flags: sh_flags,
            sh_addr: sh_addr,
            sh_offset: sh_offset,
            sh_size: sh_size,
            sh_link: sh_link,
            sh_info: sh_info,
            sh_addralign: sh_addralign,
            sh_entsize: sh_entsize,
        }
    }

    pub fn dump(&self) {
        println!("== Section Dump ==");
        println!("  Name      : {:x}", self.sh_name);

        println!("  Type      : 0x{:x}", self.sh_type);
        println!("  Flags     : 0x{:x}", self.sh_flags);
        println!("  Addr      : 0x{:x}", self.sh_addr);
        println!("  Offset    : 0x{:x}", self.sh_offset);
        println!("  Size      : 0x{:x}", self.sh_size);
        println!("  Link      : 0x{:x}", self.sh_link);
        println!("  Info      : 0x{:x}", self.sh_info);
        println!("  AddrAlign : 0x{:x}", self.sh_addralign);
        println!("  EntSize   : 0x{:x}", self.sh_entsize);
    }
}

const EM_X86_64: u16 = 62;
const EM_RISCV: u16 = 243;

pub struct ELFLoader {
    mapped_file: Mmap,
}

impl ELFLoader {
    pub fn new(file_path: &str) -> std::io::Result<ELFLoader> {
        let file = File::open(&file_path)?;
        Ok(ELFLoader {
            mapped_file: unsafe { Mmap::map(&file)? },
        })
    }

    fn get_1byte_elf(&self, start: usize) -> u8 {
        self.mapped_file[start + 0]
    }

    fn get_2byte_elf(&self, start: usize) -> u16 {
        (self.mapped_file[start + 1] as u16) << 8 | (self.mapped_file[start + 0] as u16)
    }
    fn get_4byte_elf(&self, start: usize) -> u32 {
        (self.mapped_file[start + 3] as u32) << 24
            | (self.mapped_file[start + 2] as u32) << 16
            | (self.mapped_file[start + 1] as u32) << 8
            | (self.mapped_file[start + 0] as u32) << 0
    }
    fn get_8byte_elf(&self, start: usize) -> u64 {
        (self.mapped_file[start + 7] as u64) << 56
            | (self.mapped_file[start + 6] as u64) << 48
            | (self.mapped_file[start + 5] as u64) << 40
            | (self.mapped_file[start + 4] as u64) << 32
            | (self.mapped_file[start + 3] as u64) << 24
            | (self.mapped_file[start + 2] as u64) << 16
            | (self.mapped_file[start + 1] as u64) << 8
            | (self.mapped_file[start + 0] as u64) << 0
    }

    pub fn get_elf_header(&self) -> ELFHeader {
        if self.mapped_file[0..4] != HEADER_MAGIC {
            panic!("Not ELF File");
        }

        let mut elf_off = 16;

        let e_type = self.get_2byte_elf(elf_off as usize);
        elf_off += 2;
        let e_machine = self.get_2byte_elf(elf_off as usize);
        elf_off += 2;
        let e_version = self.get_4byte_elf(elf_off as usize);
        elf_off += 4;
        let e_entry = self.get_8byte_elf(elf_off as usize);
        elf_off += 8;
        let e_phoff = self.get_8byte_elf(elf_off as usize);
        elf_off += 8;
        let e_shoff = self.get_8byte_elf(elf_off as usize);
        elf_off += 8;
        let e_flags = self.get_4byte_elf(elf_off as usize);
        elf_off += 4;
        let e_ehsize = self.get_2byte_elf(elf_off as usize);
        elf_off += 2;
        let e_phentsize = self.get_2byte_elf(elf_off as usize);
        elf_off += 2;
        let e_phnum = self.get_2byte_elf(elf_off as usize);
        elf_off += 2;
        let e_shentsize = self.get_2byte_elf(elf_off as usize);
        elf_off += 2;
        let e_shnum = self.get_2byte_elf(elf_off as usize);
        elf_off += 2;
        let e_shstrndx = self.get_2byte_elf(elf_off as usize); // elf_off += 2;

        ELFHeader::new(
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        )
    }

    pub fn get_program_header<'a>(
        &self,
        elf_header: &'a ELFHeader,
        e_phoff: u64,
        e_phentsize: u16,
        idx: u32,
    ) -> ProgramHeader<'a> {
        let mut ph_off = e_phoff;
        let ph_size = e_phentsize as u32;

        ph_off += (ph_size * idx) as u64;

        let p_type = self.get_4byte_elf(ph_off as usize);
        ph_off += 4;
        let p_flags = self.get_4byte_elf(ph_off as usize);
        ph_off += 4;
        let p_offset = self.get_8byte_elf(ph_off as usize);
        ph_off += 8;
        let p_vaddr = self.get_8byte_elf(ph_off as usize);
        ph_off += 8;
        let p_paddr = self.get_8byte_elf(ph_off as usize);
        ph_off += 8;
        let p_filesz = self.get_8byte_elf(ph_off as usize);
        ph_off += 8;
        let p_memsz = self.get_8byte_elf(ph_off as usize);
        ph_off += 8;
        let p_align = self.get_8byte_elf(ph_off as usize); // ph_off += 8;

        let phdr_type = match Phdr_Type::from_u64(p_type as u64) {
            Some(phdr_type) => phdr_type,
            None => panic!("Unknown Phdr Type"),
        };

        ProgramHeader::new(
            elf_header, phdr_type, p_flags, p_offset, p_vaddr, p_paddr, p_filesz, p_memsz, p_align,
        )
    }

    pub fn get_section_header<'a>(
        &self,
        elf_header: &'a ELFHeader,
        e_shoff: u64,
        e_shentsize: u16,
        idx: u32,
    ) -> SectionHeader<'a> {
        let mut sh_off = e_shoff;
        let sh_entsize = e_shentsize as u32;

        sh_off += (sh_entsize * idx) as u64;

        let sh_name = self.get_4byte_elf(sh_off as usize);
        sh_off += 4;
        let sh_type = self.get_4byte_elf(sh_off as usize);
        sh_off += 4;
        let sh_flags = self.get_8byte_elf(sh_off as usize);
        sh_off += 8;
        let sh_addr = self.get_8byte_elf(sh_off as usize);
        sh_off += 8;
        let sh_offset = self.get_8byte_elf(sh_off as usize);
        sh_off += 8;
        let sh_size = self.get_8byte_elf(sh_off as usize);
        sh_off += 8;
        let sh_link = self.get_4byte_elf(sh_off as usize);
        sh_off += 4;
        let sh_info = self.get_4byte_elf(sh_off as usize);
        sh_off += 4;
        let sh_addralign = self.get_8byte_elf(sh_off as usize);
        sh_off += 8;
        let sh_entsize = self.get_8byte_elf(sh_off as usize); // sh_off += 8;

        SectionHeader::new(
            elf_header,
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize.into(),
        )
    }

    pub fn load_section(&self, v: &mut Vec<u8>, start: u64, memsz: u64) {
        for byte_idx in start..(start + memsz) {
            let inst_byte: u8 = self.get_1byte_elf(byte_idx as usize);
            v.push(inst_byte);
        }
    }

    //    fn dump_section(&self, tracer: &Tracer, start: u64, memsz: u64) {
    //        for byte_idx in (start..(start + memsz)).step_by(4) {
    //            let inst_hex: u32 = self.get_4byte_elf(byte_idx as usize);
    //
    //            print!("{:08x} ", inst_hex);
    //
    //            let inst_decode = RiscvDecoder::decode_inst(inst_hex);
    //
    //            match inst_decode {
    //                Some(id) => {
    //                    let inst_str = get_riscv_inst_mnemonic(id);
    //                    let operand_info = tracer.m_inst_operand_map.get(&id);
    //                    match operand_info {
    //                        Some(operand_info) => {
    //                            let mut at_index = 0;
    //                            let mut consume_idx = 0;
    //                            let mut idx = 0;
    //                            while idx < inst_str.len() {
    //                                if inst_str.chars().nth(idx) == Some('@') {
    //                                    let mut opr_val = 0;
    //                                    let mut max_msb = 0;
    //                                    let mut min_lsb = 32;
    //                                    loop {
    //                                        let msb = operand_info.m_msb_lst[at_index];
    //                                        let lsb = operand_info.m_lsb_lst[at_index];
    //                                        max_msb = cmp::max(max_msb, msb);
    //                                        min_lsb = cmp::min(min_lsb, lsb);
    //
    //                                        let mask = (1 << (msb - lsb + 1)) - 1;
    //                                        opr_val =
    //                                            opr_val << (msb - lsb + 1) | (inst_hex >> lsb) & mask;
    //                                        if !operand_info.m_connect[at_index] {
    //                                            break;
    //                                        }
    //                                        at_index += 1;
    //                                    }
    //                                    let mut shift_val = 0;
    //                                    if inst_str.chars().nth(idx + 1) == Some('<')
    //                                        && inst_str.chars().nth(idx + 2) == Some('<')
    //                                    {
    //                                        idx += 3;
    //                                        while match inst_str.chars().nth(idx) {
    //                                            Some(c) => c.is_digit(10),
    //                                            _ => false,
    //                                        } {
    //                                            shift_val <<= 10;
    //                                            shift_val += match inst_str.chars().nth(idx) {
    //                                                Some(c) => match c.to_digit(10) {
    //                                                    Some(d) => d,
    //                                                    None => 0,
    //                                                },
    //                                                None => 0,
    //                                            };
    //                                            idx += 1;
    //                                        }
    //                                    }
    //                                    opr_val <<= shift_val;
    //
    //                                    match operand_info.m_type_lst[at_index] {
    //                                        OperandType::TypeXReg => {
    //                                            print!("x{:}", opr_val);
    //                                            consume_idx = consume_idx + 3;
    //                                        }
    //                                        OperandType::TypeFreg => {
    //                                            print!("f{:}", opr_val);
    //                                            consume_idx = consume_idx + 3;
    //                                        }
    //                                        OperandType::TypeSign => {
    //                                            print!("{:}", opr_val);
    //                                            consume_idx =
    //                                                consume_idx + ((opr_val as f32).log10() as u32);
    //                                        }
    //                                        OperandType::TypeBit => {
    //                                            print!("0b{:b}", opr_val);
    //                                            consume_idx = consume_idx + (max_msb - min_lsb + 1);
    //                                        }
    //                                        OperandType::TypeUnSign => {
    //                                            print!("0x{:x}", opr_val);
    //                                            consume_idx =
    //                                                consume_idx + 2 + ((opr_val as f32).log10() as u32);
    //                                        }
    //                                        OperandType::TypeUnSignJ => {
    //                                            print!("0x{:x}", opr_val);
    //                                            consume_idx =
    //                                                consume_idx + 2 + ((opr_val as f32).log10() as u32);
    //                                        }
    //                                        OperandType::TypeSignBit => {
    //                                            print!("0b{:b}", opr_val);
    //                                            consume_idx = consume_idx + 2 + (max_msb - min_lsb + 1);
    //                                        }
    //                                        OperandType::TypeHex => {
    //                                            let bit_width =
    //                                                ((max_msb as f32 - min_lsb as f32 + 1.0) / 4.0)
    //                                                    .ceil()
    //                                                    as u32;
    //                                            print!(
    //                                                "0x{:0>width$x}",
    //                                                opr_val,
    //                                                width = bit_width as usize
    //                                            );
    //                                            consume_idx = consume_idx + 2 + bit_width;
    //                                        }
    //                                        OperandType::TypeRoundMode => {
    //                                            panic!("TypeRoundMode is currently not supported")
    //                                        }
    //                                        OperandType::TypeCompactReg => {
    //                                            panic!("TypeCompactReg is currently not supported")
    //                                        }
    //                                        OperandType::TypeCompactFReg => {
    //                                            panic!("TypeCompactFReg is currently not supported")
    //                                        }
    //                                        _ => panic!(
    //                                            "Unknown operand type {:?}",
    //                                            operand_info.m_type_lst[at_index] as u32
    //                                        ),
    //                                    }
    //                                    at_index = at_index + 1;
    //                                    idx += 1;
    //                                } else {
    //                                    match inst_str.chars().nth(idx) {
    //                                        Some(c) => {
    //                                            print!("{:}", c);
    //                                        }
    //                                        _ => {}
    //                                    }
    //                                    idx += 1;
    //                                    consume_idx = consume_idx + 1
    //                                }
    //                            }
    //                        }
    //                        None => {
    //                            panic!("Implementation Error: No operand information in this inst.")
    //                        }
    //                    }
    //                }
    //                _ => {}
    //            }
    //
    //            // if byte_idx % 16 == 16-4 {
    //            //     print!("\n");
    //            // }
    //            print!("\n");
    //        }
    //    }
}

// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let filename = &args[1];
//     let loader = match ELFLoader::new(filename) {
//         Ok(loader) => loader,
//         Err(error) => panic!("There was a problem opening the file: {:?}", error),
//     };
//
//     let elf_header = loader.get_elf_header();
//     elf_header.dump();
//
//     let mut ph_headers = Vec::new();
//     for ph_idx in 0..elf_header.e_phnum {
//         let phdr: ProgramHeader = loader.get_program_header(
//             &elf_header,
//             elf_header.e_phoff,
//             elf_header.e_phentsize,
//             ph_idx.into(),
//         );
//         ph_headers.push(phdr);
//     }
//
//     let mut sh_headers = Vec::new();
//     for sh_idx in 0..elf_header.e_shnum {
//         let shdr: SectionHeader = loader.get_section_header(
//             &elf_header,
//             elf_header.e_shoff,
//             elf_header.e_shentsize,
//             sh_idx.into(),
//         );
//         sh_headers.push(shdr);
//     }
//
//     let mut tracer = Tracer {
//         m_inst_operand_map: HashMap::new(),
//     };
//
//     tracer.format_operand();
//
//     // Dump All Program Headers
//     for ph_header in ph_headers {
//         ph_header.dump();
//     }
//
//     // Dump All Section Headers
//     for sh_header in sh_headers {
//         sh_header.dump();
//         loader.dump_section(&tracer, sh_header.sh_offset, sh_header.sh_size);
//     }
// }
