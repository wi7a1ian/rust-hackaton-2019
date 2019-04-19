
#[repr(C,packed)]
pub struct PartitionTableStruct
{
	pub bootIndicator : u8,         /* Boot flags                 */
	pub startingHead : u8,          /* Starting head              */
    pub startingSector : u8,        /* Starting sector            */
	pub startingCylinder : u8,      /* Starting cylinder          */

	pub systemID : u8,              /* 2:Xenix, 5:X-DOS, 7:HPFS,  */
						        /* 99:UNIX, 100-104:NetWare,  */
							    /* 196-198:DRDOS, 0,1,3,4,6,  */
						        /* 8,9:DOS,219:Concurrent DOS,*/
						        /* 80-84:Ontrack, B,C:FAT32   */
	pub endingHead : u8,            /* Partition ending head      */
	pub endingSector : u8,          /* Partition ending sector    */
	pub endingCylinder : u8,        /* Partition ending cylinder  */

    pub relativeSector : u32,       /* Relative parition addr     */
	pub sectorCnt : u32             /* Num sectors in partition   */
}

#[repr(C,packed)]
pub struct MbrStruct
{
	pub bootCode : [u8;446],
	pub partition : [PartitionTableStruct;4],
	pub signature: [u8;2]
}