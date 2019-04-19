use std::fs::File;
use std::io::Read;

struct PartitionTableStruct
{
	bootIndicator : u8,         /* Boot flags                 */
	startingHead : u8,          /* Starting head              */
    startingSector : u8,        /* Starting sector            */
	startingCylinder : u8,      /* Starting cylinder          */

	systemID : u8,              /* 2:Xenix, 5:X-DOS, 7:HPFS,  */
						        /* 99:UNIX, 100-104:NetWare,  */
							    /* 196-198:DRDOS, 0,1,3,4,6,  */
						        /* 8,9:DOS,219:Concurrent DOS,*/
						        /* 80-84:Ontrack, B,C:FAT32   */
	endingHead : u8,            /* Partition ending head      */
	endingSector : u8,          /* Partition ending sector    */
	endingCylinder : u8,        /* Partition ending cylinder  */

    relativeSector : u32,       /* Relative parition addr     */
	sectorCnt : u32             /* Num sectors in partition   */
}

struct MbrStruct
{
	bootCode : [u8;446],
	partition : [PartitionTableStruct;4],
	signature: [u8;2]
}

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