use std::fs;
use std::fmt;

struct Header {
    is_32bit: bool,
    is_little_endian: bool,
    version: u8,
    target_abi: TargetABI,
    abi_version: u8,
    object_type: ObjectFileType,    
    target_isa: ISA,
    entry_point: u64,
    program_header_offset: u64,
    section_header_offset: u64,
    flags: u32,
    header_size: u16,
    program_header_size: u16,
    program_header_number: u16,
    section_header_size: u16,
    section_header_number: u16,
    section_name_table_index: u16,    
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = format!("ELF{}", if self.is_32bit { 32 } else { 64 });
        let endian = if self.is_little_endian { "little endian" } else { "big endian" };
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
        self.section_name_table_index
        );

        write!(f, "{}", header_format)
    } 
}

#[derive(Debug)]
enum TargetABI {
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
    fn new(value: u8) -> Option<TargetABI> {
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
enum ObjectFileType {
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
    fn new(value: u16) -> Option<ObjectFileType> {
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
enum ISA {
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
    fn new(value: u16) -> Option<ISA> {
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



fn main() {
    let binary = read_binary("a.out");
    if let Some(header) = parse_header(&binary) {
        println!("{}", header);
    }
}

fn read_binary(filename: &str) -> Vec<u8> {
    match fs::read(filename) {
        Ok(bins) => bins,
        Err(e) => panic!(e),
    }
}

fn parse_header(binary: &Vec<u8>) -> Option<Header> {
    let magic = {
        let first = binary[0x0] as u32;
        let second = binary[0x1] as u32;
        let third = binary[0x2] as u32;
        let forth = binary[0x3] as u32;

        first | (second << 8) | (third << 16) | (forth << 24)
    };

    if magic != 0x46_4c_45_7f { return None; }
    let is_little_endian = match binary[0x5] {
        1 => true,
        2 => false,
        _ => return None,
    };

    match binary[0x4] {        
        1 => None,
        2 => parse_64bit_header(binary, is_little_endian),
        _ => None,
    }
}

fn parse_64bit_header(binary: &Vec<u8>, is_little_endian: bool) -> Option<Header> {    
    if binary[0x6] != 1 { return None };

    let target_abi = TargetABI::new(binary[0x7])?;
    let abi_version = binary[0x8];
    let object_type = ObjectFileType::new(make_u16(&binary[0x10..0x12], is_little_endian))?;
    let target_isa = ISA::new(make_u16(&binary[0x12..0x14], is_little_endian))?;

    if make_u32(&binary[0x14..0x18], is_little_endian) != 1 { return None }

    let entry_point = make_u64(&binary[0x18..0x20], is_little_endian);
    let program_header_offset = make_u64(&binary[0x20..0x28], is_little_endian);
    let section_header_offset = make_u64(&binary[0x28..0x30], is_little_endian);

    let flags = make_u32(&binary[0x30..0x34], is_little_endian);
    let header_size = make_u16(&binary[0x34..0x36], is_little_endian);
    let program_header_size = make_u16(&binary[0x36..0x38], is_little_endian);
    let program_header_number = make_u16(&binary[0x38..0x3A], is_little_endian);
    let section_header_size = make_u16(&binary[0x3A..0x3C], is_little_endian);
    let section_header_number = make_u16(&binary[0x3C..0x3E], is_little_endian);
    let section_name_table_index = make_u16(&binary[0x3E..0x40], is_little_endian);

    Some(Header {
        is_32bit: false,
        is_little_endian,
        version: 1,
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
        section_name_table_index,
    })
}
