use std::fs::File;
use std::io::Read;

fn io_read(path: &str, start_sector: u64, count: u64) -> Vec<u8>
{
    let mut handle = File::open(path).unwrap();

    let mut buffer = vec![0u8;512*count as usize];
    //or let mut buffer : Vec<u8> = Vec::with_capacity(512 * count as usize);
    handle.read_exact(&mut buffer);

    buffer
}


#[cfg(test)]
mod tests;