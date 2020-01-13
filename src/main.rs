mod header;
mod program_header;

use std::fmt;
use std::fs;

pub use header::{Header, Class, get_elf_class};
pub use program_header::ProgramHeader;

fn main() {
    let binary = read_binary("hello.out");
    match get_elf_class(&binary) {
        Some(Class::ELF64) => {
            match ELF::<u64>::new(&binary) {
                Some(elf) => println!("{}", elf),
                None => println!("{}", "Something Error!"),
            }
        }
        _ => (),
    }
}

struct ELF<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy
{
    header: Header<T>,
    program_headers: Vec<ProgramHeader<T>>
}

impl ELF<u32>
{
    pub fn new(binary: &Vec<u8>) -> Option<ELF<u32>> {
        let header = Header::<u32>::new(binary)?;
        let program_headers = ProgramHeader::<u32>::new(binary, &header)?;

        Some(ELF {
            header,
            program_headers,
        })
    }
}

impl ELF<u64>
{
    pub fn new(binary: &Vec<u8>) -> Option<ELF<u64>> {
        let header = Header::<u64>::new(binary)?;
        let program_headers = ProgramHeader::<u64>::new(binary, &header)?;

        Some(ELF {
            header,
            program_headers,
        })
    }
}

impl<T> fmt::Display for ELF<T>
    where T: fmt::Display + fmt::Debug + fmt::LowerHex + Copy
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

        write!(f, "{}\n\n{}", self.header, program_header_string)
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
