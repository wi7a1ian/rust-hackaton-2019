use super::*;

#[test]
fn io_read_can_read_from_drive() {
    let first_sector = io_read(r#"\\.\PHYSICALDRIVE1"#, 0, 1);
    assert_eq!(first_sector.len(),512);
}

#[test]
fn if_mbr() {
    let mut first_sector = io_read(r#"\\.\PHYSICALDRIVE1"#, 0, 1);
    let mut signature = &first_sector[510..512];
    let expected = [0x55, 0xAA];
    for character in signature{
        print!("{:x?} ", character);
    }

    assert_eq!(signature[0],expected[0]);
    assert_eq!(signature[1],expected[1]);
}