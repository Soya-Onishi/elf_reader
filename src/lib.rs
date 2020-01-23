extern crate num;

mod header;
mod program_header;
mod section_header;

use std::fmt;
use std::fs;
use num::cast;

pub use header::{Header, Class};
pub use program_header::ProgramHeader;
pub use section_header::SectionHeader;

struct ELF<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy
{
    header: Header<T>,
    program_headers: Vec<ProgramHeader<T>>,
    section_headers: Vec<SectionHeader<T>>,
    data: Vec<u8>,
}

impl ELF<u32>
{
    pub fn new(binary: Vec<u8>) -> Option<ELF<u32>> {
        let header = Header::<u32>::new(&binary)?;
        let program_headers = ProgramHeader::<u32>::new(&binary, &header)?;
        let section_headers = SectionHeader::<u32>::new(&binary, &header)?;

        Some(ELF {
            header,
            program_headers,
            section_headers,
            data: binary,
        })
    }
}

impl ELF<u64>
{
    pub fn new(binary: Vec<u8>) -> Option<ELF<u64>> {
        let header = Header::<u64>::new(&binary)?;
        let program_headers = ProgramHeader::<u64>::new(&binary, &header)?;
        let section_headers = SectionHeader::<u64>::new(&binary, &header)?;

        Some(ELF {
            header,
            program_headers,
            section_headers,
            data: binary,
        })
    }
}

impl<T> fmt::Display for ELF<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy + cast::AsPrimitive<usize>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let program_header_string =
            self.program_headers
                .iter()
                .zip(0..)
                .map(|(ph, index)| {
                    let ph_string = ph.to_string();
                    let ph_string = ph_string
                        .split("\n")
                        .collect::<Vec<&str>>()
                        .join("\n    ");

                    format!("[{}]\n    {}", index, ph_string)
                })
                .collect::<Vec<String>>()
                .join("\n");

        let section_header_string =
            self.section_headers
                .iter()
                .map(|sh| { format!("{}", sh)} )
                .collect::<Vec<String>>()
                .join("\n");

        write!(f, "{}\n\n{}\n\n{}", self.header, program_header_string, section_header_string)
    }
}

fn read_binary(filename: &str) -> Vec<u8> {
    match fs::read(filename) {
        Ok(bin) => bin,
        Err(e) => panic!(e),
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
