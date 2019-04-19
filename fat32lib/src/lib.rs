use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;

mod fat32structs;
pub use fat32structs::*;

fn io_read(path: &str, start_sector: u64, count: u64) -> Vec<u8>
{
    const sector_size : usize = 512;
    let mut handle = File::open(path).unwrap();

    let mut buffer = vec![0u8;sector_size * count as usize];
    handle.seek(SeekFrom::Start(sector_size as u64 * start_sector));
    handle.read_exact(&mut buffer);

    buffer
}


fn to_struct<T>(v : &Vec<u8>) -> T
{
    let s: T = unsafe { std::ptr::read(v.as_ptr() as *const _) };
    s
}

// impl std::fmt::Display for Point {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

#[cfg(test)]
mod tests;