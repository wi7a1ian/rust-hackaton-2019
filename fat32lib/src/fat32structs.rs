#[repr(C, packed)]
pub struct PartitionTableStruct {
	pub boot_indicator: u8,    /* Boot flags                 */
	pub starting_head: u8,     /* Starting head              */
	pub starting_sector: u8,   /* Starting sector            */
	pub starting_cylinder: u8, /* Starting cylinder          */

	pub system_id: u8, /* 2:Xenix, 5:X-DOS, 7:HPFS,  */
	/* 99:UNIX, 100-104:NetWare,  */
	/* 196-198:DRDOS, 0,1,3,4,6,  */
	/* 8,9:DOS,219:Concurrent DOS,*/
	/* 80-84:Ontrack, B,C:FAT32   */
	pub ending_head: u8,     /* Partition ending head      */
	pub ending_sector: u8,   /* Partition ending sector    */
	pub ending_cylinder: u8, /* Partition ending cylinder  */

	pub relative_sector: u32, /* Relative parition addr     */
	pub sector_cnt: u32,      /* Num sectors in partition   */
}

#[repr(C, packed)]
pub struct MbrStruct {
	pub boot_code: [u8; 446],
	pub partition: [PartitionTableStruct; 4],
	pub signature: [u8; 2],
}

const BPB_JUMPCODE_SIZE: usize = 3;
const BPB_NAME_SIZE: usize = 8;
const BPB_LABEL_SIZE: usize = 11;
const BPB_SYSTYPE_SIZE: usize = 8;
//const BPB_BOOT_SIGNATURE: usize = 0x29;

#[repr(C, packed)]
pub struct BpbStruct {
	pub jump_code: [u8; BPB_JUMPCODE_SIZE], /* 2 or 3 byte jump instruct  */
	pub bpb_name: [u8; BPB_NAME_SIZE],      /* 8 byte name field          */
	pub sec_size: u16,                      /* Bytes per sector           */
	pub secs_per_cluster: u8,                /* Sectors per cluster        */
	pub reserved: u16,                     /* Number of reserved sectors */
	pub fat_cnt: u8,                        /* Number of fats in this part*/
	pub dir_cnt: u16,                       /* Number of ROOT entries     */
	pub total_secs: u16,                    /* Total number of sectors    */
	pub media: u8,                         /* Media descriptor byte      */
	pub fat_secs: u16,                      /* FAT sector count (12 or 16)*/
	pub spt: u16,                          /* Sectors/track in this part */
	pub heads: u16,                        /* Heads in this partition    */
	pub hidden_secs: u32,                   /* Number of hidden sectors   */
	pub big_total_secs: u32,                 /* Number of total sectors    */

	// for fat32 only:
	pub fat32_secs: u32, /* FAT sector count (32 only) */
	pub ext_flags: u16,  /* Extended flags             */
	/* Bits 0-3: active FAT 0 rel */
	/* Bits 4-6: reserved         */
	/* Bit  7:   0-FAT's mirrored */
	/*           1-one FAT active */
	/* Bits 8-15: reserved        */
	pub fs_version: u16,      /* version number             */
	pub root_cluster: u32,    /* 1st cluster of root dir    */
	pub fs_info: u16,         /* location of FS info struct */
	pub backup_boot: u16,     /* location of backup boot    */
	pub reserved2: [u32; 3], /* reserved area              */
	pub drive_num: u8,        /* Device address             */
	pub reserved3: u8,
	pub boot_signature: u8, /* Ext-boot signature 0x29    */
	pub bpb_checksum: u32,  /* Checksum of the BPB        */
	/* Volume serial number       */
	pub volume_label: [u8; BPB_LABEL_SIZE], /* Volume label               */
	pub file_sys_type: [u8; BPB_SYSTYPE_SIZE], /* File system (FAT32)      */
}

#[repr(C, packed)]
pub struct Fat32DirStruct {
	pub name: [u8; 11], /* File or directory name     */
	pub attribute: u8,  /* File attribute             */

	pub reserved3: u8,         /* Reserved by Windows NT     */
	pub create_mil: u8,        /* Millisecond stamp at create*/
	pub create_time: u16,      /* Creation time              */
	pub create_date: u16,      /* Creation date              */
	pub access_date: u16,      /* Last accessed date         */
	pub start_cluster_hi: u16, /* High word of starting clust*/

	pub time: u16, /* DOS time                   */
	pub date: u16, /* DOS date                   */

	pub start_cluster: u16, /* Starting cluster number    */
	pub file_size: u32,     /* Size of the file           */
}
