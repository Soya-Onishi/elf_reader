use std::fmt;

use super::header::Header;
use super::{make_u32, make_u64};
use std::ops::Range;

pub struct ProgramHeader<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy
{
    program_type: ProgramType,
    offset: T,
    vaddr: T,
    paddr: T,
    file_size: T,
    memory_size: T,
    flags: u32,
    align: T,
}

impl<T> ProgramHeader<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy
{
    fn construct(
        binary: &Vec<u8>,
        header: &Header<T>,
        flags: u32,
        entry: &[Range<usize>],
        make_unsigned: impl Fn(&[u8], bool) -> T,
    ) -> Option<ProgramHeader<T>> {
        let program_type =
            ProgramType::new(
                make_u32(&binary[entry[0].clone()], header.is_little())
            )?;

        let offset = make_unsigned(&binary[entry[1].clone()], header.is_little());
        let vaddr = make_unsigned(&binary[entry[2].clone()], header.is_little());
        let paddr = make_unsigned(&binary[entry[3].clone()], header.is_little());
        let file_size = make_unsigned(&binary[entry[4].clone()], header.is_little());
        let memory_size = make_unsigned(&binary[entry[5].clone()], header.is_little());
        let align = make_unsigned(&binary[entry[6].clone()], header.is_little());

        Some(ProgramHeader {
            program_type,
            offset,
            vaddr,
            paddr,
            file_size,
            memory_size,
            flags,
            align,
        })
    }
}

impl ProgramHeader<u32> {
    pub fn new(binary: &Vec<u8>, header: &Header<u32>) -> Option<Vec<ProgramHeader<u32>>> {
        (0..header.ph_num() as usize).map(|index| {
            let offset = index * header.ph_size() as usize;
            let entry_point = header.ph_offset() as usize + offset;
            let entry = [
                (entry_point + 0x00..entry_point + 0x04),
                (entry_point + 0x04..entry_point + 0x08),
                (entry_point + 0x08..entry_point + 0x0C),
                (entry_point + 0x0C..entry_point + 0x10),
                (entry_point + 0x10..entry_point + 0x14),
                (entry_point + 0x14..entry_point + 0x18),
                (entry_point + 0x1C..entry_point + 0x20),
            ];

            let flags = make_u32(&binary[entry_point + 0x18..entry_point + 0x1C], header.is_little());

            Self::construct(binary, header, flags, &entry, make_u32)
        }).collect::<Option<Vec<_>>>()
    }
}

impl ProgramHeader<u64> {
    pub fn new(binary: &Vec<u8>, header: &Header<u64>) -> Option<Vec<ProgramHeader<u64>>> {
        (0..header.ph_num() as usize).map(|index| {
            let offset = index * header.ph_size() as usize;
            let entry_point = header.ph_offset() as usize + offset;
            let entry = [
                (entry_point + 0x00..entry_point + 0x04),
                (entry_point + 0x08..entry_point + 0x10),
                (entry_point + 0x10..entry_point + 0x18),
                (entry_point + 0x18..entry_point + 0x20),
                (entry_point + 0x20..entry_point + 0x28),
                (entry_point + 0x28..entry_point + 0x30),
                (entry_point + 0x30..entry_point + 0x38),
            ];

            let flags = make_u32(&binary[entry_point + 0x04..entry_point + 0x08], header.is_little());

            Self::construct(binary, header, flags, &entry, make_u64)
        }).collect()
    }
}

#[derive(Debug)]
pub enum ProgramType {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Tls,
    Loos,
    Hios,
    Loproc,
    Hiproc,
}

impl ProgramType {
    pub fn new(value: u32) -> Option<ProgramType> {
        let t = match value {
            0x0000_0000 => ProgramType::Null,
            0x0000_0001 => ProgramType::Load,
            0x0000_0002 => ProgramType::Dynamic,
            0x0000_0003 => ProgramType::Interp,
            0x0000_0004 => ProgramType::Note,
            0x0000_0005 => ProgramType::Shlib,
            0x0000_0006 => ProgramType::Phdr,
            0x0000_0007 => ProgramType::Tls,
            0x6000_0000 => ProgramType::Loos,
            0x6FFF_FFFF => ProgramType::Hios,
            0x7000_0000 => ProgramType::Loproc,
            0x7FFF_FFFF => ProgramType::Hiproc,
            _           => return None,
        };

        Some(t)
    }
}

impl<T> fmt::Display for ProgramHeader<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let program_type = format!("Type:          {:?}", self.program_type);
        let offset       = format!("Offset:        0x{:016x}", self.offset);
        let vaddr        = format!("Virtual Addr:  0x{:016x}", self.vaddr);
        let paddr        = format!("Physical Addr: 0x{:016x}", self.paddr);
        let file_size    = format!("File Size:     0x{:016x}", self.file_size);
        let memory_size  = format!("Memory Size:   0x{:016x}", self.memory_size);
        let align        = format!("Align:         0x{:x}", self.align);
        let r = if (self.flags & 0b100) > 0 {"R"} else {" "};
        let w = if (self.flags & 0b010) > 0 {"W"} else {" "};
        let x = if (self.flags & 0b001) > 0 {"E"} else {" "};
        let flags        = format!("Flags:         {}{}{}", r, w, x);

        let formatted_string = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            program_type,
            offset,
            vaddr,
            paddr,
            file_size,
            memory_size,
            flags,
            align,
        );

        write!(f, "{}", formatted_string)
    }
}