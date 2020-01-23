extern crate num;

use std::fmt;
use std::ops;

use num::cast;

use super::Header;
use super::{make_u32, make_u64};

pub struct SectionHeader<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy
{
    name: String,
    inner: InnerSectionHeader<T>,
}

impl<T> SectionHeader<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy + cast::AsPrimitive<usize>
{
    fn construct(binary: &Vec<u8>, header: &Header<T>, inner_headers: Vec<InnerSectionHeader<T>>) -> Option<Vec<SectionHeader<T>>> {
        let shstrtab = match inner_headers.last() {
            Some(section) => section,
            None => return Some(Vec::new()),
        };

        let section_entry = shstrtab.offset.as_();

        inner_headers.iter().map(|inner| {
            let name_entry = inner.name_offset + section_entry;
            let end_of_string = binary[name_entry..].iter().position(|&bin| { bin == 0 })?;
            let name = match std::str::from_utf8(&binary[name_entry..name_entry + end_of_string]) {
                Ok(s) => String::from(s),
                Err(_) => return None,
            };

            Some(SectionHeader{
                name,
                inner: inner.clone(),
            })
        }).collect::<Option<Vec<_>>>()
    }

    pub fn name(&self) -> String { self.name.clone() }
    pub fn section_type(&self) -> SectionType { self.inner.section_type }
    pub fn flags(&self) -> T { self.inner.flags }
    pub fn target_addr(&self) -> T { self.inner.addr }
    pub fn file_offset(&self) -> T { self.inner.offset }
    pub fn size(&self) -> T { self.inner.size }
    pub fn link(&self) -> u32 { self.inner.link }
    pub fn info(&self) -> u32 { self.inner.info }
    pub fn addr_align(&self) -> T { self.inner.addr_align }
    pub fn entry_size(&self) -> T { self.inner.entry_size }
}

impl SectionHeader<u32> {
    pub fn new(binary: &Vec<u8>, header: &Header<u32>) -> Option<Vec<SectionHeader<u32>>> {
        let ih = InnerSectionHeader::<u32>::new(binary, header)?;
        Self::construct(binary, header, ih)
    }
}

impl SectionHeader<u64> {
    pub fn new(binary: &Vec<u8>, header: &Header<u64>) -> Option<Vec<SectionHeader<u64>>> {
        Self::construct(binary, header, InnerSectionHeader::<u64>::new(binary, header)?)
    }
}

impl<T> fmt::Display for SectionHeader<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy + cast::AsPrimitive<usize>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.to_string();
        let inner = inner
            .split("\n")
            .map(|line| { format!("    {}", line)})
            .collect::<Vec<String>>()
            .connect("\n");

        write!(f, "{}\n{}", self.name, inner)
    }
}

#[derive(Clone)]
struct InnerSectionHeader<T> {
    name_offset: usize,
    pub section_type: SectionType,
    pub flags: T,
    pub addr: T,
    pub offset: T,
    pub size: T,
    pub link: u32,
    pub info: u32,
    pub addr_align: T,
    pub entry_size: T,

}

impl<T> InnerSectionHeader<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy
{
    fn construct(binary: &Vec<u8>, header: &Header<T>, entry: &[ops::Range<usize>], make_unsigned: impl Fn(&[u8], bool) -> T) -> Option<InnerSectionHeader<T>> {
        let name_offset = make_u32(&binary[entry[0].clone()], header.is_little()) as usize;
        let section_type = SectionType::new(make_u32(&binary[entry[1].clone()], header.is_little()))?;
        let flags = make_unsigned(&binary[entry[2].clone()], header.is_little());
        let addr = make_unsigned(&binary[entry[3].clone()], header.is_little());
        let offset = make_unsigned(&binary[entry[4].clone()], header.is_little());
        let size = make_unsigned(&binary[entry[5].clone()], header.is_little());
        let link = make_u32(&binary[entry[6].clone()], header.is_little());
        let info = make_u32(&binary[entry[7].clone()], header.is_little());
        let addr_align = make_unsigned(&binary[entry[8].clone()], header.is_little());
        let entry_size = make_unsigned(&binary[entry[9].clone()], header.is_little());

        Some(InnerSectionHeader {
            name_offset,
            section_type,
            flags,
            addr,
            offset,
            size,
            link,
            info,
            addr_align,
            entry_size,
        })
    }
}

impl InnerSectionHeader<u32> {
    fn make_entry_point(header: &Header<u32>, index: usize) -> usize {
        let sh_offset = header.sh_offset() as usize;
        let sh_size = header.sh_size() as usize;

        sh_offset + index * sh_size
    }

    pub fn new(binary: &Vec<u8>, header: &Header<u32>) -> Option<Vec<InnerSectionHeader<u32>>> {
        let sh_num = header.sh_num() as usize;

        (0..sh_num).map(|index| {
            let ep = Self::make_entry_point(header, index);
            let entry = [
                ep + 0x00..ep + 0x04,
                ep + 0x04..ep + 0x08,
                ep + 0x08..ep + 0x0C,
                ep + 0x0C..ep + 0x10,
                ep + 0x10..ep + 0x14,
                ep + 0x14..ep + 0x18,
                ep + 0x18..ep + 0x1C,
                ep + 0x1C..ep + 0x20,
                ep + 0x20..ep + 0x24,
                ep + 0x24..ep + 0x28,
            ];

            Self::construct(binary, header, &entry, make_u32)
        }).collect::<Option<Vec<_>>>()
    }
}

impl InnerSectionHeader<u64> {
    fn make_entry_point(header: &Header<u64>, index: usize) -> usize {
        let sh_offset = header.sh_offset() as usize;
        let sh_size = header.sh_size() as usize;

        sh_offset + index * sh_size
    }

    pub fn new(binary: &Vec<u8>, header: &Header<u64>) -> Option<Vec<InnerSectionHeader<u64>>> {
        let sh_num = header.sh_num() as usize;

        (0..sh_num).map(|index| {
            let ep = Self::make_entry_point(header, index);
            let entry = [
                ep + 0x00..ep + 0x04,
                ep + 0x04..ep + 0x08,
                ep + 0x08..ep + 0x10,
                ep + 0x10..ep + 0x18,
                ep + 0x18..ep + 0x20,
                ep + 0x20..ep + 0x28,
                ep + 0x28..ep + 0x2C,
                ep + 0x2C..ep + 0x30,
                ep + 0x30..ep + 0x38,
                ep + 0x38..ep + 0x40,
            ];

            Self::construct(binary, header, &entry, make_u64)
        }).collect::<Option<Vec<_>>>()
    }
}

impl<T> fmt::Display for InnerSectionHeader<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy + cast::AsPrimitive<usize>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flag_pairs = [
            (0x0000_0001, 'W'),
            (0x0000_0002, 'A'),
            (0x0000_0004, 'X'),
            (0x0000_0010, 'M'),
            (0x0000_0020, 'S'),
            (0x0000_0040, 'I'),
            (0x0000_0080, 'L'),
            (0x0000_0100, 'O'),
            (0x0000_0200, 'G'),
            (0x0000_0400, 'T'),
            (0x0FF0_0000, 'o'),
            (0xF000_0000, 'p'),
            (0x4000_0000, 'x'),
            (0x8000_0000, 'x'),
        ];

        let flag = flag_pairs.iter().flat_map(|(mask, ch)| {
            if mask & self.flags.as_() > 0 { Some(ch) } else { None }
        }).collect::<String>();

        let section_type = format!("Type:   {:?}\n", self.section_type);
        let flag         = format!("Flags:  {}\n", flag);
        let addr         = format!("Addr:   0x{:016x}\n", self.addr);
        let offset       = format!("Offset: 0x{:016x}\n", self.offset);
        let size         = format!("Size:   0x{:016x}\n", self.size);
        let link         = format!("Link:   {}\n", self.link);
        let info         = format!("Info:   {}\n", self.info);
        let addr_align   = format!("Align:  0x{:x}\n", self.addr_align);

        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            section_type,
            flag,
            addr,
            offset,
            size,
            link,
            info,
            addr_align,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SectionType {
    Null,
    ProgBits,
    SymTab,
    StrTab,
    Rela,
    Hash,
    Dynamic,
    Note,
    NoBits,
    Rel,
    ShLib,
    DynSym,
    InitArray,
    FiniArray,
    PreinitArray,
    Group,
    SymTabSHNDX,
    Num,
    Loos(u32),
    LoProc(u32),
    LoUser(u32),
}

impl SectionType {
    pub fn new(value: u32) -> Option<SectionType> {
        let range_check = |value: u32, base: u32| { value >= base && value < base + 0x1000_0000 };

        let sec_type = match value {
            0x0000_0000 => SectionType::Null,
            0x0000_0001 => SectionType::ProgBits,
            0x0000_0002 => SectionType::SymTab,
            0x0000_0003 => SectionType::StrTab,
            0x0000_0004 => SectionType::Rela,
            0x0000_0005 => SectionType::Hash,
            0x0000_0006 => SectionType::Dynamic,
            0x0000_0007 => SectionType::Note,
            0x0000_0008 => SectionType::NoBits,
            0x0000_0009 => SectionType::Rel,
            0x0000_000A => SectionType::ShLib,
            0x0000_000B => SectionType::DynSym,
            0x0000_000E => SectionType::InitArray,
            0x0000_000F => SectionType::FiniArray,
            0x0000_0010 => SectionType::PreinitArray,
            0x0000_0011 => SectionType::Group,
            0x0000_0012 => SectionType::SymTabSHNDX,
            0x0000_0013 => SectionType::Num,
            value if range_check(value, 0x6000_0000) => SectionType::Loos(value - 0x6000_0000),
            value if range_check(value, 0x7000_0000) => SectionType::LoProc(value - 0x7000_0000),
            value if range_check(value, 0x8000_0000) => SectionType::LoUser(value - 0x8000_0000),
            _           => return None,
        };

        Some(sec_type)
    }
}
