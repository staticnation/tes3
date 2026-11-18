//! Exporters write `NiGeometryData` with its UV sets present and `Has UV` 0. The UV sets
//! are as many as `Num UV Sets` says, as nif.xml reads them, so the flag must not drop them.

use nif::glam::{Vec2, Vec3};
use nif::*;

fn shape() -> NiTriShapeData {
    let mut data = NiTriShapeData::default();
    data.base.base.vertices = vec![Vec3::ZERO, Vec3::X, Vec3::Y];
    data.base.base.uv_sets = vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)];
    data.triangles = vec![[0, 1, 2]];
    data
}

#[test]
fn uv_sets_read_with_has_uv_off() -> std::io::Result<()> {
    let mut stream = NiStream::new();
    let link = stream.insert(shape());
    stream.roots.push(link.cast());
    let mut bytes = stream.save_bytes()?;

    // `Num UV Sets` (u16 = 1) then `Has UV` (u32 = 1): turn the flag off.
    let at = bytes.windows(6).position(|w| w == [1, 0, 1, 0, 0, 0]).expect("uv flag");
    bytes[at + 2] = 0;

    let read = NiStream::from_bytes(&bytes)?;
    let data = read.objects_of_type::<NiTriShapeData>().next().expect("shape");
    assert_eq!(data, &shape());
    Ok(())
}
