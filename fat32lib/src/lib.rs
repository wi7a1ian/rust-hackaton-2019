use std::fs::File;
use std::io::{prelude::*, ErrorKind, Read, SeekFrom};

mod fat32structs;
pub use fat32structs::*;

pub fn io_read(path: &str, start_sector: u64, count: u64) -> Vec<u8> {
    const SECTOR_SIZE: usize = 512;
    let mut handle = File::open(path).unwrap();

    let mut buffer = vec![0u8; SECTOR_SIZE * count as usize];
    let _offset = handle.seek(SeekFrom::Start(SECTOR_SIZE as u64 * start_sector));

    let read_result = handle.read_exact(&mut buffer);
    match read_result {
        Ok(_) => println!("Read {} into buffer", &buffer.len()),
        Err(ref err)
            if err.kind() == ErrorKind::UnexpectedEof || err.kind() == ErrorKind::Interrupted =>
        {
            println!("An error has occurred {}", err)
        }
        Err(_) => println!("An unknown error has occurred"),
    }

    buffer
}

pub trait ToStruct {
    fn to_struct<T>(&self) -> T;
}

impl ToStruct for [u8] {
    fn to_struct<T>(&self) -> T {
        let s: T = unsafe { std::ptr::read(self.as_ptr() as *const _) };
        s
    }
}

#[cfg(test)]
mod tests;
