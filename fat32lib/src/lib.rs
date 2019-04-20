use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::io::SeekFrom;

mod fat32structs;
pub use fat32structs::*;

fn io_read(path: &str, start_sector: u64, count: u64) -> Vec<u8> {
    const sector_size: usize = 512;
    let mut handle = File::open(path).unwrap();

    let mut buffer = vec![0u8; sector_size * count as usize];
    handle.seek(SeekFrom::Start(sector_size as u64 * start_sector));
    handle.read_exact(&mut buffer);

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
