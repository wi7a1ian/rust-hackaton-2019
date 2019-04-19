use super::*;
use std::mem;

#[test]
fn io_read_can_read_from_drive() {
    let first_sector = io_read(r#"\\.\PHYSICALDRIVE1"#, 0, 1);
    assert_eq!(first_sector.len(),512);
}

#[test]
fn check_if_mbr_has_valid_signature() {
    let mut first_sector = io_read(r#"\\.\PHYSICALDRIVE1"#, 0, 1);
    let mut signature = &first_sector[510..512];
    let expected = [0x55, 0xAA];
    for character in signature{
        print!("{:x?} ", character);
    }

    assert_eq!(signature[0],expected[0]);
    assert_eq!(signature[1],expected[1]);
}

#[test]
fn typecasting_is_working() {
    struct SomeStruct { i : i32, }
    let mut one_elem_vec : Vec<u8> = vec![5,0,0,0];
    let s : SomeStruct = to_struct(&one_elem_vec);

    assert_eq!(5, s.i);
}

#[test]
fn check_if_mbr_can_be_casted() {
    let mut first_sector = io_read(r#"\\.\PHYSICALDRIVE1"#, 0, 1);
    let expected = [0x55, 0xAA];

    let mbr : MbrStruct = to_struct(&first_sector);

    assert_eq!(mbr.signature[0],expected[0]);
    assert_eq!(mbr.signature[1],expected[1]);
}

#[test]
fn check_if_mbr_has_valid_partition_position() {
    let mut first_sector = io_read(r#"\\.\PHYSICALDRIVE1"#, 0, 1);
    
    let mbr : MbrStruct = to_struct(&first_sector);

    assert_eq!(mem::size_of::<PartitionTableStruct>(), 16 as usize);
    assert_eq!(mbr.partition[0].sectorCnt, 4_188_160);
    assert_eq!(mbr.partition[0].relativeSector, 128);
}

#[test]
fn check_if_bpb_has_valid_data() {
    const partition_start : u64 = 128;
    let mut memo = io_read(r#"\\.\PHYSICALDRIVE1"#, partition_start, 1);
    let sig = [0x55, 0xAA];

    let bpb : BpbStruct = to_struct(&memo);

    assert_eq!(bpb.reserved, 8234);
    assert_eq!(bpb.fatCnt, 2);
    assert_eq!(bpb.fat32Secs, 4075);
    assert_eq!(bpb.rootCluster, 2);
    assert_eq!(bpb.secSize, 512);
    assert_eq!(bpb.secsPerCluster, 8);

    let fat_data_start = partition_start + bpb.reserved as u64 + (bpb.fatCnt as u64 * bpb.fat32Secs as u64);
    assert_eq!(fat_data_start, 16512);
}