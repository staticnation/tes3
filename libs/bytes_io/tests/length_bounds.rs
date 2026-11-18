//! A length read from corrupt data (a mesh whose triangle count is garbage) is an error, never a
//! panic inside `bytemuck::zeroed_vec` or an abort from `vec!`.

use bytes_io::Reader;

#[test]
fn a_length_past_the_end_is_an_error_not_a_panic() {
    let data = [1u8, 2, 3, 4];
    // u32::MAX floats from four bytes: `zeroed_vec` panicked trying to allocate them.
    let r = std::panic::catch_unwind(|| Reader::new(&data).load_vec::<f32>(u32::MAX).map(|_| ()));
    let err = r.expect("no panic").unwrap_err();
    assert!(err.to_string().starts_with("Length out of bounds"), "{err}");
    // and bytes (`vec![0; len]` aborted the process when the allocation failed).
    assert!(Reader::new(&data).load_bytes(usize::MAX / 2).is_err());
    // A length that fits still reads.
    assert_eq!(Reader::new(&data).load_vec::<u8>(4u32).unwrap(), data);
}
