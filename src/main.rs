mod header;

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
