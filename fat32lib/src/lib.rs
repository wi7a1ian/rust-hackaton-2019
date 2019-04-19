use std::fs::File;
use std::io::Read;

mod fat32structs;
pub use fat32structs::*;

fn io_read(path: &str, start_sector: u64, count: u64) -> Vec<u8>
{
    let mut handle = File::open(path).unwrap();

    let mut buffer = vec![0u8;512*count as usize];
    //or let mut buffer : Vec<u8> = Vec::with_capacity(512 * count as usize);
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