use super::*;
use std::mem;
use std::str;
use ToStruct;

#[test]
fn io_read_can_read_from_drive() {
    let first_sector = io_read(r#"\\.\PHYSICALDRIVE1"#, 0, 1);
    assert_eq!(first_sector.len(), 512);
}

#[test]
fn check_if_mbr_has_valid_signature() {
    let first_sector = io_read(r#"\\.\PHYSICALDRIVE1"#, 0, 1);
    let signature = &first_sector[510..512];
    let expected = [0x55, 0xAA];
    for character in signature {
        print!("{:x?} ", character);
    }

    assert_eq!(signature[0], expected[0]);
    assert_eq!(signature[1], expected[1]);
}

#[test]

fn is_typecasting_working() {
    struct SomeStruct {
        i: i32,
    }
    let s: SomeStruct = vec![5, 0, 0, 0].to_struct();

    assert_eq!(5, s.i);
}

#[test]
fn check_if_mbr_can_be_cast() {
    let expected = [0x55, 0xAA];

    let mbr: MbrStruct = io_read(r#"\\.\PHYSICALDRIVE1"#, 0, 1).to_struct();

    assert_eq!(mbr.signature[0], expected[0]);
    assert_eq!(mbr.signature[1], expected[1]);
}

#[test]
fn check_if_mbr_has_valid_partition_position() {
    let mbr: MbrStruct = io_read(r#"\\.\PHYSICALDRIVE1"#, 0, 1).to_struct();

    assert_eq!(mem::size_of::<PartitionTableStruct>(), 16 as usize);
    unsafe {
        assert_eq!(mbr.partition[0].sectorCnt, 4_188_160);
        assert_eq!(mbr.partition[0].relativeSector, 128);
    }
}

#[test]
fn check_if_bpb_has_valid_data() {
    const PARTITION_START: u64 = 128;
    let bpb: BpbStruct = io_read(r#"\\.\PHYSICALDRIVE1"#, PARTITION_START, 1).to_struct();

    unsafe {
        assert_eq!(bpb.reserved, 8234);
        assert_eq!(bpb.fatCnt, 2);
        assert_eq!(bpb.fat32Secs, 4075);
        assert_eq!(bpb.rootCluster, 2);
        assert_eq!(bpb.secSize, 512);
        assert_eq!(bpb.secsPerCluster, 8);
    }

    let fat_data_start =
        PARTITION_START + bpb.reserved as u64 + (bpb.fatCnt as u64 * bpb.fat32Secs as u64);

    assert_eq!(fat_data_start, 16512);
}

#[test]
fn check_if_fat_dir_struct_is_valid() {
    const FAT_DIR_START: u64 = 16512;

    let bpb: Fat32DirStruct = io_read(r#"\\.\PHYSICALDRIVE1"#, FAT_DIR_START, 1).to_struct();

    let volume_label = str::from_utf8(&bpb.name).unwrap();
    assert_eq!(volume_label, r#"RUST-HACKA "#);
}

#[test]
fn should_iterate_over_root_dir_entries() {
    const FAT_DIR_START: u64 = 16512;
    const ENTRY_SIZE: usize = std::mem::size_of::<Fat32DirStruct>();

    let memo = io_read(r#"\\.\PHYSICALDRIVE1"#, FAT_DIR_START, 1);

    let is_zeroed = |entry: &[u8]| -> bool {
        let non_zero_bytes: Vec<_> = entry
            .iter()
            .filter_map(|byte| if *byte == 0u8 { None } else { Some(()) })
            .collect();

        return non_zero_bytes.len() == 0;
    };

    let mut index: usize = 0;
    loop {
        let entry_start = index * ENTRY_SIZE;
        let entry_end = entry_start + ENTRY_SIZE;

        let entry_memory_slice = &memo[entry_start..entry_end].to_vec();

        if is_zeroed(&entry_memory_slice) {
            break;
        } else {
            index += 1;

            let fat32_entry: Fat32DirStruct = entry_memory_slice.to_struct();

            unsafe {
                println!(
                    "Name: {:#?} Start cluster: {:#?} File size: {:#?} Create date: {}",
                    str::from_utf8(&fat32_entry.name).unwrap(),
                    &fat32_entry.start_cluster,
                    fat32_entry.file_size,
                    fat32_entry.create_date
                );
            }
        }
    }

    assert_eq!(true, true);
}
