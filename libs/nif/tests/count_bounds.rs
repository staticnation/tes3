//! The object and root counts are reserve hints read from the file: a damaged count must
//! fail the load with an error, not abort the process asking for memory.

use nif::NiStream;

fn file(num_objects: u32) -> Vec<u8> {
    [
        &NiStream::HEADER[..],
        &NiStream::VERSION.to_le_bytes(),
        &num_objects.to_le_bytes(),
    ]
    .concat()
}

#[test]
fn a_huge_object_count_is_an_error() {
    assert!(NiStream::from_bytes(&file(u32::MAX)).is_err());
}

#[test]
fn a_huge_root_count_is_an_error() {
    let bytes = [file(0), u32::MAX.to_le_bytes().to_vec()].concat();
    assert!(NiStream::from_bytes(&bytes).is_err());
}
