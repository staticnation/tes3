//! A subrecord a record type does not know (an NPC_ with a FILL, as tools and test fixtures
//! write) fails the record by default, and is stepped over with `Reader::lenient`.

use bytes_io::Reader;
use esp::{Npc, TES3Object};

fn sub(tag: [u8; 4], data: &[u8]) -> Vec<u8> {
    [&tag[..], &u32::try_from(data.len()).unwrap().to_le_bytes(), data].concat()
}

/// `NPC_` with an id, an unknown 5,000-byte `FILL` between its subrecords, and a name.
fn npc_with_fill() -> Vec<u8> {
    let body = [
        sub(*b"NAME", b"noise\0"),
        sub(*b"FILL", &[0; 5000]),
        sub(*b"FNAM", b"Noise\0"),
    ]
    .concat();
    [
        &b"NPC_"[..],
        &u32::try_from(body.len()).unwrap().to_le_bytes(),
        &[0; 8],
        &body,
    ]
    .concat()
}

#[test]
fn an_unknown_subrecord_fails_strict_and_is_skipped_lenient() {
    let bytes = npc_with_fill();

    let err = Reader::new(&bytes).load::<TES3Object>().unwrap_err();
    assert_eq!(err.to_string(), "Unexpected Tag: NPC_::FILL");

    let mut lenient = Reader::new(&bytes);
    lenient.lenient = true;
    let TES3Object::Npc(npc) = lenient.load::<TES3Object>().unwrap() else {
        panic!("not an NPC")
    };
    let npc: Npc = npc;
    assert_eq!(
        (npc.id.as_str(), npc.name.as_str()),
        ("noise", "Noise"),
        "the subrecords after the unknown one are read"
    );
}
