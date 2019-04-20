use std::io::{Read, SeekFrom, ErrorKind, prelude::*};
use std::fs::File;

mod fat32structs;
pub use fat32structs::*;

pub fn io_read(path: &str, start_sector: u64, count: u64) -> Vec<u8> {
    const SECTOR_SIZE: usize = 512;
    let mut handle = File::open(path).unwrap();

    let mut buffer = vec![0u8; SECTOR_SIZE * count as usize];
    let _offset = handle.seek(SeekFrom::Start(SECTOR_SIZE as u64 * start_sector));

    let res = handle.read_exact(&mut buffer);
    match res {
        Ok(_) => println!("Read {} into buffer", &buffer.len()),
        Err(ref err) if err.kind() == ErrorKind::UnexpectedEof 
                                        || err.kind() == ErrorKind::Interrupted
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
