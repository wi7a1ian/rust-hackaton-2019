
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


const BPB_JUMPCODE_SIZE : usize = 3;
const BPB_NAME_SIZE : usize = 8;
const BPB_LABEL_SIZE : usize = 11;
const BPB_SYSTYPE_SIZE : usize = 8;
const BPB_BOOT_SIGNATURE : usize = 0x29;

#[repr(C,packed)]
pub struct BpbStruct
{
	pub jumpCode : [u8; BPB_JUMPCODE_SIZE],      /* 2 or 3 byte jump instruct  */
	pub bpbName : [u8; BPB_NAME_SIZE],           /* 8 byte name field          */
	pub secSize : u16,                          /* Bytes per sector           */
	pub secsPerCluster : u8,                   /* Sectors per cluster        */
	pub reserved : u16,                         /* Number of reserved sectors */
	pub fatCnt : u8,                           /* Number of fats in this part*/
	pub dirCnt : u16,                           /* Number of ROOT entries     */
	pub totalSecs : u16,                        /* Total number of sectors    */
	pub media : u8,                            /* Media descriptor byte      */
	pub fatSecs : u16,                          /* FAT sector count (12 or 16)*/
	pub spt : u16,                              /* Sectors/track in this part */
	pub heads : u16,                            /* Heads in this partition    */
	pub hiddenSecs : u32,                       /* Number of hidden sectors   */
	pub bigTotalSecs : u32,                     /* Number of total sectors    */

	// for fat32 only:
	pub fat32Secs : u32,						  /* FAT sector count (32 only) */
	pub extFlags : u16,						  /* Extended flags             */
													/* Bits 0-3: active FAT 0 rel */
													/* Bits 4-6: reserved         */
													/* Bit  7:   0-FAT's mirrored */
													/*           1-one FAT active */
													/* Bits 8-15: reserved        */
	pub fsVersion : u16,						  /* version number             */
	pub rootCluster : u32,					  /* 1st cluster of root dir    */
	pub fsInfo : u16,							  /* location of FS info struct */
	pub backupBoot : u16,					  /* location of backup boot    */
	pub reserved2 : [u32; 3],					  /* reserved area              */
	pub driveNum : u8,                   /* Device address             */
	pub reserved3 : u8,
	pub bootSignature : u8,              /* Ext-boot signature 0x29    */
	pub bpbChecksum : u32,                /* Checksum of the BPB        */
												/* Volume serial number       */
	pub volumeLabel : [u8; BPB_LABEL_SIZE],		/* Volume label               */
	pub fileSysType: [u8; BPB_SYSTYPE_SIZE],		/* File system (FAT32)      */
}