use std::fmt;

pub struct Header<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex
{
    elf_class: Class,
    endian: Endian,
    target_abi: TargetABI,
    abi_version: u8,
    object_type: ObjectFileType,
    target_isa: ISA,
    entry_point: T,
    program_header_offset: T,
    section_header_offset: T,
    flags: u32,
    header_size: u16,
    program_header_size: u16,
    program_header_number: u16,
    section_header_size: u16,
    section_header_number: u16,
    section_name_table_entry: u16,
}

impl<T> Header<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex
{
    pub fn get_elf_class(binary: &Vec<u8>) -> Option<Class> {
        match binary[0x4] {
            1 => Some(Class::ELF32),
            2 => Some(Class::ELF64),
            _ => return None,
        }
    }
}

impl Header<u32> {
    pub fn new (binary: &Vec<u8>) -> Option<Header<u32>> {
        construct(binary, |bin, offset, is_little| {
            let value = make_u32(&bin[offset..offset + 4], is_little);
            (value, offset + 4)
        })
    }
}

impl Header<u64> {
    pub fn new(binary: &Vec<u8>) -> Option<Header<u64>> {
        construct(binary, |bin, offset, is_little| {
            let value = make_u64(&bin[offset..offset + 8], is_little);
            (value, offset + 8)
        })
    }
}

fn construct<T>(binary: &Vec<u8>, truncator: impl Fn(&Vec<u8>, usize, bool) -> (T, usize)) -> Option<Header<T>>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex
{
    let magic = make_u32(&binary[0x0..0x4], true);

    if magic != 0x46_4C_45_7F { return None }
    if binary[0x6] != 1 { return None }
    if binary[0x14] != 1 { return None }

    let elf_class = match binary[0x4] {
        1 => Class::ELF32,
        2 => Class::ELF64,
        _ => return None,
    };

    let endian = match binary[0x5] {
        1 => Endian::Little,
        2 => Endian::Big,
        _ => return None,
    };

    let is_little = endian == Endian::Little;

    let target_abi = TargetABI::new(binary[0x7])?;
    let abi_version = binary[0x8];
    let object_type = ObjectFileType::new(make_u16(&binary[0x10..0x12], is_little))?;
    let target_isa = ISA::new(make_u16(&binary[0x12..0x14], is_little))?;

    let offset = 0x18;
    let (entry_point, offset) = truncator(binary, offset, is_little);
    let (program_header_offset, offset) = truncator(binary, offset, is_little);
    let (section_header_offset, offset) = truncator(binary, offset, is_little);

    let flags = make_u32(&binary[offset..offset + 4], is_little);
    let offset = offset + 4;
    let header_size = make_u16(&binary[offset..offset + 2], is_little);
    let offset = offset + 2;
    let program_header_size = make_u16(&binary[offset..offset + 2], is_little);
    let offset = offset + 2;
    let program_header_number = make_u16(&binary[offset..offset + 2], is_little);
    let offset = offset + 2;
    let section_header_size = make_u16(&binary[offset..offset + 2], is_little);
    let offset = offset + 2;
    let section_header_number = make_u16(&binary[offset..offset + 2], is_little);
    let offset = offset + 2;
    let section_name_table_entry = make_u16(&binary[offset..offset + 2], is_little);

    Some(Header {
        elf_class,
        endian,
        target_abi,
        abi_version,
        object_type,
        target_isa,
        entry_point,
        program_header_offset,
        section_header_offset,
        flags,
        header_size,
        program_header_size,
        program_header_number,
        section_header_size,
        section_header_number,
        section_name_table_entry,
    })
}

impl<T> fmt::Display for Header<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = format!("{:?}", self.elf_class);
        let endian = format!("{:?}", self.endian);
        let abi = format!("{:?}", self.target_abi);
        let abi_version = self.abi_version;
        let object_type = format!("{:?}", self.object_type);
        let isa = format!("{:?}", self.target_isa);

        let header_format = format!("
        Class:                             {}
        Endian:                            {}
        ABI:                               {}
        ABI Version:                       {}
        Type:                              {}
        ISA:                               {}
        Entry Point Address:               0x{:x}
        Start of program headers:          0x{:x}
        Start of section headers:          0x{:x}
        Flags:                             0x{:x}
        Size of this header:               0x{:x}
        Size of program headers:           0x{:x}
        Number of program headers:         {}
        Size of section headers:           0x{:x}
        Number of section headers:         {}
        Section header string table index: 0x{:x}
        ",
                                    class,
                                    endian,
                                    abi,
                                    abi_version,
                                    object_type,
                                    isa,
                                    self.entry_point,
                                    self.program_header_offset,
                                    self.section_header_offset,
                                    self.flags,
                                    self.header_size,
                                    self.program_header_size,
                                    self.program_header_number,
                                    self.section_header_size,
                                    self.section_header_number,
                                    self.section_name_table_entry,
        );

        write!(f, "{}", header_format)
    }
}

#[derive(Debug)]
pub enum Class {
    ELF32,
    ELF64,
}

#[derive(Debug, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum TargetABI {
    SystemV,
    HP_UX,
    NetBSD,
    Linux,
    GNUHard,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    Tru64,
    NovellModestro,
    OpenBSD,
    OpenVMS,
    NonStopKernel,
    AROS,
    FenixOS,
    CloudABI,
}

impl TargetABI {
    pub fn new(value: u8) -> Option<TargetABI> {
        let abi = match value {
            0x00 => TargetABI::SystemV,
            0x01 => TargetABI::HP_UX,
            0x02 => TargetABI::NetBSD,
            0x03 => TargetABI::Linux,
            0x04 => TargetABI::GNUHard,
            0x06 => TargetABI::Solaris,
            0x07 => TargetABI::AIX,
            0x08 => TargetABI::IRIX,
            0x09 => TargetABI::FreeBSD,
            0x0A => TargetABI::Tru64,
            0x0B => TargetABI::NovellModestro,
            0x0C => TargetABI::OpenBSD,
            0x0D => TargetABI::OpenVMS,
            0x0E => TargetABI::NonStopKernel,
            0x0F => TargetABI::AROS,
            0x10 => TargetABI::FenixOS,
            0x11 => TargetABI::CloudABI,
            _    => return None,
        };

        Some(abi)
    }
}

#[derive(Debug)]
pub enum ObjectFileType {
    NONE,
    REL,
    EXEC,
    DYN,
    CORE,
    LOOS,
    HIOS,
    LOPROC,
    HIPROC,
}

impl ObjectFileType {
    pub fn new(value: u16) -> Option<ObjectFileType> {
        let file_type = match value {
            0x0000 => ObjectFileType::NONE,
            0x0001 => ObjectFileType::REL,
            0x0002 => ObjectFileType::EXEC,
            0x0003 => ObjectFileType::DYN,
            0x0004 => ObjectFileType::CORE,
            0xFE00 => ObjectFileType::LOOS,
            0xFEFF => ObjectFileType::HIOS,
            0xFF00 => ObjectFileType::LOPROC,
            0xFFFF => ObjectFileType::HIPROC,
            _      => return None,
        };

        Some(file_type)
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ISA {
    NONE,
    SPARC,
    x86,
    MIPS,
    PowerPC,
    S390,
    ARM,
    SuperH,
    IA_64,
    x86_64,
    AArch64,
    RISC_V,
}

impl ISA {
    pub fn new(value: u16) -> Option<ISA> {
        let isa = match value {
            0x00 => ISA::NONE,
            0x02 => ISA::SPARC,
            0x03 => ISA::x86,
            0x08 => ISA::MIPS,
            0x14 => ISA::PowerPC,
            0x16 => ISA::S390,
            0x28 => ISA::ARM,
            0x2A => ISA::SuperH,
            0x32 => ISA::IA_64,
            0x3E => ISA::x86_64,
            0xB7 => ISA::AArch64,
            0xF3 => ISA::RISC_V,
            _    => return None,
        };

        Some(isa)
    }
}

fn make_u16(values: &[u8], is_little_endian: bool) -> u16 {
    if is_little_endian {
        (values[0] as u16) | ((values[1] as u16) << 8)
    } else {
        (values[1] as u16) | ((values[0] as u16) << 8)
    }
}

fn make_u32(values: &[u8], is_little_endian: bool) -> u32 {
    let (v0, v1, v2, v3) =
        if is_little_endian {
            (values[0], values[1], values[2], values[3])
        } else {
            (values[3], values[2], values[1], values[0])
        };

    (v0 as u32) | ((v1 as u32) << 8) | ((v2 as u32) << 16) | ((v3 as u32) << 24)
}

fn make_u64(values: &[u8], is_little_endian: bool) -> u64 {
    let values =
        if is_little_endian { values.to_vec() }
        else { values.iter().rev().cloned().collect() };

    values.iter().zip(0..).fold(0, |acc, (&v, index)| {
        acc | ((v as u64) << (index * 8))
    })
}