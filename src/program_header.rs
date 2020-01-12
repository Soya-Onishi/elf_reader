use std::fmt;

use super::header::Header;
use super::{make_u16, make_u32, make_u64};

pub struct ProgramHeader<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex
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

impl ProgramHeader<u32> {
    fn new(binary: &Vec<u8>, header: &Header<u32>) -> Option<Vec<ProgramHeader<u32>>> {
        (0..header.ph_num() as u32).map(|index| {
            let offset = index + header.ph_size() as u32;
            let entry = header.ph_offset() + offset;
            Self::construct(binary, header,entry as usize)
        }).collect::<Option<Vec<_>>>()
    }

    fn construct(binary: &Vec<u8>, header: &Header<u32>, entry: usize) -> Option<ProgramHeader<u32>> {
        let program_type =
            ProgramType::new(
                make_u32(&binary[entry + 0x0..entry + 0x4], header.is_little())
            )?;
        let offset = make_u32(&binary[entry + 0x4..entry + 0x8], header.is_little());
        let vaddr = make_u32(&binary[entry + 0x8..entry + 0xC], header.is_little());
        let paddr = make_u32(&binary[entry + 0xC..entry + 0x10], header.is_little());
        let file_size = make_u32(&binary[entry + 0x10..entry + 0x14], header.is_little());
        let memory_size = make_u32(&binary[entry + 0x14..entry + 0x18], header.is_little());
        let flags = make_u32(&binary[entry + 0x18..entry + 0x1C], header.is_little());
        let align = make_u32(&binary[entry + 0x1C..entry + 0x20], header.is_little());

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