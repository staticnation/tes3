//! A leveled list's `INDX` count is a reserve hint read from the file: a damaged count must
//! not ask the allocator for more than the record could hold.

use bytes_io::Reader;
use esp::TES3Object;

fn sub(tag: [u8; 4], data: &[u8]) -> Vec<u8> {
    [&tag[..], &u32::try_from(data.len()).unwrap().to_le_bytes(), data].concat()
}

fn record(tag: [u8; 4], body: &[u8]) -> Vec<u8> {
    [&tag[..], &u32::try_from(body.len()).unwrap().to_le_bytes(), &[0; 8], body].concat()
}

#[test]
fn a_huge_leveled_list_count_reads_the_entries_there_are() {
    for (tag, entry) in [(*b"LEVC", *b"CNAM"), (*b"LEVI", *b"INAM")] {
        let body = [
            sub(*b"NAME", b"list\0"),
            sub(*b"INDX", &u32::MAX.to_le_bytes()),
            sub(entry, b"thing\0"),
            sub(*b"INTV", &1u16.to_le_bytes()),
        ]
        .concat();
        let object = Reader::new(&record(tag, &body)).load::<TES3Object>().unwrap();
        let count = match object {
            TES3Object::LeveledCreature(list) => list.creatures.len(),
            TES3Object::LeveledItem(list) => list.items.len(),
            _ => panic!("not a leveled list"),
        };
        assert_eq!(count, 1);
    }
}
