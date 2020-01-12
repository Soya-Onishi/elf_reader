mod header;
mod program_header;

use std::fs;

pub use header::*;

fn main() {
    let binary = read_binary("hello.out");
    match Header::<u32>::get_elf_class(&binary) {
        Some(Class::ELF64) => {
            if let Some(header) = Header::<u64>::new(&binary) {
                println!("{}", header);
            }
        }
        _ => (),
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
